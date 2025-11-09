fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(10, 'o');
    experssion();
    let x = square(5);
    println!("foo return value = {x}");
}

fn another_function(x: i32) {
    println!("Value of {x}");
}

fn print_labeled_measurement(x: i8, c: char) {
    println!("My char {c} is {x} long.")
}   

fn statement() {
    let y = 3;
}

fn experssion() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("value of y is {y}");
}

fn square(x: i32) -> i32 {
    x * x
}