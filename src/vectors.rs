//  Vectors's are resiable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // reassign a value
    numbers[2] = 9;

    //  add on to vectors
    numbers.push(8);

    //  pop off last value
    numbers.pop();

    //  get single value
    println!("{}", numbers[0]);

    //  Vector length
    println!("Vector Length: {}", numbers.len());

    //   are stack allocated
    println!("This  accoupies {} bytes", mem::size_of_val(&numbers));

    // slices from array
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    //  Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //  loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
        println!("{}",  x);
    }
}
