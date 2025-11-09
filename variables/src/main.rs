use std::io;

fn main() {
   
    mut_values();
    const_values();
    shadow_value();
    data_types();
    functions();
}


fn mut_values() {
    println!("------ mut value() ------");
    let mut x = 5;
    println!("Value of x {x}");
    x = 6;
    println!("Value of y {x}");
}

fn const_values() {
    println!("------ const value() ------");
    const PI: f32 = 3.14159265358;
    println!("PI: {PI}");
}

fn shadow_value() {
    println!("------ shadow value() ------");
    let x = 5;
    let x = x + 5;

    {
        let x = x * 2;
        println!("Value of x inside a scope: {x}");
    }

    println!("Value of x outside of scope: {x}");
}

fn data_types() {
    println!("------ data_types() ------");

    // addition
    let sum = 5 + 10;

    // subtraction
    let subtraction = 10 - 5;

    // multiplication
    let mulitplication = 5 * 5;

    // division
    let division = 25 / 5;

    let remainder = 12 % 5;

    // bool
    let t: bool = true;
    let f: bool = false;

    // char
    let c: char = 'a';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    // compound types (pairs)
    /**  ---- TUPLE ---- **/
    let tuple: (i32, f32, u8) = (500, 3.14, 0);
    let tuple_as_pair: (i8, i8) = (12, 6);

    // tuple destructor to get value
    let (a, b, c) = tuple;
    let (x, y) = tuple_as_pair;
    println!("Tuple: {a}, {b}, {c}");
    println!("Tuple as pair: {x}, {y}");

    // tuple access via index
    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;
    println!("Tuple via index. [{a}], [{b}], [{c}]");


    /** ---- ARRAYS ---- **/
    let arr_a = [1, 2, 5, 6, 3, 9];
    let arr_b = [3; 10]; // contain 10 elements with value of 3. [3, 3, 3, 3, 3....] 10 times
    let three = arr_b[4];
    println!("arr_b at index[4] = {three}");

    println!("Enter index you'd like access to from 0-5");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read command line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Unable to parse");

    let elemet = arr_a[index];
    println!("The value of the element at index {index} is: {elemet}");
}
