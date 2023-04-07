pub fn functions() {
    fn another_function(x: i32, unit_label: char) {
        println!("another function!");
        println!("The value of x is: {x}{unit_label}");
    }

    another_function(10, 'h');

    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value. Let’s look at some examples.

    // example of expression retuning a value
    let y = {
        let x = 3;
        x + 1 // this value does not include a semicolon because it is returned from the experssion
    };
    println!("The value of y is: {y}");

    // return values in functions
    fn five() -> i32 {
        5
    }
    let x = five();
    println!("The value of x is: {x}");

    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    // shadowing x
    let x = plus_one(2);
    println!("The value of x is: {x}");
}
