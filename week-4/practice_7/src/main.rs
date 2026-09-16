// RUST PROGRAM TO ADD ONE TO A NUMBER LESS THAN TEN

use std::io;

fn main() {
    println!("Enter A Number");
    let mut input1 = String::new();
    io::stdin() .read_line(&mut input1) .expect("Failed To Read Input");
    let mut num:i32 = input1 .trim() .parse() .expect("Failed To Input");

    while num < 10 {
        println!("Inside Loop Number Value Is {}", num);
        num+=1;
    }
    println!("Outside Loop Number Value Is {}", num);
}
