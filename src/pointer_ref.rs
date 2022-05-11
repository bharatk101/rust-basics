//  Reference Pointers - Point to a resource in memory

pub fn run() {
    // Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("{:?}", (arr1, arr2));

    // With non primitives, if you assign another variable to a peice of data, the first
    // variable will no longer hold that value. You'll need to use a reference (&) to point

    //  vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("{:?}", (&vec1, vec2));
}
