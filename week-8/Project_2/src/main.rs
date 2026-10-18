// PROGRAM TO SELECT THE DEVELOPER WITH THE HIGHEST EXPERIENCE

use std::io;

fn input(display: &str) -> String {
    println!("{}", display);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid Input");
    input.trim().to_string()
}

// CODE TO WELCOME THE USER AND COLLECT DATA FOR THE PROGRAM
fn main() {
        println!("==== Welcome To EY Nigeria's Developer Experience Scout System ====");
        let candidate_num = input("\nEnter The Number Of Candidates You Would Like To Interview:");
        let total_candidates:i8 = candidate_num .parse() .expect("Not A Number");
        
        // CODE TO PUT THE CANDIDATE'S NAME AND YEARS OF EXPERIENCE IN A TUPLE
        // CONTAINED IN A VECTOR
        if total_candidates > 0 {
        let mut candidate_list: Vec<(String, i8)> = Vec::new();

            // CODE TO COLLECT CANDIDATES DETAILS
            for current_candidate in 1..=total_candidates {
            println!("\nCANDIDATE {}", current_candidate);

            let name = input("Enter Candidate Name:");

            let exp_str = input("Enter The Candidate's Years Of Programming Experience:");
            let experience:i8 = exp_str.parse() .expect("Not A Number");

            // CODE TO PUSH THE TUPLE INTO THE VECTOR
            candidate_list.push((name, experience));
        }

        // CODE TO DETERMINE THE CANDIDATE WITH THE HIGHEST EXPERIENCE
        let mut highest_name = String::new();
        let mut highest_exp = -1;

        // CODE TO LOOP THROUGH THE VECTOR AND COMPARE EXPERIENCE YEARS USING CONDITIONS
        for candidate in &candidate_list {
            if candidate.1 > highest_exp {
                highest_exp = candidate.1;
                highest_name = candidate.0.clone();
            }
        }

        // CODE TO DISPLAY THE CANDIDATE WITH THE HIGHEST EXPERIENCE
        println!("==== Candidate with the Highest Experience Is: ====");
        println!("NAME: {}", highest_name);
        println!("EXPERIENCE: {} YEARS", highest_exp);
    } 

    else {
        println!("\nPlease Follow The Instructions Correctly.");
    }
}