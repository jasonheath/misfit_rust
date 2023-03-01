use std::{os::unix::prelude::OsStrExt, path::Path};

fn main() {
    // comparisons();
    path_methods();
}
fn path_methods() {
    let prefix = Path::new("/tmp/jahpaths");
    let dir_with_sep = Path::new("/tmp/jahpaths/foo/bar/baz/qux/");
    let dir_without_sep = Path::new("/tmp/jahpaths/foo/bar/baz/qux");
    let fbbq = Path::new("/tmp/jahpaths/foo/bar/baz/qux/file.txt");
    let file = Path::new("file.txt");
    let ext = Path::new("txt");
    let n = 28;
    // fs::create_dir_all(path).unwrap();
    // let _file = std::fs::File::create(fbbq).unwrap();

    // println!("{:n$}{:?}", "prefix", prefix);
    // println!("{:n$}{:?}", "fbbq", fbbq);
    // println!("{:n$}{:?}", "file", file);
    // println!("{:n$}{:?}", "ext", ext);

    println!("{:n$}{:?}", "fbbq.is_absolute()", fbbq.is_absolute());
    println!("{:n$}{:?}", "fbbq.is_relative()", fbbq.is_relative());
    println!("{:n$}{:?}", "fbbq.has_root()", fbbq.has_root());
    println!("{:n$}{:?}", "fbbq.parent()", fbbq.parent());
    println!("{:n$}{:?}", "fbbq.ancestors()", fbbq.ancestors());
    println!("{:n$}{:?}", "fbbq.file_name()", fbbq.file_name());
    println!(
        "{:n$}{:?}",
        "fbbq.strip_prefix(prefix)",
        fbbq.strip_prefix(prefix)
    );
    println!(
        "{:n$}{:?}",
        "fbbq.starts_with(prefix)",
        fbbq.starts_with(prefix)
    );
    println!("{:n$}{:?}", "fbbq.starts_with(file)", fbbq.ends_with(file));
    println!("{:n$}{:?}", "fbbq.starts_with(ext)", fbbq.ends_with(ext));
    println!("{:n$}{:?}", "fbbq.file_stem()", fbbq.file_stem());
    // nightly-only println!("{:n$}{:?}", "fbbq.file_prefix()", fbbq.file_prefix());
    println!("{:n$}{:?}", "fbbq.extension()", fbbq.extension());
    // println!(
    //     "{:n$}{:?}",
    //     "fbbq.metadata()",
    //     fbbq.metadata().expect("metadata call failed")
    // );
    // println!(
    //     "{:n$}{:?}",
    //     "fbbq.metadata()",
    //     fbbq.symlink_metadata().expect("symlink_metadata call failed")
    // );

    println!(
        "{:n$}{:?}",
        "dir_with_sep.file_name()",
        dir_with_sep.file_name()
    );
    println!(
        "{:n$}{:?}",
        "dir_with_sep.extension()",
        dir_with_sep.extension()
    );
    println!(
        "{:n$}{:?}",
        "dir_with_sep.file_stem()",
        dir_with_sep.file_stem()
    );

    println!(
        "{:n$}{:?}",
        "dir_without_sep.file_name()",
        dir_without_sep.file_name()
    );
    println!(
        "{:n$}{:?}",
        "dir_without_sep.extension()",
        dir_without_sep.extension()
    );
    println!(
        "{:n$}{:?}",
        "dir_without_sep.file_stem()",
        dir_without_sep.file_stem()
    );

    // println!("{:n$}{:?}", "dir_with_sep.ends_with()", dir_with_sep.ends_with(std::path::MAIN_SEPARATOR));
    // println!("{:n$}{:?}", "dir_without_sep.ends_with()", dir_without_sep.ends_with());

    println!(
        "{:n$}{:?}",
        "dir_with_sep.experiment",
        dir_with_sep.as_os_str().as_bytes().last()
    );
    println!(
        "{:n$}{:?}",
        "std::path::MAIN_SEPARATOR",
        std::path::MAIN_SEPARATOR
    );
    println!(
        "{:n$}{:?}",
        "std::path::MAIN_SEPARATOR",
        std::path::MAIN_SEPARATOR.to_string().as_bytes()
    );
    //let a: u8 = dir_with_sep.as_os_str().as_bytes().last().unwrap_or(0);
    let a: u8 = *(dir_with_sep.as_os_str().as_bytes().last().take().unwrap_or(&0));
    let b: u8 = std::path::MAIN_SEPARATOR.to_string().as_bytes()[0];

    println!("{:n$}{:?}", "a == b", a == b);

}

fn _comparisons() {
    let foo = Path::new("/tmp/jahpaths/foo");
    let bar = Path::new("/tmp/jahpaths/bar");

    println!("foo == foo is '{:?}'", foo == foo);
    println!("foo == bar is '{:?}'", foo == bar);
    println!("foo != bar is '{:?}'", foo != bar);
    println!("foo >= foo is '{:?}'", foo >= foo);
    println!("foo >= bar is '{:?}'", foo >= bar);
    println!("foo <= foo is '{:?}'", foo <= foo);
    println!("foo <= bar is '{:?}'", foo <= bar);
}
