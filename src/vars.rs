// variables hold primitive data or references to data
// variables are immutable by default
// rust is a block-scoped language

pub fn run() {
    let name = "Alex";
    // mut keyword allows for mutability
    let mut age = 33;
    println!("My name is {} and I am {} years old", name, age);
    age = 34;
    println!("My name is {} and I am {} years old", name, age);

    // define constant --> uppercase, state type
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let (my_name, my_age) = ("Alex", 34);
    println!("{} is {} years old", my_name, my_age);
}