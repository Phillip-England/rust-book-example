use std::io::Read;
use std::{
    fs::File,
    io::{self, ErrorKind},
};

#[allow(unused)]
pub fn errors() {
    // manually make the program crash
    // panic!("crash and burn");

    // panic occured due to unrecoverable error
    let _v = vec![1, 2, 3];
    // _v[99]; // accessing a value which doesnt exist causes panic

    // the following function returns a Result<T, E>
    // we can handle this type with a match statement to extract the value
    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     // if we find the file, simply return it
    //     Ok(file) => file,
    //     // if we fail to find the element, define custom error handling behavior
    //     Err(error) => panic!("Problem opening the file: {:?}", error)
    // };

    // we can take this a step further by defining customer behavior for all kinds of errors
    // a lot of functions can return multiple different error types which are predefined
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("problem opening the file: {:?}", other_error);
            }
        },
    };

    // using unwrap to shortcut error when we want a default panic
    // if the file is not found, a panic will be called
    let greeting_file = File::open("hello.txt").unwrap();

    // if we want to define a custom panic message, use expect
    // let goodbye_file: File = File::open("bye.txt")
    //     .expect("bye.txt should be included in this project");

    // propagating errors (create your own Result<T, E> and pass it to where your function is called)
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    let _function_result = read_username_from_file().unwrap();

    // shorthand syntax for propagating errors using "?"
    fn read_username_from_another_file() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
    let _function_result: String = read_username_from_another_file().unwrap();

    // the ? operator can only be used in function which have a return type that matches the potential error type
    // the following code fails to run because we have defined the return type
    // let greeting_file = File::open("hello.txt")?;
}
