fn main() {
    // MUTABLE ARRAY
    let mut colors = ["red","green","yellow","white"];

    println!("\nOriginal array = {:?}",colors);

    // MUTABLE SLICE
    let sliced_colors = &mut colors[1..3];

    println!("First slice = {:?}", sliced_colors);

    // CHANGE THE VALUE OF THE ORIGINAL SLICE AT THE FIRST INDEX
    sliced_colors[1] = "purple";

    println!("Changed slice = {:?}", sliced_colors);
}
