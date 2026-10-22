// PROGRAM TO CREATE A DATABASE FOR CONVICTED MINISTERS
use std::fs::File;
use std::io::Write;


fn main(){
    // GROUPING DATA WITH AN ARRAY
    let record_array = [
 ["Commissioner", "Ministry", "zone"],
 ["Aigbogun Alamba Daudu", "Internal Affairs", "South West"],
 ["Murtala Afeez Bendu", "Justice", "North East"],
 ["Okorocha Calistus Ogbona", "Defense", "South South"],
 ["Adewale Jimoh Akanbi", "Power & Steel", "South West"],
 ["Osazuwa Faith Etieye", "Petroleum", "South East"],
 ];

// CREATING THE FILE FOR DATA STORAGE
let mut file = File::create("Convicted_Ministers.csv")
.expect("Failed To Create The File");

// SEPERATION OF GROUPED ARRAYS AND ARRANGEMENT IN A FILE
for i in record_array{
    let ungrouped_record = format!("{}\n",i.join(","));
    file.write_all(ungrouped_record.as_bytes()) .expect("Failed To Write");
    }

    // COMPLETION INFORMATION
println!("Information Was Successfully Saved");
}

