// Vectors - resizable arrays

use std::mem;

pub fn run() {
    // name: Vec<data type> = vec![values]
    let mut numbers: Vec<i32> = vec![1, 2, 20, 4];

    // you can re-assign a value
    numbers[2] = 3;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    // prints the whole vector
    println!("{:?}", numbers);

    // print single value
    println!("Single Value: {}", numbers[0]);

    // get vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values; similar to .map() in JS
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec Mutated: {:?}", numbers);
}
