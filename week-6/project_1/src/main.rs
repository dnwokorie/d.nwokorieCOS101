use std::io;

fn main(){
    // PROGRAM TO WELCOME GUEST AND DELIVER THE MENU
println!("Welcome To Warrior Eats, May I Have Your Order Please");
println!("P Poundo Yam / Edinkaiko Soup - N3,200");
println!("F Fried Rice And Chicken - N3,000");
println!("A Amala And Ewedu Soup - N2,500");
println!("E Eba & Egusi soup - N2,000");
println!("W White Rice & Stew - N2,500");
 

    //PROGRAM TO ALLOW THE CUSTOMER SELECT A MEAL
println!("Please Enter food type code (P, F, A, E, W):");
let mut input1 = String::new();
io::stdin() .read_line(&mut input1) .expect("Not An Option");
let input1 = input1 .trim() .to_uppercase();
 

 // PROGRAM TO ASSIGN EACH DISH A PRICE

 let price: f64 = if input1 == "P" {
        3200.0
    } else if input1 == "F" {
        3000.0
    } else if input1 == "A" {
        2500.0
    } else if input1 == "E" {
        2000.0
    } else if input1 == "W" {
        2500.0
    } else {
        println!("Not An English Letter");
        return;
    };


// PROGRAM TO LET A CUSTOMER CHOOSE THE QUANTITY HE WANTS

println!("Please How Many Servings Do You Request:");
let mut input2 = String::new();
io::stdin() .read_line(&mut input2) .expect("Not A Number");
let quantity:f64 = match input2 .trim() .parse(){
    Ok(num) => num,
    Err(_) => {
        println!("Please Enter A Float");
        return;
    }
};

// PROGRAM TO CALCULATE THE CUSTOMER'S ORDER
let order = price * quantity;
println!("Your Bill Is N{}",order);

// PROGRAM TO GIVE THE CUSTOMER A DISCOUNT
if order > 10_000.00{
let discount = order *0.05;
let new_price = order - discount;
println!("Congratulations Your discount is {} And Your Bill Is {}",discount,new_price);
}







}