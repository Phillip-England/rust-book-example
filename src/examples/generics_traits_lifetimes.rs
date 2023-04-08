use std::fmt::{Debug, Display};

#[allow(unused)]
pub fn generic_traits_lifetimes() {
    // using generics in a struct definition
    struct Point<T> {
        x: T,
        y: T,
    }

    // building the point with different data types
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // but you cannot mix types with our scruct because it only has one type T
    // let wont_work = Point { x: 5, y: 1.2 };

    // if we include multiple types in the definition, we can blend types
    struct AnotherPoint<T, U> {
        x: T,
        y: U,
    }

    // now we can use multiple data types when building AnotherPoint
    let _will_work = AnotherPoint { x: 5, y: 1.2 };

    // lets use generics in a method definition
    // this method will work with ALL Points built with ALL types of data
    // thats because we said impl<T>, which applies to all types
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // now we can use the method here
    println!("p.x = {}", integer.x());

    // now lets write a method which only applies to Points built with floats
    // notice we do not need any <T> after impl because we specificed a concrete type
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    // lets use the method for floats on a float
    println!(
        "the distance from the origin for our float is: {}",
        float.distance_from_origin()
    );

    // but if we try to apply this to a Point build with integers, it will cause a panic
    // the following method call will fail
    // integer.distance_from_origin();

    // the types we defined in our struct definition and our method definitions can be different
    // here is an example where we blend two data types using this concept
    struct MorePoints<X1, Y1> {
        x: X1,
        y: Y1,
    }

    // this method takes a type with X1, and X2, but returns a type with X1, and Y2
    // what you call these Y1s and Y2s does not really matter, we just do this for readability
    impl<X1, Y1> MorePoints<X1, Y1> {
        fn mixup<X2, Y2>(self, other: MorePoints<X2, Y2>) -> MorePoints<X1, Y2> {
            MorePoints {
                x: self.x,
                y: other.y,
            }
        }
    }

    // here we define 3 points and "mix them up" using our mixup method
    let p1 = MorePoints { x: 5, y: 10.6 };
    let p2 = MorePoints { x: "hello", y: 'x' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // defining shared behavior using traits
    // lets create a trait called Summary that has a method called summarize
    pub trait Summary {
        // leaving this without defined functionality will require all types that
        // implement this trait to define their own functionality
        // fn summarize(&self) -> String;

        // but if we define some behaviour here, it will serve as the default implementation
        // and can be called without defining behavior on the actual type which inherits the trait
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }

        // methods within a trait can call other methods within the same trait
        // here is a good example, we create a method called "summarize_reliant_on_author"
        // this method calls a method called "author"
        // so when we build our type, we will rely on author to be defined in that type
        // but summarize_reliant_on_author will run with it's default behavior
        // this makes it so our types only have to define small details, and allow other methods
        // to use their default implementation
        fn summarize_reliant_on_author(&self) -> String {
            format!("(Read more from {})", self.summarize_author())
        }

        // so the above method is using some default implmentation
        // but the below trait will be required to be defined with our types
        fn summarize_author(&self) -> String;
    }

    // lets create some types to implement our trait on
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
        // summarize author is required because it is used in our default implementation for the method summarize
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
        // summarize author is required because it is used in our default implementation for the method summarize
        fn summarize_author(&self) -> String {
            format!("{}", self.username)
        }
    }

    // creating instances of our types and calling the summarize method
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    // lets create a type where we do not define summarize, then we can use the default implementation
    pub struct Post {
        pub author: String,
        pub content: String,
    }
    impl Summary for Post {
        // here we do not define any behaviour for summarize_reliant_on_author, which allows the default behaviour to take authority
        // we also do not define any behaviour for summarize because we do not have to
        // the default implementation for both are already established in our trait definition
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
    }

    let post: Post = Post {
        author: String::from("Bill"),
        content: String::from("I was walking and then.."),
    };
    println!("Using posts default summarize method: {}", post.summarize());
    println!(
        "Using posts default summarize_reliant_on_author method: {}",
        post.summarize_reliant_on_author()
    );

    // we can also use traits as parameters for functions
    // basically, any type with the trait can be passed to the function
    // notice how we are taking a reference to the type with the trait
    // this means when we call the function we must pass a reference of our type which contains the trait
    pub fn notify(item: &impl Summary) {
        println!(
            "Printing from a function which takes a trait as a param! Result = {}",
            item.summarize()
        );
    }
    // here we use the method, and we are sure to pass the reference to tweet
    notify(&tweet);

    // above, we used the impl Trait syntax, but you can also use the trait bound syntax
    // lets rewrite the function call above to use trait bound syntax
    // this type of syntax is preferred if you intend for both items to have the same type
    // using the syntax in notify, we can pass multiple params which implement Summary, but we have no
    // was of ensuring they are of the same type
    // using trait bound syntax, we can be sure item1 and item2 are always the same type
    pub fn notify_twice<T: Summary>(item1: &T, item2: &T) {
        println!("{}", item1.summarize());
        println!("{}", item2.summarize());
    }
    notify_twice(&tweet, &tweet);

    // using multiple trait bounds using the "+" syntax
    // this method only accepts a type which implements Summary and Display traits
    pub fn notify_a_third_time(item: &(impl Summary + Display)) {}

    // we can also do the same thing using trait bound syntax
    // this method also ensures and params implement Summary and Display traits
    pub fn notify_a_fourth_time<T: Summary + Display>(item: &T) {}

    // here is an issue, as we require more traits to be required for a function call,
    // things can get muddy and unreadable
    // so we can use the "where" syntax to make things more readable
    // this helps us clearly see that t must implement Display and Clone
    // and u must implement Clone and Debug
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        2
    }

    // we can also use traits as return types from function
    // the return type here must implement the trait Summary
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    // functions which can return more than one type with the implemented trait wont work
    // if you have a function which returns a trait
    // you have to make sure only one possible type with that trait can be returned from the function
    // below we use a switch statement to return 2 potentials types which implement Summary
    // and this will not work because the function must only be able to return one type
    // which implements summary
    // fn returns_summarizable_again(switch: bool) -> impl Summary {
    //     if switch {
    //         NewsArticle {
    //             headline: String::from(
    //                 "Penguins win the Stanley Cup Championship!",
    //             ),
    //             location: String::from("Pittsburgh, PA, USA"),
    //             author: String::from("Iceburgh"),
    //             content: String::from(
    //                 "The Pittsburgh Penguins once again are the best \
    //                  hockey team in the NHL.",
    //             ),
    //         }
    //     } else {
    //         Tweet {
    //             username: String::from("horse_ebooks"),
    //             content: String::from(
    //                 "of course, as you probably already know, people",
    //             ),
    //             reply: false,
    //             retweet: false,
    //         }
    //     }
    // }

    // okay, here is another thing we can do
    // lets say we want to define a method on a type,
    // but we do not want all types built from the struct to implement all methods
    // we can conditionally allow methods to exist on a type by using trait bounds
    // here is an example

    // first we create a type called Pair
    struct Pair<T> {
        x: T,
        y: T,
    }

    // now we implement a method on pair which every one can use
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // now we implement a method which has a restriction
    // these methods can be called only if the created type has implemented Display and PartialOrd
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("the largest member is x = {}", self.x);
            } else {
                println!("the largest member is y = {}", self.y);
            }
        }
    }

    // okay, lets say we want to do something like this
    // say you have a trait called Display
    // and you want to say:
    // "for every type that implements Display, I also want to implement ToString"
    // this is called blanket implementation
    // for example, in the rust standard lib, every type which implements Display also implements ToString
    // this allows for easy method sharing amongst types

    // lifetimes ensure that a reference is valid as long as we need them to be
    // lifetimes exist to prevent dangling references

    // here is an example where we can use lifetimes
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("the longest string is {}", result);

    // if we implements the longest function as follows, we will get an error about lifetimes
    // we have no idea if x or y will be returned from the function
    // we also have no way of knowing the context in which this function will be called
    // so we must specify lifetimes
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    // lets rewrite the function using lifetimes
    // now the function will work!
    // this function now tells rust the the references pass into the function must live as long
    // as the values from which they were derived from
    // so if we do no adhere to this rule, the compiler will throw an error
    // these annocations create a constraint on the function
    // and gatekeep any values which might be passed ins
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // lets try to challenge the constraint by using a value whos lifetime does not last long enough
    let s1 = String::from("this is longer than the other");
    // let result;
    {
        let s2 = String::from("shorter");
        // here, s2 is borrowed, but it's lifetime ends at the end of this scope
        // result = longest(s1.as_str(), s2.as_str());
    }
    // we now try to print out the value of result, but s2 is now invalid
    // so the borrow checker will complain and let us know s2 dies before we try and use result
    // these constaints are derived from the lifetimes we specified in the longest function definition
    // println!("the longest string is {}", result);

    // we can also use reference in struct definitions, but to do so, we must speficy the lifetimes
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("call me jon. some years ago...");
    // this returns a reference
    let first_sentance = novel.split('.').next().expect("could not find a .");
    // now we are attempting to use a reference in our type
    // so if we failed to specify the lifetime in the struct definition, this would fail
    // the lifetime helps to ensure first_sentance lives long enough to be used in this data type
    // the data type we create here, cannot out live first_sentance, as the type is dependant on it
    let it = ImportantExcerpt {
        part: first_sentance,
    };

    // basiaclly, any function, method, or struct which takes hold of a reference, must annotate the lifetimes of that reference
    // this tells the compiler to ensure that everything is in the proper scope and that all attemps to access a ref are valid

    // rust has some specific cases where lifetimes are not required
    // basically, rust has some rules typed in which identify patterns in written code
    // code where lifetimes are frequently used without any sort of risk have been identified
    // these types of cases now do not require lifetime checks
    // because they have been proven to not be a risk to memory management
    // less lifetimes will be required overtime as more of these cases are identified

    // THE THREE RULES OF LIFETIMES
    // rust has some rules where it tries to figure out the lifetimes of the parameters, and return types of functions
    // #1 = all params to functions get their own lifetime
    // #2 = if only 1 param is listed, all output will match the params lifetime
    // #3 = if one param is &self, then the lifetime of self is applied to all inputs and all outputs

    // the above rules are why we needed to specify the lifetimes in the longest function
    // we had two params, so rule number one gave them each their own lifetime
    // rule 2 and rule 3 did not apply, so rust had no way of knowing the outputs lifetime
    // so, we had to manually apply the lifetime and tell rust that hey, all of these params
    // are of lifetime 'a, and that the output was also of lifetime 'a
    // rust will not assume the lifetime of the output if it cannot do so using the rules above

    // lets use lifetimes in method definitions
    impl<'a> ImportantExcerpt<'a> {
        // the lifetime to self is not required becuase of rule 3 of the lifetime elision rules
        fn level(&self) -> i32 {
            3
        }
        // here is an example of where we can omit the lifetime of the return value because of rule 3
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("attention please: {}", announcement);
            self.part
        }
    }

    // lets discuss static lifetimes
    // static lifetimes are stored directly in a programs binary and are always availble
    let static_string: &'static str = "I have a static lifetime";
    println!("{static_string}");

    // an example of a function which uses trait bounds, lifetimes, and generics all in one
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
