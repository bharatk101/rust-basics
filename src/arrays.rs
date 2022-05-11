//  Array's are fixed size elements of same data type

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5]; // data type and the size

    println!("{:?}", numbers);

    // reassign a value
    numbers[2] = 9;

    //  get single value
    println!("{}", numbers[0]);

    //  Array length
    println!("Array Length: {}", numbers.len());

    //  Arrays are stack allocated
    println!("This array accoupies {} bytes", mem::size_of_val(&numbers));

    // slices from array
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}
