//RUST PROGRAM TO HELP AN EMPLOYER IN EMPLOYMENT

use std::io;

fn main() {
// CODE TO ASK IF THE EMPLOYEE IS EXPERIENCED
println!("Is The Employee Experienced Enter (experienced Or unexperienced)");
let mut ex = String::new();
io::stdin() .read_line(&mut ex) .expect("Not An Integer");
let experienced = ex .trim() .to_lowercase();

// CODE TO TELL THE PROGRAM THE EMPLOYEES AGE
let mut a = String::new();
println!("Enter The Employees Age:");
io::stdin() .read_line(&mut a) .expect("Not An Integer");
let age:u8 = a .trim() .parse() .expect("Not A Correct Number");

// CODE TO DETERMINE THE EMPLOYEES SALARY
if experienced == "experienced"{
 if age >= 40{
    println!("The Employee Should Be Paid N1,560,000");
}
else if age >= 30 && age <= 39{
    println!("The Employee Should Be Paid N1,480,000");
}

else if age <= 29{
    println!("The Employee Should Be Paid N1,300,000");
}
}

else if experienced == "unexperienced"{
 println!("The Employee Should Be Paid N100,000");
}
else{
    println!("Please Enter Either experienced or unexperienced");
}
}

