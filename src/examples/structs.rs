pub fn structs() {

    // creating a struct
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64, 
    }

    // using a struct to create a value
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // changing a mutable struct
    user1.email = String::from("anotheremail@example.com");
    user1.username = String::from("anotherusername");
    user1.active = false;
    user1.sign_in_count = 2;

    // creating a builder function to create a user
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username, // using a shorthand to define username
            email, // using a shorthand to define email
            sign_in_count: 1,
        }
    }
    let _user2: User = build_user(String::from("myemail@example.com"), String::from("rustyboy11"));

    // building a new struct from other structs
    // this pulls all values from user1 except the email and copies them into user3
    let _user3 = User {
        email: String::from("anotheremail@example.com"),
        ..user1 // must come last
    };

    // using tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // unit structs
    // these types of struct have no data and behave like the unit type "()"
    // these are useful when you want to implement a trait on a type, but you dont have any data you want to store
    struct AlwaysEqual;
    let _subject = AlwaysEqual;

    // struct values can be reference, but you need to speicify a lifetime in order to do so
    // see /examples/lifetimes.rs for more information

}