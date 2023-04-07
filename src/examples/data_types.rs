use std::io;

pub fn data_types() {
    // unused variables can be preceeded by "_" to prevent compiler from complaining

    // if we do not put a type here, the compiler will ask for one
    let _guess: u32 = "42".parse().expect("Not a number!");

    // floating point numbers
    let _x: f64 = 2.0;
    let _y: f32 = 3.0;

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    // booleans
    let _t: bool = true;
    let _f: bool = false;

    // characters
    let _c: char = 'z';

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destruturing a tuple (extracting the values into their own variables)
    let (x, y, z) = tup;
    println!("The tuple contains: {x}, {y}, and {z}");

    // accessing a tuple with dot notation
    println!(
        "Here is how to access the tuple using dot notation: {}, {}, and {}",
        { tup.0 },
        { tup.1 },
        { tup.2 }
    );

    // "the unit"
    // if a function or expression does not return a value, it automatically returns a unit type instead
    let _unit = ();

    // arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // must specify the types in the array and the length

    // accessing arrays
    let first = a[0];
    println!("{first}");

    // arrays are useful when we know our data won't change
    let _months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // shorthand init for arrays
    let arr: [i32; 5] = [3; 5]; // [3, 3, 3, 3, 3]

    // trying to access an array index which doesn't exist will cause a panic
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = arr[index];
    println!("The value of the element at index {index} is: {element}");
}
