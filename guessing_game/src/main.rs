use std::{cmp::Ordering, io::{self}};
use rand::Rng;


fn guess_num(expected: u32, actual: u32) -> bool {
    expected == actual
}

fn main() {
    println!("----- Number Guessing Game -----");
    println!("Guess a number between 0-100.");

    let rand_num = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        
        println!("You guessed: {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
   
        match guess.cmp(&rand_num) {
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
        
    println!("Random number: {rand_num}")

}
