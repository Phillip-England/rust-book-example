pub fn borrowing() {
    // using a reference for a function parameter
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // in order to change areference, you need to declare it mutable
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    let mut s = String::from("hello");
    change(&mut s);
    println!("the value of s is: {s}");

    // you can only have one mut reference to a value at a time
    // the following code causes an error
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // you can create multiple references, as long as they are at different scopes
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("Reference 1: {r1}");
    }
    let r2 = &mut s;
    println!("Reference 2: {r2}");

    // we cannot have a mutable reference while we also have an immutable reference to the same value
    // the following code causes an error
    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    // a reference goes out of scope after it is used (or the last time it is used)
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);

    // a dangling reference is when we have a reference which points to a value that doesn't exist anymore
    // rust prevents dangling reference and will not let us compile if we have them
    //the following code creates a dangling reference and will not compile
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s
    // }
    // let ref_to_nothing = dangle();

    // instead of returning a reference, return the value instead
    fn no_dangle() -> String {
        let s = String::from("hello");
        s
    }
    let the_value = no_dangle();
    println!("The value is: {the_value}");
}
