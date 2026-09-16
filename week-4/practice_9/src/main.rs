// RUST PROGRAM TO DEMONSTRATE CONTINUE

fn main() {
    let mut count = 0;

    for num in 1..21 {
        if num > 10{
            println!("{:?}",num);
            continue;
        }
        count+=1;
    }
    println!("The Count Of Values Greater Than 10 (Between 1 And 20) is: {}",count);
    //outputs 10
}
