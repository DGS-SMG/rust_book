use std::io;

fn main() {
    //let mut x: i32 = 5;
    //println!("The value of x is: {x}");
    //x = 6;
    //println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 06 * 3; // namnig convention for constants is all upper case wiht underscores

    let tup: (i32, f64, u8) = (50, 6.4, 1);

    let ( x, y, z) = tup;

    println!("The value of x is: {x}");

    let fifty = tup.0; // very indexed
    let six_point_four = tup.1;
    let one = tup.2;

    println!("Fifty: {fifty}");

    // an empty tuple () are known as a unit


    // Arrays
    // every element of an array must have the same type
    // in Rust, arrays have a fixed length

    let a = [1, 2, 3, 4, 5, 6];


    // Vectors are simiar to arrays, thye are provided by the std library but are allowed to grow/shrink in size
    // Arays are more useul when you know the size will not need to change

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"]; // infered type nad length

    let b : [i32; 5] = [1, 2, 3, 4, 5]; // declared type and length

    let array_one = b[0];
    let array_two = b[1];


    let c = [3; 6]; // value an lenght declared [3, 3, 3, 3, 3, 3]



    println!("Guess the index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Invalid number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");

}
