// DEFINE DIMENSIONS OF A RECTANGLE
struct Rectangle {
    width:u32, height:u32
}

// LOGIC TO CALCULATE AREEA OF A RECTANGLE
impl Rectangle{
    fn area(&self) -> u32 {
        // USE THE . OPERATOR TO FETCH THE
        // VALUE OF A FIELD VIA THE SELF KEYWORD
        self.width * self.height
    }
}

fn main() {
    // INSTANTIATE THE STRUCTURE
    let small = Rectangle {
        width:10,
        height:20,
    };
println!("Width is {}\n height is {}\n area of Rectangle is {}",small.width,small.height,small.area());

}