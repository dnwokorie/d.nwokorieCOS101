// RUST PROGRAM TO CHECK IF AN INPUT IS A DIGIT
use std::io;

// PROGRAM TO DEFINE THE FUNCTION
fn checker(){
    println!("Please Enter A Digit:");
    let mut input = String::new();
    io::stdin() .read_line(&mut input) .expect("Not A Character");
    let ch:char = input .trim() .parse() .expect("Not A Character");

// PROGRAM TO GIVE CONDITIONS TO THE FUNCTION
if ch >= '0' && ch <= '9' {
    println!("{} Is A Digit", ch);
}
else{
    println!("{} Is Not A Digit", ch);
}
}



fn main() {
println!("This Is A Program to Help You Identify A Digit");    
checker();
}
