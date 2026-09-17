//RUST PROGRAM TO SOLVE A QUADRATIC EQUATION

use std::io;

fn main() {
println!("HELLO,LET ME HELP YOU WITH THAT QUADRATIC EQUATION");

//CODE TO INPUT THE FIRST NUMBER

println!("Input The First Number");
let mut a = String::new();
io::stdin() .read_line(&mut a) .expect("Not An Integer");
let a:i64 = a .trim() .parse() .expect("Not A Number");

// CODE TO INPUT THE SECOND NUMBER

println!("Input The Second Number");
let mut b = String::new();
io::stdin() .read_line(&mut b) .expect("Not An Integer");
let b:i64 = b .trim() .parse() .expect("Not A Number");

// CODE TO INPUT THE THIRD NUMBER

println!("Input The Third Number");
let mut c = String::new();
io::stdin() .read_line(&mut c) .expect("Not An Integer");
let c:i64 = c .trim() .parse() .expect("Not A Number");

// BREAKING DOWN THE QUADRATIC FORMULA
let d:i64 =  (b * b) - (4 * a * c);

// REFORMING THE FORMULA
let e:i64 = d .isqrt();

// GETTING THE FIRST ROOT
let first_root = (-b + e) / 2 * a;
println!("THE FIRST ROOT IS {}",first_root);

// GETTING THE SECOND ROOT
let second_root = (-b - e) / 2 * a;
println!("THE SECOND ROOT IS {}",second_root);

}
