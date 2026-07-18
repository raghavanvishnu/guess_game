use std::io; // use stnadard librarys input output - this is a module
// standard is preloaded
use rand::Rng;//for gnerating random numbers


fn main() {// we now have the opener of teh function here

  // we now print teh firts line
  // and start from here
println!("Guess the number");
// we now add a second comment here
println!("Enter your guess");
// we now proceed further

// sample line for contrast

let apples = 5;
// we oprint
println!("Apples = {apples}");

//mut here helps
let mut _guess = String::new(); // we create a mutble variable 
// mutable means it can be altetred
// in contrast a 

io::stdin()
  .read_line(&mut _guess)// & indicates reference
  //mut indicates its mutyable/ changing
  .expect("Failed to read line");

println!("you guessed: {_guess}");// println is a print func/macro
}//



