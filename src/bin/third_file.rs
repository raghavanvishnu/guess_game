//use core::num;
use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
println!("This is a guessing game");

let _secret_number= rand::thread_rng().gen_range(1..=100);

//println!("the secret number is : {_secret_number}");

loop{
println!("Enter the number you guessed");

let mut _guess= String::new();// defines an mutable value/ variable
io::stdin()
.read_line(&mut _guess)// a reference- and a mutable reference
.expect("Failed to read_line");

let _guess:u32 = match _guess.trim().parse(){
Ok(num)=>num,
Err(_) => continue,
};

println!( "You guessed : {_guess}");

match _guess.cmp(&_secret_number){

   Ordering::Less => println!("too small"),
   Ordering::Greater => println!("too big"),
   Ordering::Equal => {println!("You win");
break;// we break the loop if guess is correct

   }
}
    }   
        }