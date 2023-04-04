pub fn variables() {
    // in order to change a variables value, we must mark it with mut
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constants cannot have their value changed
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("How many seconds are in three hours?: {THREE_HOURS_IN_SECONDS}");

    // variable shadowing
    let y = 5;
    let y = y + 1;

    // creating scope
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y in the outer scoper is: {y}");

    // getting the length of a string
    let spaces = "     ";
    let spaces = spaces.len();
    println!("How many spaces?: {spaces}");

    println!("Important note: If you try to mutate a variable into a different type, you will get an error")
}
