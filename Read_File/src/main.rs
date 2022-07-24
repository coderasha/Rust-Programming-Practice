use std::fs::File;
use std::io::prelude::*;
fn main() {

    let mut file = File::open("info.txt").expect("The file is not opening");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("OOps!! It dodn't open");

    println!("Hello, world! {}",contents);
}
