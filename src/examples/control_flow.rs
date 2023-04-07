pub fn control_flow() {
    // if statements
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // you cannot use values as booleans unless they are specifically bool
    // this code causes an error
    // let number = 3;
    // if number {
    //     println!("number was three");
    // }

    // using else if
    // important note: Rust only finds the first block which is true, and then ignores the rest
    // so multiple conditions can be true, but only the first will run
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // both outcomes must be of the same type, otherwise the compiller will error
    // let condition = true;
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {number}");

    // returning values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // value is returned here (can omit semicolon or include)
        }
    };
    println!("The result is {result}");

    // loops within loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");

    // counting down with a while loop
    let mut number = 3000;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!");

    // looping through a collection with for
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    // counting down using a range
    for number in (1..1000).rev() {
        println!("{number}");
    }
    println!("LIIFTOFF.. AGAIN!!!");
}
