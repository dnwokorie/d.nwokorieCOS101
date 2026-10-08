fn main() {
    let num: i32 = 6;
    mut_by_zero(num);
    println!("The Value Is {}",num);
}

fn mut_by_zero(param_num: i32){
let param_num = param_num * 0;
println!("param number is: {}",param_num);
}
