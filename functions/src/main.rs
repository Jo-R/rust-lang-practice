// fn main() {
//   println!("Hello, world!");

//   another_function(23, 11);
// }

// snake case
// could define before or after main, doesn't matter
// parameters MUST be typed
// fn another_function(x: i32, y: i32) {
//   println!("The value of x is: {}", x);
//   println!("The value of y is: {}", y);
// }

// "Function bodies are made up of a series of statements optionally ending in an expression."
// "Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. "
//  Calling a function is an expression. Calling a macro is an expression. The block that we use to create new scopes, {}, is an expression,

// #![allow(unused)]
// fn main() {
//   let x = 5; // this line is a statement - 5 is an expression (it evaluates to 5)

//   let y = {
//     // this block is an expression
//     let x = 3;
//     x + 1 // NO SEMI COLON is an expression and returns a value if there isn't one
//   };

//   println!("The value of y is: {}", y);
// }

// -> and type for return
// most function return implictly in rust (like the one below) but can use return keyword to rtn early
fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}

fn main() {
  let x = five();

  println!("The value of x is: {}", x);

  let y = plus_one(5);

  println!("The value of y is: {}", y);
}
