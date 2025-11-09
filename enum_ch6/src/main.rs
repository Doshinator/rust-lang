enum IpAddr {
    // We attach data to each variant of the enum directly, no need for struct to hold ip addr kind and the address
    V4(String),
    V6(String),
}

/**
 * You can put any piece of data inside enum
 */
struct IpV4 {
    ipAddr: String,
}

struct IpV6 {
    ipAddr: String,
}

enum IpAddrEnum {
    V4(IpV4), // now it contains of type IpV4
    V6(IpV6),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
       // self match Message::Write(text: String) { print }, ();
    }
}

/**
 * enum options
 */
// enum Option<T> {
//     None,
//     Some(T),
// }

/**
 * match control flow
 */

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter (UsState),
    Dollar,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter's state: {state:?}");
            25
        },
        Coin::Dollar => 100,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn count_coin(coin: Coin) {
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }
    println!("{count}");
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let local = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let a = Option::Some('a');
    let five = Option::Some(5);

    let absent_number: Option<i32> = Option::None;
    let absent_char: Option<char> = Option::None;
    let absent_string: Option<String> = Option::None;

    value_in_cents(Coin::Quarter(UsState::Alabama));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    let dice_roll = 255;
    match dice_roll {
        3 => put_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // () means nothing happens
    }

    let config_max: Option<u8> = Some(5);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }


}



fn put_fancy_hat() {
    println!("Putting on fancy hat");
}

fn remove_fancy_hat() {
    println!("Removing fancy hat");
}


impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

fn describe_state(coin: Coin) -> Option<String> {
    // if let Coin::Quarter(state) = coin {
    //     if state.existed_in(1900) {
    //         Some(format!("{state:?} is pretty old, for America!"))
    //     }
    //     else {
    //         Some(format!("{state:?} is relatively new."))
    //     }
    // } else {
    //     None
    // }

    // transforms into this.
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}   