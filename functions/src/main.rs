fn five() -> i32 {
    // Because the 5 has no ;, it is returned
    5
}

fn main() {
    let _x = 5;

    let y = {
        let x = 3;
        // if no ; is supplied, it is an expression and will return the value
        // if a ; is supplied, the expression will become an statement, which will not return a value
        x + 1
    };

    println!("The value of y is: {}", y);

    let return_five = five();

    println!("the value of return_five is {}", return_five);
}
