fn main()
{
	// DEFINITION OF VARIABLES
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.00;
	let t:f64 = 5.00;

	//CODE TO CALCULATE COMPOUND INTEREST
	let a = p * (1.0 + (r / 100.00)). powf(t);
    println!("Amount is {}", a);
    let ci = a - p;
    println!("compound interest is {}", ci);
}