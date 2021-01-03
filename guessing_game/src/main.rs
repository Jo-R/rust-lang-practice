use rand::Rng; // Rng is trait which is kinda like an interface (allows use of gen_range below)
use std::cmp::Ordering;
use std::io; // import stuff that isn't in the prelude

fn main() {
  println!("Guess the number!"); // ! means its a macro not a function

  let secret_number = rand::thread_rng().gen_range(1, 101);
  // println!("The secret number is: {}", secret_number);

  loop {
    println!("Please input your guess.");

    // vars are immutable by default
    let mut guess = String::new(); // :: is calling the new method on string type like static method

    io::stdin()
      .read_line(&mut guess) // & = as ref, refs are immutable but we want mutable so input can be added
      // read_line returns an io::Result (Ok or Err) and has an expect method...which can print errors (and will otherwise return the return value held by Ok) - proper error handling this is not!
      .expect("Failed to read line");

    // using shadowing we can convert the guess var to a number
    // trim eliminates the /n character from pressing enter on stdin.read_line
    // had this originally:
    // let guess: u32 = guess.trim().parse().expect("Please type a number!");
    // use match to improve error handling
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("You guessed: {}", guess);

    //match epxression - arms (a pattern and the code that should be run if match that pattern)
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break; // exits the prog as exits the loop
      }
    }
  }
}
