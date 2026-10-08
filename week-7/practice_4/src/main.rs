// RUST PROGRAM TO PROVIDE THE SUM OF TWO VALUES FROM THE USER'S INPUT
use std::io;

fn main() {
    println!("===== Welcome To This Magnificent Program For Calculating The Sum Of Two Numbers =====");
    let mut a = String::new();
    let mut b = String::new();
    
// PROGRAM TO COLLECT USER DATA
    println!("Please Provide The First Input:");
    io::stdin() .read_line(&mut a) .expect("Not An Integer");
    let a = a .trim() .parse() .expect("Not An Integer");


    println!("Please Provide The Second Input:");
    io::stdin() .read_line(&mut b) .expect("Not An Integer");
    let b = b .trim() .parse() .expect("Not An Integer");

    sum(a,b);
}
// PROGRAM TO DEFINE THE ADDITION FUNCTION
fn sum(a:f64, b:f64){
let sum = a + b;    
println!("The Sum Of {} And {} Is {}", a, b, sum );
println!("Thank You For Endorsement");
}