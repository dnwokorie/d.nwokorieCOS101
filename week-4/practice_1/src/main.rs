// RUST PROGRAM TO OUTPUT NAME AND AGE

use std::io;

fn main() {
println!("\nStudents Information Management System!");


// INPUT NAME
println!("\nPlease Enter Your Name.");
let mut name = String::new();
   io::stdin()
   .read_line(&mut name)
   .expect("Failed To Read Input");
println!("Your Name Is {}", name);


// INPUT AGE
println!("\nEnter Your Age.");
let mut age = String::new();
    io::stdin()
    .read_line(&mut age)
    .expect("Failed to read input");
let age:u16 = age.trim(). parse().expect("Input Is Not An Integer");
println!("Your Age Is: {}", age);













}
