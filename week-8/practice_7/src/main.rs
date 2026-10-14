fn main() {
    // INITIALIZATION OF TUPULE WITH DATA TYPE
    let datatype_tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
    println!("Tupule contents = {:?}",datatype_tuple);

    // INITIALIZATION OF TUPULE WITHOUT DATA TYPE
    let no_datatype_tuple =  ("Rust", "fun", 100);
    println!("Tuple contents = {:?}", no_datatype_tuple);

    // ACCESSING TUPLE ELEMENT AT INDEX 0
    println!("Value at Index 0 = {}",datatype_tuple.0);

    // ACCESSING TUPLE ELEMENT AT INDEX 1
    println!("Value at Index 1 = {}",datatype_tuple.1);

    // ACCESSING TUPLE ELEMENT AT INDEX 2
    println!("Value at Index 2 = {}",datatype_tuple.2);
}
