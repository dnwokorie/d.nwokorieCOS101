// RUST PROGRAM TO CAAALCULATE THE AREA OF A
// TRIANGLE FOR A GIVEN BASE AND HEIGHT

use std::io;

fn main() {
let mut input1 = String::new();
let mut input2 = String::new();

println!("Enter Base:");
io::stdin() .read_line(&mut input1) .expect("Not A Valid String");
let base:f32 = input1 .trim() .parse() .expect("Not A Valid Number");

println!("Enter Height:");
io::stdin() .read_line(&mut input2) .expect("Not a Valid String");
let height:f32 = input2 .trim() .parse() .expect("Not A Valid Number");

if base > 0.0 {
    let area:f32 = (base * height) / 2.0;
    println!("Area Of A Triangle: {}",area);
}    
}
