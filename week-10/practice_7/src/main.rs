struct Employee {
    name: String,
    company: String,
    age: u8,
}



fn main() {
    let emp1 = Employee {
        company: String::from("Ernest & Young"),
        name: String::from("Abiola Johnson"),
        age: 34
    };
println!("Name = {}\n",emp1.name);
println!("Company = {}\n",emp1.company);
println!("Age = {}\n",emp1.age);

}

