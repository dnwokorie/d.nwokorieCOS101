// CODE TO CREATE A STRUCTURE FOR THE LAPTOPS
struct Laptop{
brand: String,
cost: f32,
}

// PROGRAM TO DEFINE THE DEVICES AND CALCULATE THE COST OF PURCHASING 3
fn main(){
    let device1 = Laptop{
        brand: String::from("Hp"),
        cost: 650_000.00 * 3.00,
    };

    let device2 = Laptop{
        brand: String::from("IBM"),
        cost: 755_000.00 * 3.00,
    };

    let device3 = Laptop{
        brand: String::from("Toshiba"),
        cost: 550_000.00 * 3.00,
    };

    let device4 = Laptop{
        brand: String::from("Dell"),
        cost: 850_000.00 * 3.00,
    };

// CODE TO CALCULATE THE TOTAL COST OF ALL LAPTOPS AND INFORM THE USER

    let total_cost = device1.cost + device2.cost + device3.cost + device4.cost;
println!("\n====  Mr.Ogbeifuna Welcome To Warriors Rust Code  ====");  
println!("\nYou Bought 3 {} Laptops At {}",device1.brand, device1.cost);
println!("\nYou Bought 3 {} Laptops At {}",device2.brand, device2.cost);  
println!("\nYou Bought 3 {} Laptops At {}",device3.brand, device3.cost);  
println!("\nYou Bought 3 {} Laptops At {}",device4.brand, device4.cost);       
println!("\nThe Total Cost For All Three Laptops Each is {}",total_cost);

}