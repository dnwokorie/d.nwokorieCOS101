fn display(v:Vec<i32>) -> Vec<i32>{
    // RETURNING SAME VECTOR
    println!("Inside display {:?}",v);
    return v;
}


fn main() {
    let v = vec![20,30,60,80];
    // VECTOR V OWNS THE OBJECT IN HEAP

    let v2 = v;
    let v2_return = display(v2);
    println!("In main {:?}",v2_return);
}
