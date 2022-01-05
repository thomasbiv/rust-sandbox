//Arrays - Fixed list where elements are the same data types (fixed length)

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5]; //mut allows values to be changed, NOT added
    
    //Re-assign value
    numbers[2] = 20;
    
    println!("{:?}", numbers);
    
    //Get single val
    println!("Single value: {}", numbers[0]);

    //Get array length
    println!("Array length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers)); //if we want, we can bring in the std::mem namespace with "use std::mem;" at the top of the file (similar to C++)

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}