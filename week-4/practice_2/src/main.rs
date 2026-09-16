// RUST PROGRAM TO CALCULATE THE AREA OF A TRIANGLE GIVEN THREE SIDES

use std::io;

fn main() {
let mut input1 = String::new();
let mut input2 = String::new();
let mut input3 = String::new();


println!("Enter First Edge Of The Triangle:");
io::stdin() .read_line(&mut input1) .expect("Not A Valid String");
let a:f32 = input1 .trim().parse() .expect("Not A Valid Number");

println!("Enter Second Edge Of The Triangle");
io::stdin() .read_line(&mut input2) .expect("Not A Valid String");
let b:f32 = input2 .trim() .parse() .expect("Not A Valid Number");

println!("Enter The Third Edge Of The Triangle");
io::stdin() .read_line(&mut input3) .expect("Not A Valid String");
let c:f32 = input3 .trim() .parse() .expect("Not A Valid Number");

let s:f32 =(a + b + c) / 2.0;
let mut area:f32 = s * (s - a) * (s - b) * (s - c);
area =  area .sqrt();

println!("Area Of A Triangle: {}", area);


}
