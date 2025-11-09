use std::thread;

use closure::{Inventory, ShirtColor};

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Blue]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let give_away1 = Inventory::giveaway(&store, user_pref1);

    println!(
        "User with preference {:?} gets t-shirt of color {:?}",
        user_pref1,
        give_away1);    

    let user_pref2 = None;
    let give_away2 = Inventory::giveaway(&store, user_pref2);
    println!(
        "User with preference {:?} gets t-shirt of color {:?}",
        user_pref2,
        give_away2); 


    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
