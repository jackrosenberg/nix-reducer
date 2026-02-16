use std::{fs, env};

fn main() {
    // take in command line args
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let path = &args[1];
    println!("Searching path '{}'", path);

    let contents = fs::read_to_string(path)
        .expect("Unable to read path ");

    println!("File contains \n '{}'", contents);

}
