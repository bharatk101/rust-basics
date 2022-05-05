/*
Primitive types --
1. Integers: u8, i8, u16, i16, .... (u -> unsigned, i-> integer, number of bit sthey take in memory)
Floats: f32, f34
Boolean: bool
Characters: char
Tuples
Arrays
*/

pub fn run() {
    //  default i32
    let x = 1;

    //  default f64
    let y = 2.5;

    // explicit type
    let z: i64 = 1234596870;

    //  find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // bool
    let is_active: bool = true;

    // get boolean rom an expression
    let is_greater = 10 > 5;

    //  CHAR
    let a1 = 'a';

    

    println!("{:?}", (x, y, z, is_active, is_greater, a1));
}
