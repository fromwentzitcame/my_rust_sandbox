pub fn run() {
    // print to console
    println!("Hello from the print.rs file");

    // basic formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Ellen", "Michigan");

    // positional arguments
    println!(
        "{0} is a {1} and {0} likes to eat {2}",
        "Cranberry", "cat", "chicken"
    );

    // named arguments
    println!(
        "{name} likes to take {activity}",
        name = "Lucy",
        activity = "naps"
    );

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    // called a tuple?
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}
