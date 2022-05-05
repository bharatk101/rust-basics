// Primitive String = Immutable fixed length string somewhere in memory
// String = Growable heap-alloicated data structure - Use when you need to modify or own string data

pub fn run() {
    // Primitive string
    let _greetings = "Hello";

    // String
    let mut _new_greet = String::from("Hello ");

    println!("{}", _new_greet);
    println!("Length: {}", _new_greet.len());

    // adds/pushes a char
    _new_greet.push('W');

    //  pushed string
    _new_greet.push_str("orld!");

    println!("{}", _new_greet);

    //  Capacity in bytes
    println!("Capacity in bytes {}", _new_greet.capacity());

    //  Is empyt
    println!("Is Empty: {}", _new_greet.is_empty());

    //  Contains
    println!("Contains 'World' {}", _new_greet.contains("World"));

    // Replace
    println!(
        "Replace world with there {}",
        _new_greet.replace("World", "There")
    );

    // Loop through white spaces
    for word in _new_greet.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    //  Assertion tetsing
    assert_eq!(2, s.len());
    assert_eq!(11, s.capacity());
}
