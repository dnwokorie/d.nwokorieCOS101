fn main() {
    // A LIST OF NOS
    let v = vec![15,25,35,45,55];
    print_vector(v);
    // THIS LINE GIVES ERROR
}


fn print_vector(x: Vec<i32>){
    println!("Inside print_vector function {:?}",x);
    println!("{}", x[0]);
}