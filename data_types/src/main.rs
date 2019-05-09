fn main() {
    // tuples have a fixed length, and can be of different data types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructing the tupel
    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);
    println!("it can be acces with tup.0 too {}", tup.0);

    // Every element of an ARRAY must have the SAME type
    // array length and type can be predefined
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Access arrays with a[0], {}", a[0]);
}
