// if expressions
// fn main() {
//   let number = 6;

//   // once you're getting lots of branches match may be better to use
//   if number % 4 == 0 {
//     println!("number is divisible by 4");
//   } else if number % 3 == 0 {
//     println!("number is divisible by 3");
//   } else if number % 2 == 0 {
//     println!("number is divisible by 2");
//   } else {
//     println!("number is not divisible by 4, 3, or 2");
//   }

//   // can use if in let statement but the two options need to be same type (and able to be determined at compile time)
//   let condition = true;
//   let number = if condition { 5 } else { 8 };

//   println!("The value of number is: {}", number);
// }

// LOOOOOPS
// loop - need to break out of these and can optionally return something by putting it after the break
// fn main() {
//   let mut counter = 0;

//   let result = loop {
//     counter += 1;

//     if counter == 10 {
//       break counter * 2;
//     }
//   };

//   println!("The result is {}", result);
// }

//while
// fn main() {
//   let mut number = 3;

//   while number != 0 {
//     println!("{}!", number);

//     number -= 1;
//   }

//   println!("LIFTOFF!!!");
// }

// and for to loop over collections
// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a.iter() {
//         println!("the value is: {}", element);
//     }
// }

// the while example rewritten as a for
// (1..4) is a Range -  a type provided by the standard library that generates all numbers in sequence starting from one number and ending before another number
fn main() {
  for number in (1..4).rev() {
    println!("{}!", number);
  }
  println!("LIFTOFF!!!");
}
