const MAX_POINTS: u32 = 100_000;

fn main() {
    // If a variable is not mutable, it cannot be assigned twice
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Just printing the constant
    println!("MAX points {}", MAX_POINTS);

    // Here we are SHADOWING a variable, it will throw an compile-time error,
    // when trying to reassing without using the let keyword
    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);

    // Shadowing allows to reassing the variable with different types
    let spaces = "   ";
    let spaces = spaces.len();

    // this would NOT compile, because it would be assinged to a different type
    let mut spaces = "   ";
    spaces = spaces.len();
    
    println!("spaces: {}", spaces);
}