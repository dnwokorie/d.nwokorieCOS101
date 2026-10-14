fn main() {
    // NAME VECTOR
    let name = vec!["Mary","Sam","Sally","Greg","Ade","Mark","June","Ife"];

    // AGE VECTOR
    let age = vec![16,17,19,22,20,21,18,23];

    print!("\nAge allocation: \n");

    // LOOP TO ITERATE ELEMENTS IN VECTOR
    for i in 0..age.len()
    {
        // ITERATING THROUGH i ON THE VECTOR
        print!("{} is {} years old\n",name[i],age[i]);
    }
}
