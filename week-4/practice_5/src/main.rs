// RUST PROGRAM THAT READS HEIGHT 
// AND GIVES AN INTERPRETATION
 
 use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter Your Height (IN CENTIMETERS):");
    io::stdin() .read_line(&mut input) .expect("Not A Valid String");
    let height:f32 = input .trim() .parse() .expect("Not A Valid Number");

    if height >= 150.0 && height <= 170.0
    {
        println!("You Are An Average Height Person");
    }
    else if height > 170.0 && height <=195.0
    {
        println!("You Are Tall");
    }
    else if height < 150.0 && height > 100.0
    {
        println!("You Are A Dwarf");
    }
    else
    {
        println!("Abnormal Height");
    }
    
}
