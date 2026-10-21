use std::fs::OpenOptions;
use std::io::Write;



fn main() {
    let mut file = OpenOptions::new() .append(true).open("data.txt").unwrap();
    file.write_all("This Is A program\n".as_bytes()) .unwrap();
    file.write_all("This is Another Program\n".as_bytes()).unwrap();
    println!("File Edit Is done");
}
