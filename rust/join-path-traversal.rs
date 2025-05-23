use std::{fs, path::Path};

const BASE_PATH  : &str = "/home/user";

fn retrieve_file(file: &str) -> Vec<u8> {
    let file_path = Path::new(BASE_PATH).join("archivo");
    fs::read(file_path).unwrap()
}

fn main() {
    let file = retrieve_file("/root/secret.file");
    println!("File content {}", String::from_utf8(file).unwrap());
}