fn main() {
    let x = 10;
    if x < 5 {
        println!("x is less than 7; condition branch true")
    }
    else {
        println!("x is greater than or equals to 7; condition branch false")
    }


    if x == 10 {
        println!("the condition is true; x == 10")
    }

    if x != 7 {
        println!("This condition is true. x is NOT equals to 7")
    }

    let number_is_odd = if x % 2 != 0 { x } else { -1 };

    println!("the number is odd? {number_is_odd}");

    loops();
    while_loop();
    for_loop();
}

fn loops() {
    let mut x = 5;
    let val = loop {
        println!("{x}");
        x = x - 1;
        if x < 0 {
            break x; // return the value AFTER break expression if you want to return it
        }
    };

    println!("Returned val from loop: {val}");
    let mut count = 0;
    'loop_label: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'loop_label; // this is what is important in this; loop label to break out the outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
}

fn while_loop() {
    let mut number = 0;
    while number != 10 {
        println!(" Number: {number}");
        number += 1;
    }

}

fn for_loop() {
    let a = [1, 2, 5, 3, 9, 10, 12, 15, 22];
    for element in a {
        println!("value of element is {element}");
    }

}
