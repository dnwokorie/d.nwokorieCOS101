// PROGRAM TO CREATE A STUDENT DATABASE SYSTEM
use std::fs::File;
use std::io::Write;


fn main(){
    // GROUPING DATA WITH AN ARRAY
    let record_array = [
 ["Student Name", "Matric Number", "Department", "Level"],
 ["Oluchi Mordi", "ACC10211111", "Accounting", "300"],
 ["Adams Aliyu", "ECO10110101", "Economics", " 100"],
 ["Shania Bolade", "CSC10328828", "Computer", "200"],
 ["Adekunle Gold", "EEE11020202", "Electrical", "200"],
 ["Blanca Edemoh", "MEE10202001", "Mechanical", "100"],
 ];

 // CREATING THE FILE FOR DATA STORAGE
let mut file = File::create("PAU Student Management Information System records.csv")
.expect("Failed To Create The File");

// SEPERATION OF GROUPED ARRAYS AND ARRANGEMENT IN A FILE
for i in record_array{
    let ungrouped_record = format!("{}\n" , i.join(","));
    file.write_all(ungrouped_record.as_bytes()) .expect("Failed To Write");
    }
    
    // COMPLETION INFORMATION
println!("Information Was Successfully Saved");
}

