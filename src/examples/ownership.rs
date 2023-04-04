pub fn ownership() {

    // ownership rules
    // 1. Each value in Rust has an owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // mutating a string
    let mut s: String = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    // this code errors because we are trying to access a moved value
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);

    // cloning a value
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // functions take ownership of parameters
    let s = String::from("hello");
    fn takes_ownership(some_string: String) {
        println!("This string is mine: {some_string}");
    }
    takes_ownership(s);
    // this will error because s does not exist in this scope any longer
    // println!("{s}");

    // this function returns the original value and places it back into "s"
    let mut s = String::from("hello");
    fn returns_ownership(some_string: String) -> String {
        println!("Okay, you can have this string back: {some_string}");
        some_string
    }
    s = returns_ownership(s);
    println!("{s}");

    // retuning multiple values with a tuple
    let mut x = 2;
    let mut y = 4;
    fn double_two_numbers(mut num1: i32, mut num2: i32) -> (i32, i32) {
        num1 = num1 * 2;
        num2 = num2 * 2;
        (num1, num2)
    }
    (x, y) = double_two_numbers(x, y);
    println!("x = {x} y = {y}");



}