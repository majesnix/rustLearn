fn main() {
    let s = String::from("hello");  // s comes into scope
    reference(&s);

    let mut t = takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
                                    // it returns the value to t, which is valid here
    
    println!("Pre reference changed t: {}", t);
    mutable_reference(&mut t);      // t need to be mutable to be reference changed/modified

    println!("{}", t);

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) -> String { // some_string comes into scope
    println!("{}", some_string);
    some_string
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn reference(some_string: &String) {
    println!("Hey, i got a reference of {}", some_string);
}

fn mutable_reference(some_string: &mut String) { // With &mut a referenced value can be changed!
    some_string.push_str(", world");
}