// DECLARE A STRUCTURE
struct Employee {
ceo: String,
company: String,
age:u8,
}

fn main() {
    // INITIALIZE A STRUCTURE
    let emp1 = Employee {
        company: String::from("Microsoft"),
        ceo: String::from("Jobara"),
        age: 45,
    };

     // INITIALIZE A STRUCTURE
    let emp2 = Employee {
        company: String::from("Apple"),
        ceo: String::from("Sandaskin"),
        age: 43,
    };
    // PASS emp1 AND emp2 TO DISPLAY()
    display(emp1);
    display(emp2);
}

// FETCH VALUES OF SPECIFIC STRUCTURE FIELDS USING THE
// OPERATOR AND PRINT IT TO THE CONSOLE

fn display(emp: Employee){
    println!("Name is {} company is {} age is {}",emp.ceo,emp.company,emp.age);
}
