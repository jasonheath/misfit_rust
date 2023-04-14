fn main() {

    println!("if let some");

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {:?}", max);
    }
    // println!("Is max still in scope? {:?}", max); // the answer is no, it is not
    println!("Is config_max still in scope? {:?}", config_max);

}

