// Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses
// ownership rules :
// 1 Each value in Rust has a variable that’s called its owner.
// 2 There can only be one owner at a time.
// 3 When the owner goes out of scope, the value will be dropped.

// String type is heap allocated and can store text unknown at compile time (like the input in guessing game)
// fn main() {
//   {
//     // can be mutated unlike a string literal
//     // literals are known size at compile time so fast and efficient (they go on the stack)
//     // String is dynamic and needs heap allocation (request memory at RT - get a pointer that
//     // can be stored on the stack that points to the value - and return the memory when done)
//     let mut s = String::from("hello"); // String::from requests the memory
//     s.push_str(", world!"); // push_str() appends a literal to a String

//     println!("{}", s);
//   } // we return the memory allocation when s goes out of scope - Rust calls drop
// }

// fn main() {
//   // this works.. we have two known, fixed-size values (5) which are pushed onto the stack
//   // note the value of x is COPIED to y
//   // what allows this is the Copy trait which ints have (as do the data types mentioned in thats ection - the stack ones) (and Copy must not have Drop trait)
//   let x = 5;
//   let y = x;
//   println!("{} {}", x, y);

//   // this doesn't work...
//   // when the String is copied we actually only copy the pointer that is on the stack
//   // so there's only one string on the heap
//   // but when s1 and s2 go out of scope, rust would try to clean up the same memory location twice
//   // (a double free error) which could lead to memory corruption
//   // so waht rust does is MOVE s1 into s2 and s1 is no longer valid (so this kind of thing is inexpensive)
//   let s1 = String::from("hello");
//   let s2 = s1;
//   // println!("{} {}", s1, s2); compiler error on s1

//   // if we want to actually copy the heap data (deep copy essentially) we use .clone (which could be expensive)
//   let s3 = String::from("hello");
//   let s4 = s3.clone();
//   println!("s3 = {}, s4 = {}", s3, s4);
// }

//FUNCTIONS
// fn main() {
//   let s = String::from("hello"); // s comes into scope

//   takes_ownership(s); // s's value moves into the function...
//                       // ... and so is no longer valid here
//                       // println!("{}", s);

//   let x = 5; // x comes into scope

//   makes_copy(x); // x would move into the function,
//                  // but i32 is Copy, so it’s okay to still
//                  // use x afterward
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) {
//   // some_string comes into scope
//   println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) {
//   // some_integer comes into scope
//   println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.

// RETURN VALUES AND SCOPE
// fn main() {
//   let s1 = gives_ownership(); // gives_ownership moves its return
//                               // value into s1

//   let s2 = String::from("hello"); // s2 comes into scope

//   let s3 = takes_and_gives_back(s2); // s2 is moved into
//                                      // takes_and_gives_back, which also
//                                      // moves its return value into s3
//                                      //  println!("{} {} {}", s1, s2, s3);

//   // so we want the length of a string but it's kinda a pita that passing the string in
//   // means the fx takes ownership so we have to use a tuple to return both - which is also a pita
//   // so.. rust has REFERENCES to help with this...
//   let s4 = String::from("hello");
//   let (s5, len) = calculate_length(s4);
//   println!("The length of '{}' is {}. And {} is gone", s5, len, s4);
// } // Here, s3/5 goes out of scope and is dropped. s2/4 goes out of scope but was
//   // moved, so nothing happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {
//   // gives_ownership will move its
//   // return value into the function
//   // that calls it

//   let some_string = String::from("hello"); // some_string comes into scope

//   some_string // some_string is returned and
//               // moves out to the calling
//               // function
// }

// // takes_and_gives_back will take a String and return one
// fn takes_and_gives_back(a_string: String) -> String {
//   // a_string comes into
//   // scope

//   a_string // a_string is returned and moves out to the calling function
// }

// fn calculate_length(s: String) -> (String, usize) {
//   let length = s.len(); // len() returns the length of a String

//   (s, length)
// }

// REFERENCES AND BORROWING
// so instead of the pita of returning a tuple...have a reference (&) as a param instead of taking ownership
// just BORROW it
// (note dereferencing (*) also exists but not to worry about that yet)
// fn main() {
//   let s1 = String::from("hello");

//   let len = calculate_length(&s1);

//   println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//   // s is a ref to a string
//   s.len()
// } // s goes out of scope but doesn't own the value it refs to so nowt happens

// you can't modify something you borrow (so refs are immutable by default, just like variables)
// fn main() {
//   let s = String::from("hello");

//   change(&s);
// }

// fn change(some_string: &String) {
//   some_string.push_str(", world");
// }

// you can make refs mutable tho...
// fn main() {
//   let mut s = String::from("hello");
//   change(&mut s);
// was fiddling with this a god knows how you print the changed variable haha!

//   // only one mut ref to a pice of data in a single scope
//   // controlled mutation to prevent data races
//   let mut s2 = String::from("hello");
//   let r1 = &mut s2;
//   let r2 = &mut s2;
//   println!("{}, {}", r1, r2);
// }

// fn change(some_string: &mut String) {
//   some_string.push_str(", world");
// }

// "Even though borrowing errors may be frustrating at times, remember that it’s the Rust compiler pointing out a potential bug early (at compile time rather than at runtime) and showing you exactly where the problem is."

// dangling refs
// fn main() {
//   let reference_to_nothing = dangle();

//   let string = no_dangle();
// }
// // the crucial bit of the error here is "this function's return type contains a borrowed value, but there is no value for it to be borrowed from.""
// fn dangle() -> &String {
//   // so, return a ref to a string
//   let s = String::from("hello"); // s the string is created

//   &s // rtn the ref to the string
// } // s goes out of scope so is dropped

// fn no_dangle() -> String {
//   let s = String::from("hello");

//   s
// }
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.
