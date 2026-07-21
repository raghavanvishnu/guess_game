use std::{cmp::Ordering, io}; // standard module

use rand::Rng;// module to generate random numbers


fn main() {// we now have the opener of the function here
println!("Guess the number");
// we now add a second comment here
// sample line for contrast
let secret_number= rand::thread_rng().gen_range(1..=100);// we assign secret no
// random no generator 
// rand contains rng function
println!("Your secret number is : {secret_number}");
// next one
println!("Enter your _guess");
//mut here helps
let mut _guess = String::new(); // we create a mutable variable 
// mutable means it can be altetred

io::stdin()
  .read_line(&mut _guess)// & indicates reference
  //mut indicates its mutable/ changing
  .expect("Failed to read line");

println!("you guessed: {_guess}");// println is a print func/macro

// We now add the code to compare the guess

match _guess.cmp(& secret_number){
Ordering::Less => println!("Too Small"),
Ordering::Equal => println!("You win"),
Ordering:: Greater=> println!("Too Large")

}


}//



