fn main() {

    // AN ARRAY OF NUMBERS
    let numbers = [1,2,3,4,5];
    println!("Original array = {:?}", numbers);

    // CREATE A SLICE OF 2ND AND 3RD ELEMENT
    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced = {:?}",slice1);


    // OMIT THE START INDEX
    let slice2 = &numbers[..3];
    
    // THIS MEANS THE SLICE STARTS FROM INDEX 0 AND GOES UP TO INDEX 3 (EXCLUSIVE)
    println!("Index 0 to index 3 sliced = {:?}", slice2);

    // OMIT THE END INDEX
    let slice3 = &numbers[2..];

    // THIS MEANS THE SLICE STARTS FROM INDEX 2 AND GOES UP TO INDEX 5 (EXCLUSIVE)
    println!("Index 2 to index 5 sliced = {:?}",slice3);

    // OMIT THE START INDEX AND THE END INDEX
    // REFERENCE THE WHOLE ARRAY 
    
    let slice4 = &numbers[..];
    
    // THIS MEANS THE SLICE STARTS FROM INDEX 0 AND GOES UP TO INDEX 5 (EXCLUSIVE).

    println!("Index 0 to index 5 sliced = {:?}", slice4);
}
