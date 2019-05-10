enum IpAddrKing {
    V4(u8,u8,u8,u8),
    V6(String)
}

// Rusts "null" value, Some can be any kind of Datatype
enum Option<T> {
    Some(T),
    None,
}

let home = IpAddr::V4(127,0,0,1);

let loopback = IpAddrKing::V6(String::from("::1"));

fn main() {
    let four = IpAddrKing::V4;
    let six = IpAddrKing::V6;

// y is 5, x is "none" (NULL)
    let y = Some(5);
    let x: Option<i32> = None;
}
