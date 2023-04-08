
#[cfg(test)]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
#[allow(unused)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(unused)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


// checking equality
#[test]
fn exploration() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

// checking for true
#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 4,
        height: 1,
    };
    assert!(larger.can_hold(&smaller))
}

// checking for false
#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 4,
        height: 1,
    };
    assert!(!smaller.can_hold(&larger))
}

#[allow(unused)]
fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

#[test]
fn greeting_test() {
    let result = greeting("Carol");
    assert!(result.contains("Carol"));
}

// custom error messages for failed tests
#[test]
fn failed_with_custom_error() {
    let result = greeting("Bob");
    assert!(result.contains("Jon"), "Greeting did not contain name, value was {}", result);
}


// testing for expected panics
#[allow(unused)]
pub struct Guess {
    value: i32,
}

#[allow(unused)]
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("guest must be between 1 and 100, got {}", value);
        }
        Guess { value: value }
    }
}

/// this test will pass because it in fact should panic
#[test]
// the below code will pass even if the panic was not the type we expect
// #[should_panic]
// to resolve this, we can pass a unique panic substring
// the substring provided should exist in the panics substring
#[should_panic(expected = "must be between 1 and 100")]
fn greater_than_100() {
    Guess::new(200);
}

// we can also write tests as a result type
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 5 {
        Ok(())
    } else {
        Err(String::from("your math sucks!"))
    }
}

// if a test passes, all stdoutput will not be printed
// run cargo test -- --show-output to see the stdoutput on passed tests

// tests run in multiple threads, so if one test depends on the results of another
// you can run in a single thread by using cargo -- --test-threads=1

// if you want to only test a specific function, you can call the test by name
// cargo test <function-name>

// you can group tests by module, and then run all tests in a module by using
// cargo test <mod-name>

// the following test will be ignored
// we can run ignored tests by using cargo test -- --ignored
// to run all tests including ignored tests run cargo test -- --include-ignored
#[test]
#[ignore]
fn ignore_me() {
    assert_eq!(1, 1);
}



