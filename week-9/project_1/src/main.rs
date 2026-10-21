// PROGRAM TO INPUT STRINGS INTO A COMPANY INVENTORY FILE

use std::fs::File;
use std::io::Write;

// CODE TO GREET THE USER OF THE PROGRAM
fn greeting() {
    println!("Welcome To Nigerian Breweries Plc");
    println!("The Inventory has been Printed In the data file");
}

fn main() {
    greeting();

    // CODE TO CREATE A VECTOR TO CONTAIN THE INDIVIDUAL VECTORS OF THE INVENTORY
        let inventory = vec![
        vec!["Lager", "Stout", "Non-Alcoholic"],
        vec!["33 Export", "Legend", "Maltina"],
        vec!["Desperados", "Turbo King", "Amstel Malta"],
        vec!["Goldberg", "Williams", "Malta Gold"],
        vec!["Gulder", "", "Fayrouz"],
        vec!["Heineken", "", ""],
        vec!["Star", "", ""],
    ];

    // CODE TO CREATE THE INVENTORY FILE
    let mut file = File::create("data.csv").expect("Failed To Create File");

    // CODE TO INDIVIDUALLY LIST THE ROWS IN A NEW LINE EACH AND SEPERATE WITH A COMMA
    for row in inventory {
        let inventory_data = format!("{}\n",row.join(","));
        file.write_all(inventory_data.as_bytes()).expect("Failed to Write");
    }
}