#[allow(unused)]
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
            email,    // using a shorthand to define email
            sign_in_count: 1,
        }
    }
    let _user2: User = build_user(
        String::from("myemail@example.com"),
        String::from("rustyboy11"),
    );

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

    // example of working without a struct
    let width1: u32 = 30;
    let height1: u32 = 50;
    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
    println!("The area of the rectangle is {} square pixels", {
        area(width1, height1)
    });

    // in the above example, the parameters are not grouped, so we can use a tuple instead
    let rect1: (u32, u32) = (100, 200);
    fn tuple_area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
    println!(
        "the area of the tuple rectangle is {} squared pixels",
        tuple_area(rect1)
    );

    // refactoring to use a struct instead of a tuple
    #[derive(Debug)] // allows us to print out our struct
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let rect2: Rectangle = Rectangle {
        width: 200,
        height: 400,
    };
    // using a ref to Rectangle allows the top level function to maintain ownership after the func call
    fn struct_area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
    println!(
        "the area of the struct rectangle is {} pixels squared",
        struct_area(&rect2)
    );

    // viewing struct details with debug
    // we use a ref so dbg! does not take ownership of rect2
    dbg!(&rect2);

    // implementing the area method on the rectangle struct
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    let area: u32 = rect2.area();
    println!("area using the area method = {area}");

    // you can name methods and fields the same name
    impl Rectangle {
        fn width(&self) -> bool {
            self.width > 0
        }
    }
    let zero_width_rect = Rectangle {
        height: 20,
        width: 10,
    };
    if zero_width_rect.width() {
        println!("This rect has a nonzero width!");
    }

    // adding more parameters to methods
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let small_rect: Rectangle = Rectangle {
        width: 10,
        height: 20,
    };
    let big_rect: Rectangle = Rectangle {
        width: 100,
        height: 200,
    };
    let can_hold: bool = big_rect.can_hold(&small_rect);
    println!("Big rect can hold small rect: {can_hold}");

    // associated methods (method that do not take in &self) are often used to construct a type from a struct
    // associated methods are accessed using "::"
    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
    let some_square: Rectangle = Rectangle::square(10);
}
