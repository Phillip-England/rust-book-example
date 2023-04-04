pub fn slices() {

    // refering to part of a string
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..11];
    let whole = &s[..];
    println!("slice1 = {hello} slice2 = {world} slice3 = {whole}");

    // getting the first word of a string
    // we make the parameter s of type &str so we can use both strings and &str types
    // if we want to pass a string in, simply pass it as a slice like &s[..]
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        // if we find a space, return a slice from the start of the string until the index of the space
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        // if we do not find a space, return the whole string as it is a single word
        &s[..]
    }
    let s = String::from("This is many words!");
    let first = first_word(&s);
    println!("the first word is: {first}");
    
    // slices are tied to the original string they are derived from
    // if the original string goes out of scope, the slice becomes invalid
    // s.clear();
    // println!("this is not valid: {first}");

    // slices of arrays
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]); // setting a test, if the values are not equal, an error will be thrown


}