fn main() {
	//DEFINITION OF VARIABLES
	let p:f64 = 210_000.00;
	let n:f64 = 3.00;
	let r:f64 = 5.00;

	//PROGRAM TO SOLVE FOR THE VALUE OF THE TV SET
	let a = p * (1.00 - (r / 100.00)) .powf(n);
	println!("VALUE OF TV is {}", a);
}