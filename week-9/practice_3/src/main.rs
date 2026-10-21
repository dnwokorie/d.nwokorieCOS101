use std::fs;

fn main() {
    fs::remove_file("data.txt") .unwrap();
    println!("File Is Gone");
}
