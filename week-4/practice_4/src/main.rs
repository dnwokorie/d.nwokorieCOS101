// RUST PROGRAM TO DETERMINE AGE PASS

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter Your Name:");
    io::stdin() .read_line(&mut input1) .expect("Not A Valid String");

    println!("Enter Your Age:");
    io::stdin() .read_line(&mut input2) .expect("Not A Valid String");
    let age:i32 = input2 .trim() .parse() .expect("Not A Valid Number");

    if age >=18 {
        println!("Welcome To The Party {}", input1);
    }
    else{
        println!("Oops, {} You Are Not Of Age To Enter The Party", input1);
    } 
}
