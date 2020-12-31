use std::io; // import stuff that isn't in the prelude

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // vars are immutable by default
    let mut guess = String::new(); // :: is calling the new method on string type like static method

    io::stdin()
        .read_line(&mut guess) // & = as ref, refs are immutable but we want mutable so input can be added
        // read_line returns an io::Result (Ok or Err) and has an expect method...which can print errors (and will otherwise return the return value held by Ok) - proper error handling this is not!
        .expect("Failed to read line"); 

    println!("You guessed: {}", guess);
}
