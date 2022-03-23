pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    println!("Number: {}", 1);

    // multiple placeholders 
    println!("{} is from {}", "Anjan", "India");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Anjan", "India", "Code");

    // Named Arguments
    println!("{name} likes to play {activity}" , name="Anjan", activity="Cricket");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Debug traits
    println!("{:?}", (12, true, "Hello"));
}