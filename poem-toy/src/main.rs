use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, prelude::*, Error};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use poem::error::InternalServerError;
use poem::{
    error::NotFoundError,
    get, handler,
    http::StatusCode,
    listener::TcpListener,
    web::{Form, Html, Multipart, Path},
    EndpointExt, IntoResponse, Response, Route, Server,
};

use serde_json::Value;
use tera::{Context, Tera};

#[macro_use]
extern crate lazy_static;

use rust_embed::RustEmbed;
use serde::Deserialize;

//----------------------------------------------------------------------
// simple hello handler
//----------------------------------------------------------------------
#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

//----------------------------------------------------------------------
// file upload via form and save to file system
//----------------------------------------------------------------------
#[handler]
async fn upload_form_save() -> Html<&'static str> {
    Html(
        r###"
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Poem Toy</title>
        </head>
        <body>
            <form action="/upload_save" enctype="multipart/form-data" method="post">
                <input type="file" name="upload" id="file">
                <button type="submit">Submit</button>
            </form>
        </body>
        </html>
        "###,
    )
}

#[handler]
async fn upload_save(mut multipart: Multipart) -> &'static str {
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().map(ToString::to_string);
        let file_name = field.file_name().map(ToString::to_string).unwrap();
        if let Ok(bytes) = field.bytes().await {
            let fnc = &file_name.clone();
            let path = std::path::Path::new(fnc);
            fs::write(path, &bytes).expect("Unable to write file");
            println!(
                "SAVE: name={:?} filename={:?} length={}",
                name,
                file_name,
                bytes.len()
            );
        }
    }
    "File uploaded successfully!"
}

//----------------------------------------------------------------------
// embedded files
//----------------------------------------------------------------------
#[derive(RustEmbed)]
#[folder = "files"]
pub struct Files;

//----------------------------------------------------------------------
// module five
//----------------------------------------------------------------------
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

#[derive(Deserialize)]
struct ModuleFiveParams {
    merchant_id: String,
    store_number: String,
    street: String,
    city: String,
    state: String,
    zip: String,
}

fn json_to_hashmap(path_string: &str) -> Result<HashMap<String, Value>, Error> {
    let path = std::path::Path::new(path_string);
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let file_values: HashMap<String, Value> =
        serde_json::from_str(contents.as_str().as_ref()).unwrap();
    Ok(file_values)
}

fn write_string_to_file(data: String, path_str: &str) -> Result<File, io::Error> {
    let mut file = File::create(path_str)?;
    file.write_all(data.as_bytes())?;
    Ok(file)
}

fn hab_config_apply() {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let s = d.as_secs().to_string();

    // sudo ???
    // hab config apply
    //      poem-toy.default
    //      $(date +%s)
    //      changes.toml
    // hab config apply poem-toy.default $(date +%s) changes.toml

    let mut cmd = Command::new("/bin/hab");
    cmd.arg("config")
        .arg("apply")
        .arg("poem-toy.default")
        .arg(s)
        .arg("/hab/svc/poem-toy/files/changes.toml");
    // JAH: ABOVE is for running inside of supervisor
    // JAH: BELOW is for running outside of supervisor
    // .arg("changes.toml");

    println!("CMD:{:#?}", &cmd);

    let output = cmd.output();

    println!("COMMAND:\n{:#?}", output);
}

#[handler]
fn module_five() -> Result<Html<String>, poem::Error> {
    // let map = json_to_hashmap("habitat/config/application-settings.json").unwrap();
    // JAH: ABOVE is for running outside of supervisor
    // JAH: BELOW is for running inside of supervisor
    let map = json_to_hashmap("/hab/svc/poem-toy/config/application-settings.json").unwrap();

    let mut context = Context::new();
    context.insert("t_merchant_id", map.get("merchant_id").unwrap());
    context.insert("t_store_number", map.get("store_number").unwrap());
    context.insert("t_street", map.get("street").unwrap());
    context.insert("t_city", map.get("city").unwrap());
    context.insert("t_state", map.get("state").unwrap());
    context.insert("t_zip", map.get("zip").unwrap());

    TEMPLATES
        .render("module_five.html.tera", &context)
        .map_err(InternalServerError)
        .map(Html)
}

#[handler]
async fn module_five_process(Form(params): Form<ModuleFiveParams>) -> impl IntoResponse {
    let params= format!(
        "merchant_id = {:?}\nstore_number = {:?}\nstreet = {:?}\ncity = {:?}\nstate = {:?}\nzip = {:?}",
        params.merchant_id,
        params.store_number,
        params.street,
        params.city,
        params.state,
        params.zip
    );

    println!("PARAMS:\n{}", params);

    let f = write_string_to_file(params, "changes.toml");
    println!("F:{:#?}", f);

    hab_config_apply();

    let output = r###"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <title>Poem Toy</title>
            <meta charset="UTF-8">
            <style>
                body {
                  background-color:slategray;
                  color: linen;
                }
            </style>
        </head>
        <body>
            <p>Please give it a moment before going <a href="/">back to /</a>.  If it hasn't updated or hangs, please reload.</p>
        </body>
        </html>
        "###;

    Html(output)
}

//----------------------------------------------------------------------
// main
//----------------------------------------------------------------------
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug,tera=debug");
    }
    tracing_subscriber::fmt::init();
    // get(module_five_form).post(module_five_process),

    let app = Route::new()
        .at("/hello/:name", get(hello))
        .at("/upload_save", get(upload_form_save).post(upload_save))
        // .at("/", EmbeddedFileEndpoint::<Files>::new("index.html"))
        .at("/", get(module_five).post(module_five_process))
        .catch_error(|_: NotFoundError| async move {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Something Went Wrong")
        });

    Server::new(TcpListener::bind("127.0.0.1:2112"))
        .run(app)
        .await
}
