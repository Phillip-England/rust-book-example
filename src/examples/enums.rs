#[allow(unused)]
pub fn enums() {
    // creating an enum
    // each enum can have variants
    // each variant can take in different types of data
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // creating an instance of an enum
    // the enums themselves act like constructors to build out their type
    let four: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);
    let six: IpAddrKind = IpAddrKind::V6(String::from("::1"));

    // defining a function which takes in the IpAddrKind type
    fn route(ip_kind: IpAddrKind) {}

    // implementing a method on an enum
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            println!("Calling!")
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    // the option type
    // can be in two starts, either it has Some value or it has None
    let some_number: Option<i32> = Some(5);
    let some_char: Option<char> = Some('e');
    let absent_number: Option<i32> = None;

    // the advantage of option is that it is a different type from the type it contains
    // the following code will cause an error
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;

    // using match with enums to derive values
    enum Coin {
        Penny,
        Nickle,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> u8 {
        // the result of the match statement is returned
        match coin {
            // if an arm has more than a simple return, use curly braces
            Coin::Penny => {
                println!("lucky penny");
                1
            }
            Coin::Nickle => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("This quarter is from {:?}!", state);
                25
            }
        }
    }
    let penny = Coin::Penny;
    let cents: u8 = value_in_cents(penny);
    println!("the coin is worth {cents} cents");

    // combining enums
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // ect..
    }
    let quarter = Coin::Quarter(UsState::Alabama);
    let which_quarter = value_in_cents(quarter);
    println!("{:?}", which_quarter);

    // running match statements with Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        // when using match with enums, you must cover all possible cases or you'll get an error
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?} {:?}", five, six, none);

    // here, we can target specific values in a match, and have general functionality for all others
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // if the value is not 3 or 7, it's other
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(_num_spaces: u8) {}

    // now we can use a catch all to make the player reroll when failing to get 3 or 7
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    fn reroll() {}

    // finally we can make nothing happen if the player rolls anything but 3 or 7
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    // using "if let" to target a specific variant in match while ignoring the rest
    //here is an example where we do this the verbose way
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("the max is configured to be {}", max),
        _ => (), // do nothing if we do not have some value
    }

    // not we can use if let to make this more consise
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("the max config is {}", max)
    }

    // using else in if let
    let mut _count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("state quarter from {:?}", state)
    } else {
        _count += 1;
    }
}
