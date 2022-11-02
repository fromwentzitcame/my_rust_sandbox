// Arrays - fixed list where elements are the same data types

use std::mem;

pub fn run() {
    // name: [data type; length] = values
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // you can re-assign a value
    numbers[2] = 20;

    // prints the whole array
    println!("{:?}", numbers);

    // print single value
    println!("Single Value: {}", numbers[0]);

    // get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
