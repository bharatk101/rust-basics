pub fn run() {
    // Print to console
    println!("Hello from the print.rs file!");

    //  Basic Formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "brad", "Mass");

    //  Poitional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Bo", "Jupyter", "code"
    );

    //  Named arguments
    println!("{name} likes to play {game}", name = "Bo", game = "Tennis");

    //  Placeholder traits
    println!("Binary: {:b} Hex: {:x} Oct: {:o}", 10, 10, 10);

    //  Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    //  Basic Math
    println!(" 10 + 10 = {}", 10 + 10);

    // Decimal Precision
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}
