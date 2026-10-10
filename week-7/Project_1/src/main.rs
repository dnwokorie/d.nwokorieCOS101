// RUST PROGRAM TO CALCULATE THE AREA OR VOLUME OF SELECTED SHAPES

use std::io;

// FUNCTION FOR THE AREA OF THE TRAPEZIUM
fn trapezium(){
    
    println!("Please, Input The First Base");
    let mut base1 = String::new();
    io::stdin() .read_line(&mut base1) .expect("Not An Integer");
    let base1:f64 = base1 .trim() .parse() .expect("Not A Float");


    println!("Please, Input The Second Base");
    let mut base2 = String::new();
    io::stdin() .read_line(&mut base2) .expect("Not An Integer");
    let base2:f64 = base2 .trim() .parse() .expect("Not A Float");

    
    println!("Please, Input The Height");
    let mut height = String::new();
    io::stdin() .read_line(&mut height) .expect("Not An Integer");
    let height:f64 = height .trim() .parse() .expect("Not A Float");

    let area1 = height / 2.00 * (base1 + base2);
    println!("Congratulations Your Area Is {}",area1);
    return; 

}

// FUNCTION TO CALCULATE THE AREA OF A RHOMBUS
fn rhombus(){
    println!("Please, Input The First Diagonal");
    let mut diagonal1 = String::new();
    io::stdin() .read_line(&mut diagonal1) .expect("Not An Integer");
    let diagonal1:f64 = diagonal1 .trim() .parse() .expect("Not A Float");

     println!("Please, Input The Second Diagonal");
    let mut diagonal2 = String::new();
    io::stdin() .read_line(&mut diagonal2) .expect("Not An Integer");
    let diagonal2:f64 = diagonal2 .trim() .parse() .expect("Not A Float");

    let area2 = 0.5 * diagonal1 * diagonal2;
    println!("Congratulations Your Area Is {}",area2);
    return; 
}


// FUNCTION TO CALCULATE THE AREA OF A PARALLELOGRAM
fn parallelogram(){
    println!("Please, Input The Base");
    let mut base3 = String::new();
    io::stdin() .read_line(&mut base3) .expect("Not An Integer");
    let base3:f64 = base3 .trim() .parse() .expect("Not A Float");

    println!("Please, Input The Altitude");
    let mut altitude = String::new();
    io::stdin() .read_line(&mut altitude) .expect("Not An Integer");
    let altitude:f64 = altitude .trim() .parse() .expect("Not A Float");

    let area3 = base3 * altitude;
    println!("Congratulations Your Area Is {}",area3);
    return;

}

// FUNCTION TO CALCULATE THE SURFACE AREA OF A CUBE
fn cube(){
    println!("Please, Input The Side Lenght");
    let mut side = String::new();
    io::stdin() .read_line(&mut side) .expect("Not An Integer");
    let side:f64 = side .trim() .parse() .expect("Not A Float");

    let area4 = 6.00 * (side * side);
    println!("Congratulations Your Area Is {}",area4);
    return;


}

// FUNCTION TO CALCULATE THE VOLUME OF A CYLINDER
fn cylinder(){
    println!("Please, Input The Radius");
    let mut radius = String::new();
    io::stdin() .read_line(&mut radius) .expect("Not An Integer");
    let radius:f64 = radius .trim() .parse() .expect("Not A Float");

    println!("Please, Input The Height");
    let mut height = String::new();
    io::stdin() .read_line(&mut height) .expect("Not An Integer");
    let height:f64 = height .trim() .parse() .expect("Not A Float");

    let pi:f64 = 3.14159265;

    let volume1 =  pi * (radius * radius) * height;
    println!("Congratulations Your Volume Is {}",volume1);
    return;
}


// PROGRAM TO GREET THE CUSTOMER AND GIVE HIM INSTRUCTIONS
fn main(){
println!("======Welcome To Warrior Solutions======");
println!("\nLet's Work On That Shape And Get The Area Or Volume");
println!("(T) Area Of A Trapezium");
println!("(R) Area Of A Rhombus");
println!("(P) Area Of A Parallelogram");
println!("(C) Surface Area Of A Cube");
println!("(Y) Volume Of A Cylinder");
println!("\nPlease Kindly Select ( T, R, P, C, Y ) From the Below Options");
    
    // PROGRAM TO LET THE CUSTOMER CHOOSE HIS OPTION 
    let mut choice = String::new();
    io::stdin() .read_line(&mut choice) .expect("Not Among The Options");
    let choice = choice .trim() .to_lowercase();

// PROGRAM TO COMPLETE THE CUSTOMERS REQUEST USING CONDITIONS
if choice == "t" {
println!("You Are Calculating For Area Of A Trapezium");
trapezium()
}

else if choice == "r" {
println!("You Are Calculating For Area Of A Rhombus");
rhombus()
}

else if choice == "p" {
println!("You Are Calculating For Area Of A Parallelogram");
parallelogram()
}

else if choice == "c" {
println!("You Are Calculating For The Surface Area Of A Cube");
cube()
}

else if choice == "y" {
println!("You Are Calculating For The Volume Of A Cylinder");
cylinder()
}

else{
    println!("Invalid Input, Please Select The Right Shape Keyword");
    return;
}

}