pub fn collections() {
    // empty vector
    let _v: Vec<i32> = Vec::new();

    // vector with values
    let mut v: Vec<i32> = vec![1, 2, 3];

    // pushing values into a vector
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // viewing the vector
    dbg!(&v);

    // you can access the vector using get or the [] syntax
    // using [] is straighforward but more error prone
    let third: &i32 = &v[2];
    println!("the third element is {third}");

    // using .get requires us to extract the value from Option
    // this is a safer option if you are not sure the element will exist at runtime
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("the third element is still {third}"),
        None => println!("there is no third element!"),
    }

    // the borrow checker prevents us from adding items to a vector if a reference to it is held
    // the following code will cause an error
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("the first element is {first}");

    // iterating through a vector
    let v = vec![100, 200, 300];
    for i in &v {
        println!("{i}");
    }

    // making changes to all items in a vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    dbg!(&v);

    // vectors can only hold values of the same type
    // if you need to have multiple types in a vector, use an enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // even though the enum contains different variants, they are all still considered the same type
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // creating an empty string
    let mut _s = String::new();

    // converting string literals into a String
    let data = "initial contents";
    let _s = data.to_string();

    // you can also call this method on the literal itself
    let _s = "inital contents".to_string();

    // strings can change in size
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("I grew: {s}");

    // push takes a single char and adds it to the string
    let mut s = String::from("lo");
    s.push('l');
    println!("haha: {s}");

    // string concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}");

    // using format! to combine strings is easier
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("formatted string: {s}");

    // rust does not support string indexing like other languages
    // the following code throws an error
    // let s1 = String::from("hello");
    // let h = s1[0];

    // not every character contains the same amount of bytes
    // take the following example
    let hello = "Здравствуйте"; // each character here is 2 bytes each
    let s = &hello[0..4]; // dont assume this gets you the first 4 characters it gets the first 4 bytes
    println!("I only contain 2 charaters! {s}");

    // iterating over a string as characters
    for c in "asdasdasdaa".chars() {
        println!("{c}");
    }

    // iterating over a string as bytes
    for b in "adasdasdas".bytes() {
        println!("{b}");
    }

    // creating an empty hash map
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // accessing the hash map
    let team_name = String::from("Blue");
    // below, unwrap or sets the score to 0 if the hash map doesnt have a key for the provided value
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("blue team score is: {score}");

    // iterating through a hash map
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // hash maps take ownership of values
    // below, field_name and field_value are now invalid
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // if we pass references to a hash map, the references must exist as long as the map does

    // overwriting a value in a hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // checking if a hashmap has key, if it does, do nothing, if it doesn't, add a value to the key
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // updating a value in a hashmap based on a previous value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
