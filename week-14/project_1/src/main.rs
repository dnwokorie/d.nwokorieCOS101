// PROGRAM TO ALLOCATE EACH DATABASE TO THE RIGHTFUL USER
use std::fs::File;
use std::io::{self, Read};

// CODE TO OPEN THE SQL FILE
fn read_sql_file(filename: &str) {
    let mut file = File::open(filename).expect("Failed to open the SQL dump file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read");
    println!("\n--- Showing Database for {} ---\n", filename);
    println!("{}", contents);
}

// CODE TO ALLOCATE EACH SQL FILE TO EACH USERS INPUT AS A FUNCTION
fn administrator() {
    read_sql_file("globacom_db.sql");
}

fn project_manager() {
    read_sql_file("project_tb.sql");
}

fn employee() {
    read_sql_file("staff_tb.sql");
}

fn customer() {
    read_sql_file("customer_tb.sql");
}

fn vendor() {
    read_sql_file("dataplan_tb.sql");
}

// CODE TO WELCOME USER AND ASK FOR INFORMATION
fn main() {
    println!("\n====GLOBACOM DATABASE ACCESS TOOL====");
    println!("Please, Select Your Role Using The Provided Numbers Below:");
    println!("1. Administrator");
    println!("2. Project Manager");
    println!("3. Employee");
    println!("4. Customer");
    println!("5. Vendor");
    
    //  CODE TO COLLECT INFORMATION FROM THE USER
    let mut user_input = String::new();
    io::stdin() .read_line(&mut user_input) .expect("Not Among The Options");
    let user_input:u8 = user_input.trim() .parse() .expect("Not A Number");

    if user_input == 1{
    administrator()
    }

    else if user_input == 2{
    project_manager()
    }

    else if user_input == 3{
    employee()
    }

    else if user_input == 4{
    customer()
    }

    else if user_input == 5{
    vendor()
    }

    else {
    println!("Please Input A Number Among The Listed Options");

    }
}
