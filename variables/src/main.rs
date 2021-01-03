// fn main() {
// variables are immutable by default - mut keyword
//   let mut x = 5;
//   println!("The value of x is: {}", x);
//   x = 6;
//   println!("The value of x is: {}", x);
// }

// there's also const CAPS_LIKE_THIS but these have to be result of expression, not determined at RT (i.e. not result of fx call etc) - must have type declared

fn main() {
  // shadowing
  //   Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

  // The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
  let x = 5;

  let x = x + 1;

  let x = x * 2;

  println!("The value of x is: {}", x);
}

// can use _ as a number separator 10_000 (10,000)

// TYPES
// all these are stored on the stack
//SCALAR TYPES
// A scalar type represents a single value. Rust has four primary scalar types:
// integers (i is signed, u unsigned 8-128 bits - so eg u64 - plus size which is 64 bits on 64 bit architecture - default is i32 - size is good for indexing collections),
// floating-point numbers, (f32 single precision, f64 double precision - latter is default)
// Booleans (bool),
// and characters (four bytes in size and represents a Unicode Scalar Value)
// fn main() {
//     let c = 'z';
//     let z = 'ℤ';
//     let heart_eyed_cat = '😻';
// }
// COMPOUND TYPES
// Group multiple values and Rust has Two: tuples and arrays
// tuples...
//  let tup = (500, 6.4, 1); - can be different types but is fixed size once decalred
// let (x, y, z) = tup; ie destruture or can do tup.0 etc like array access
// println!("The value of y is: {}", y);
// arrays...
// must all be same type and also have fixed length (can use Vectors if want variable length)
// allocated on stack not heap
// unlike some low level langs Rust will panic if try to access an array index that doesn't exist
