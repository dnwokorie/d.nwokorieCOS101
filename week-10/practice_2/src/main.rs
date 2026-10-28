fn main() {
    let v = vec![10,20,30];
    // VECTOR V OWNS THE OBJECT IN HEAP

    let v2 = v;
    // MOVES OWNERSHIP TO v2

    display(v2);
    // v2 IS MOVED TO DISPLAY AND v2 IS INVALIDATED

    //println!("In main {:?}",v2);
    // v2 IS NO LONGER USABLE HERE

}

fn display(v2:Vec<i32>){
    println!("Inside Display {:?}",v2);
}
