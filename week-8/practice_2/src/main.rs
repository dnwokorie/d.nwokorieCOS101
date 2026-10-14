fn main() {
    let v = vec!['C','O','M','P','U','T','E','R'];
    
    let mut input1 = String::new();

    println!("Enter An Input Value btw (0 - 7)");
    std::io::stdin() .read_line(&mut input1) .expect("Failed To Read Input");

    //INDEX IS THE NON NEGATIVE VALUE WHICH IS SMALLER THAN THE SIZE OF THE VECTOR
    let index:usize = input1 .trim() .parse() .expect("Invalid Input");

    // GETTING VALUE AT GIVEN VALUE
    let ch: char = v[index];

    println!("{} Is The Character For Index [{}]\n",ch, index);


}
