// io => input/output
use std::io;
// compare somtething
use std::cmp::Ordering;
// Random numbers
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // stdin, almost the same as in c++
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // parse given string to a number
        // using the same variable name as above is called shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // Error Handling, Err is thrown, when the match failes
            Err(_) => {
                println!("This is not a number, try again!");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}