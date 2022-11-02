// Primitive str = immutable fixed-length string somewhere in memory; ex, can't add to it with .push()
// String = growable, heap-allocated data structure - used when you need to modify or own string data

pub fn run() {
    // primitive
    // let hello = "Hello";
    // growable
    let mut hello = String::from("Hello ");

    // get length
    println!("Length: {}", hello.len());

    // push char
    hello.push('W');

    // push string
    hello.push_str("orld!");

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty? Boolean
    println!("Is Empty: {}", hello.is_empty());

    // contains a substring? Boolean
    println!("Contains 'World': {}", hello.contains("World"));

    // replace 'World' with 'There' - non-destructive
    println!("Replace: {}", hello.replace("World", "There"));

    // loop thru string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    // 1st ex tests to see if s is of length 2. If true, nothing happens. If false, throws error
    assert_eq!(2, s.len());
    assert_eq!(11, s.capacity());

    println!("{}", s);

    // println!("{}", hello);
}