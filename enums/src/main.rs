enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // -- snip --
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let u8_value = 0u8;
    match u8_value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => (), // default case. Matches must be exhaustive
    }

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("Three");
    }

    let mut coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quater from {:?}!", state),
        _ => count += 1,
    /}

    // or
    //if let Coin::Quarter(state) = coin {
    //    println!("State quater from {:?}!", state);
    //} else {
    //    count += 1;
    //}
}

fn value_in_centers(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}