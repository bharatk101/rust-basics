pub fn run() {
    let age: u8 = 22;
    let _check_id: bool = false;
    let _knows_person_of_age: bool = true;

    if age >= 21 && _check_id || _knows_person_of_age {
        println!("Bartender: What would you like to drink");
    } else if age < 21 && _check_id {
        println!("Bartender: Sorry you have to leave!");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthand IF

    let _is_of_age = if age >= 21 { true } else { false };
    println!("{}", _is_of_age);
}
