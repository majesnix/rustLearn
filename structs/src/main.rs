#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Needs to be set for every struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// create a user with the given email and username
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let email = String::from("test@test.de");
    let username = String::from("tester");
    let mut user1 = build_user(email, username);


// acces user1.email prop and set it
    user1.email = String::from("anotheremail@example.com");

// kind of like the spread operator. add all props of user1 to user2, that does NOT exist
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("{:?}", user1);
    // With the # the output is formatted nicely (with line breaks)
    println!("{:#?}", user2);

    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:#?}", rect1);

// {} get replaced with the variable output at the end
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!(
        "Area calculated through a method: {}",
        rect1.area()
    )
}
