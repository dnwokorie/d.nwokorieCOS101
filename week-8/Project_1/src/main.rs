// PROGRAM TO CHECK THE APS LEVEL OF A WORKER

use std::io;


// CODE TO CHECK THE APS LEVEL OF AN OFFICE WORKER
fn office_admin() {
    println!("====Please Pick A Number As Assigned To Your Experience Level====");
    let exp: Vec<&str> = vec![
        "Intern (1)", 
        "Administrator (2)",
        "Senior Administrator (3)", 
        "Office Manager (4)", 
        "Director (5)", 
        "CEO (6)"
    ];
    for role in &exp {
        println!("{}", role);
    }
    
    // CODE TO ASK THE WORKER FOR HIS EXPERIENCE LEVEL 
    let mut exp1 = String::new();
    io::stdin().read_line(&mut exp1).expect("Not A Number");
    let exp1: i8 = exp1.trim().parse().expect("Not Among The Options");

    println!("==== How Many Years Of Experience Do You Have In The Industry ====");
    println!("Please, Enter Correctly The Exact Number");
    
    //CODE TO ASK THE WORKER FOR HIS INDUSTRIAL EXPERIENCE USING YEARS
    let mut exp_yr = String::new();
    io::stdin().read_line(&mut exp_yr).expect("Not A Number");
    let exp_yr: i8 = exp_yr.trim().parse().expect("Not Among The Options");

    
    // CODE TO CHECK FOR HIS APS LEVEL USING CONDITIONS
    if exp1 == 1 && exp_yr == 1 {
        println!("You Chose {}, And You Have {} Year Of Experience, So", exp[0], exp_yr);
        println!("Your APS Level Is 1 ");
    } 

    else if exp1 == 1 && exp_yr == 2 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp[0], exp_yr);
        println!("Your APS Level Is 2 ");
    } 

    else if exp1 == 2 && exp_yr == 3 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp[1], exp_yr);
        println!("Your APS Level Is 3 ");
    } 

    else if exp1 == 2 && exp_yr == 4 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp[1], exp_yr);
        println!("Your APS Level Is 4 ");
    } 

    else if exp1 == 2 && exp_yr == 5 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp[1], exp_yr);
        println!("Your APS Level Is 5 ");
    } 

    else if exp1 == 3 && exp_yr == 6 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp[2], exp_yr);
        println!("Your APS Level Is 6 ");
    } 

    else if exp1 == 3 && exp_yr == 7 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp[2], exp_yr);
        println!("Your APS Level Is 7 ");
    } 

    else if exp1 == 3 && exp_yr == 8 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp[2], exp_yr);
        println!("Your ELI Level Is 8 ");
    } 

    else if exp1 == 4 && exp_yr == 9 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp[3], exp_yr);
        println!("Your ELI Level Is 9 ");
    } 

    else if exp1 == 4 && exp_yr == 10 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp[3], exp_yr);
        println!("Your ELI Level Is 10 ");
    } 

    else if exp1 == 5 && exp_yr == 11 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp[4], exp_yr);
        println!("Your ELI Level Is 11 ");
    } 

    else if exp1 == 5 && exp_yr == 12 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp[4], exp_yr);
        println!("Your ELI Level Is 12 ");
    } 

    else if exp1 == 5 && exp_yr == 13 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp[4], exp_yr);
        println!("Your ELI Level Is 13 ");
    } 

    else if exp1 == 6 {
        println!("You Chose {}", exp[5]);
        println!("You Have Obtained The Highest Rank In The Industry (SES)");
    } 

    else {
        println!("Please Fill In Your Details Correctly");
    }
}

// CODE TO CHECK THE APS LEVEL OF AN OFFICE WORKER
fn academic() {
    println!("====Please Pick A Number As Assigned To Your Experience Level====");
    let acc_exp: Vec<&str> = vec![
        "Research Assistant (1)", 
        "PhD Candidate (2)", 
        "Post-Doc Researcher (3)", 
        "Senior Lecturer (4)", 
        "Dean (5)"
    ];
    for role in &acc_exp {
        println!("{}", role);
    }
     
      // CODE TO ASK THE WORKER FOR HIS EXPERIENCE LEVEL 
    let mut acc_exp1 = String::new();
    io::stdin().read_line(&mut acc_exp1).expect("Not A Number");
    let acc_exp1: i8 = acc_exp1.trim().parse().expect("Not Among The Options");

    println!("==== How Many Years Of Experience Do You Have In The Industry ====");
    println!("Please, Enter Correctly The Exact Number");
    
    //CODE TO ASK THE WORKER FOR HIS INDUSTRIAL EXPERIENCE USING YEARS
    let mut exp_yr1 = String::new();
    io::stdin().read_line(&mut exp_yr1).expect("Not A Number");
    let exp_yr1: i8 = exp_yr1.trim().parse().expect("Not Among The Options");

    
    // CODE TO CHECK FOR HIS APS LEVEL USING CONDITIONS
    if acc_exp1 == 1 && exp_yr1 == 3 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", acc_exp[0], exp_yr1);
        println!("Your APS Level Is 3 ");
    } 

    else if acc_exp1 == 1 && exp_yr1 == 4 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", acc_exp[0], exp_yr1);
        println!("Your APS Level Is 4 ");
    } 

    else if acc_exp1 == 1 && exp_yr1 == 5 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", acc_exp[0], exp_yr1);
        println!("Your APS Level Is 5 ");
    } 

    else if acc_exp1 == 2 && exp_yr1 == 6 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", acc_exp[1], exp_yr1);
        println!("Your APS Level Is 6 ");
    } 

    else if acc_exp1 == 2 && exp_yr1 == 7 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", acc_exp[1], exp_yr1);
        println!("Your APS Level Is 7 ");
    } 

    else if acc_exp1 == 2 && exp_yr1 == 8 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", acc_exp[1], exp_yr1);
        println!("Your ELI Level Is 8 ");
    } 

    else if acc_exp1 == 3 && exp_yr1 == 9 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", acc_exp[2], exp_yr1);
        println!("Your ELI Level Is 9 ");
    } 

    else if acc_exp1 == 3 && exp_yr1 == 10 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", acc_exp[2], exp_yr1);
        println!("Your ELI Level Is 10 ");
    } 

    else if acc_exp1 == 4 && exp_yr1 == 11 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", acc_exp[3], exp_yr1);
        println!("Your ELI Level Is 11 ");
    } 

    else if acc_exp1 == 4 && exp_yr1 == 12 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", acc_exp[3], exp_yr1);
        println!("Your ELI Level Is 12 ");
    } 

    else if acc_exp1 == 4 && exp_yr1 == 13 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", acc_exp[3], exp_yr1);
        println!("Your ELI Level Is 13 ");
    } 

    else if acc_exp1 == 5 {
        println!("You Chose {}", acc_exp[4]);
        println!("You Have Obtained The Highest Rank In The Industry (SES)");
    } 

    else {
        println!("Please Fill In Your Details Correctly");
    }
}

// CODE TO CHECK THE APS LEVEL OF AN OFFICE WORKER
fn lawyer() {
    println!("====Please Pick A Number As Assigned To Your Experience Level====");
    let law_exp: Vec<&str> = vec![
        "Paralegal (1)", 
        "Junior Associate (2)", 
        "Associate (3)", 
        "Senior Associate 1-2 (4)", 
        "Senior Associate 3-4 (5)", 
        "Partner (6)"
    ];
    for role in &law_exp {
        println!("{}", role);
    }
    
     // CODE TO ASK THE WORKER FOR HIS EXPERIENCE LEVEL  
    let mut law_exp1 = String::new();
    io::stdin().read_line(&mut law_exp1).expect("Not A Number");
    let law_exp1: i8 = law_exp1.trim().parse().expect("Not Among The Options");

    println!("==== How Many Years Of Experience Do You Have In The Industry ====");
    println!("Please, Enter Correctly The Exact Number");
    
    //CODE TO ASK THE WORKER FOR HIS INDUSTRIAL EXPERIENCE USING YEARS
    let mut exp_yr2 = String::new();
    io::stdin().read_line(&mut exp_yr2).expect("Not A Number");
    let exp_yr2: i8 = exp_yr2.trim().parse().expect("Not Among The Options");

    
    // CODE TO CHECK FOR HIS APS LEVEL USING CONDITIONS
    if law_exp1 == 1 && exp_yr2 == 1 {
        println!("You Chose {}, And You Have {} Year Of Experience, So", law_exp[0], exp_yr2);
        println!("Your APS Level Is 1 ");
    } 

    else if law_exp1 == 1 && exp_yr2 == 2 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", law_exp[0], exp_yr2);
        println!("Your APS Level Is 2 ");
    } 

    else if law_exp1 == 2 && exp_yr2 == 3 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", law_exp[1], exp_yr2);
        println!("Your APS Level Is 3 ");
    } 

    else if law_exp1 == 2 && exp_yr2 == 4 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", law_exp[1], exp_yr2);
        println!("Your APS Level Is 4 ");
    } 

    else if law_exp1 == 2 && exp_yr2 == 5 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", law_exp[1], exp_yr2);
        println!("Your APS Level Is 5 ");
    } 

    else if law_exp1 == 3 && exp_yr2 == 6 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", law_exp[2], exp_yr2);
        println!("Your APS Level Is 6 ");
    } 

    else if law_exp1 == 3 && exp_yr2 == 7 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", law_exp[2], exp_yr2);
        println!("Your APS Level Is 7 ");
    } 

    else if law_exp1 == 3 && exp_yr2 == 8 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", law_exp[2], exp_yr2);
        println!("Your ELI Level Is 8 ");
    } 

    else if law_exp1 == 4 && exp_yr2 == 9 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", law_exp[3], exp_yr2);
        println!("Your ELI Level Is 9 ");
    } 

    else if law_exp1 == 4 && exp_yr2 == 10 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", law_exp[3], exp_yr2);
        println!("Your ELI Level Is 10 ");
    } 

    else if law_exp1 == 5 && exp_yr2 == 11 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", law_exp[4], exp_yr2);
        println!("Your ELI Level Is 11 ");
    } 

    else if law_exp1 == 5 && exp_yr2 == 12 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", law_exp[4], exp_yr2);
        println!("Your ELI Level Is 12 ");
    } 

    else if law_exp1 == 5 && exp_yr2 == 13 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", law_exp[4], exp_yr2);
        println!("Your ELI Level Is 13 ");
    } 

    else if law_exp1 == 6 {
        println!("You Chose {}", law_exp[5]);
        println!("You Have Obtained The Highest Rank In The Industry (SES)");
    }

    else {
        println!("Please Fill In Your Details Correctly");
    }
}

// CODE TO CHECK THE APS LEVEL OF AN OFFICE WORKER
fn teacher() {
    println!("====Please Pick A Number As Assigned To Your Experience Level====");
    let exp4: Vec<&str> = vec![
        "Placement (1)", 
        "Classroom Teacher (2)", 
        "Senior Teacher (3)", 
        "Leading Teacher (4)", 
        "Deputy Principal (5)", 
        "Principal (6)"
    ];
    for role in &exp4 {
        println!("{}", role);
    }
    
     // CODE TO ASK THE WORKER FOR HIS EXPERIENCE LEVEL  
    let mut teach_exp1 = String::new();
    io::stdin().read_line(&mut teach_exp1).expect("Not A Number");
    let teach_exp1: i8 = teach_exp1 .trim().parse().expect("Not Among The Options");

    println!("==== How Many Years Of Experience Do You Have In The Industry ====");
    println!("Please, Enter Correctly The Exact Number");
    
    //CODE TO ASK THE WORKER FOR HIS INDUSTRIAL EXPERIENCE USING YEARS
    let mut exp_yr3 = String::new();
    io::stdin().read_line(&mut exp_yr3).expect("Not A Number");
    let exp_yr3: i8 = exp_yr3.trim().parse().expect("Not Among The Options");
    

    // CODE TO CHECK FOR HIS APS LEVEL USING CONDITIONS
    if teach_exp1 == 1 && exp_yr3 == 1 {
        println!("You Chose {}, And You Have {} Year Of Experience, So", exp4[0], exp_yr3);
        println!("Your APS Level Is 1 ");
    } 

    else if teach_exp1 == 1 && exp_yr3 == 2 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp4[0], exp_yr3);
        println!("Your APS Level Is 2 ");
    } 

    else if teach_exp1 == 2 && exp_yr3 == 3 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp4[1], exp_yr3);
        println!("Your APS Level Is 3 ");
    } 

    else if teach_exp1 == 2 && exp_yr3 == 4 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp4[1], exp_yr3);
        println!("Your APS Level Is 4 ");
    } 

    else if teach_exp1 == 2 && exp_yr3 == 5 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp4[1], exp_yr3);
        println!("Your APS Level Is 5 ");
    } 

    else if teach_exp1 == 3 && exp_yr3 == 6 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp4[2], exp_yr3);
        println!("Your APS Level Is 6 ");
    } 

    else if teach_exp1 == 3 && exp_yr3 == 7 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp4[2], exp_yr3);
        println!("Your APS Level Is 7 ");
    } 

    else if teach_exp1 == 3 && exp_yr3 == 8 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp4[2], exp_yr3);
        println!("Your ELI Level Is 8 ");
    } 

    else if teach_exp1 == 4 && exp_yr3 == 9 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp4[3], exp_yr3);
        println!("Your ELI Level Is 9 ");
    } 

    else if teach_exp1 == 4 && exp_yr3 == 10 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp4[3], exp_yr3);
        println!("Your ELI Level Is 10 ");
    } 

    else if teach_exp1 == 5 && exp_yr3 == 11 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp4[4], exp_yr3);
        println!("Your ELI Level Is 11 ");
    } 

    else if teach_exp1 == 5 && exp_yr3 == 12 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp4[4], exp_yr3);
        println!("Your ELI Level Is 12 ");
    } 

    else if teach_exp1 == 5 && exp_yr3 == 13 {
        println!("You Chose {}, And You Have {} Years Of Experience, So", exp4[4], exp_yr3);
        println!("Your ELI Level Is 13 ");
    } 

    else if teach_exp1 == 6 {
        println!("You Chose {}", exp4[5]);
        println!("You Have Obtained The Highest Rank In The Industry (SES)");
    }

    else {
        println!("Please Fill In Your Details Correctly");
    }
}

fn main() {
    // CODE TO RECEIVE THE WORKERS PROFILE
println!("Goodday User, Welcome to Warrior's APS Level Checker");
println!("==== Please Pick A Number As Assigned To Your Field In The Industry ====");
let prof: Vec<&str> = vec!["Office Admin (1)", "Academic (2)", "Lawyer (3)" , "Teacher(4)" ];
for field in &prof {
println!("{}", field);
}
let mut prof_c = String::new();
io::stdin().read_line(&mut prof_c).expect("Not A Number");
let prof_c: i8 = prof_c.trim().parse().expect("Not Among The Options");

// CODE TO CHECK HIS APS LEVEL BY REFFERING TO PAST FUNCTIONS
if prof_c == 1 {
office_admin();
} 

else if prof_c == 2 {
academic();
} 

else if prof_c == 3 {
lawyer();
} 

else if prof_c == 4 {
teacher();
} 

else {
println!("Please Select A Listed Profession");
}
}
