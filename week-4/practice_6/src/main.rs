// RUST PROGRAM TO COUNT NUMBERS

use std::io;

fn main() {
   println!("Enter Lower Bound");
   let mut input1 = String::new();
   io::stdin() .read_line(&mut input1) .expect("Failed to read input");
   let lower_bound:i32 = input1 .trim() .parse() .expect("Failed To Input");

    println!("Enter upper bound");
    let mut input2 = String::new();
    io::stdin() .read_line(&mut input2) .expect("Failed To Read Input");
    let upper_bound:i32 = input2 .trim() .parse() .expect("Failed To Input");


    for x in lower_bound..upper_bound{ // UPPER_BOUND IS NOT INCLUSIVE 
        println!("Count Level is {}",x);
    } 
}
