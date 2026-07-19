use std::io;
// use = bring a name into scope
// std = Rust Standard Library
// io = Input/Output module within the standard library
// io is NOT part of the prelude, so it must be brought into scope explicitly.
// std is similar to an R package (rough analogy).
// use is similar to making a namespace available, not calling library().
// :: is the namespace/path operator, similar to package::function in R.

fn main() {
    // Program execution starts here.
    // Empty parentheses () mean this function takes no parameters.
    println!("Guess The number");
    println!("Please input ypur _guess");

    let mut _guess = String::new();// assigned a mutable variable 
    // this is a variable of type string
    //the staring instance is empty


}