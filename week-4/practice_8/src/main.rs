// RUST PROGRAM TO CREATE A SIMPLE LOOP

fn main() {

 // WHILE TRUE

 let mut x = 0;
 loop {
    x+=1;
    println!("x={}",x);

    if x==15 {
        break;
    }
  }   
}