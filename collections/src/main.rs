mod string;
mod hashmap;
mod practice;

use crate::string::{concat_string, to_string};
use crate::hashmap::{init_hashmap, iterate_hashmap};
use crate::practice::{median, mode, pig_latin, Ledger};

fn main() {
    // init an empty vector
    let v: Vec<i32> = Vec::new();

    // init vector with inital values
    let v = vec![1, 2, 3];

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(2);
    v.push(1);


    println!("Vector at index 0 = {}", &v[0]);
    println!("Vector at index 1 = {:?}", v.get(1));

    let val: Option<&i32> = v.get(100);
    match val {
        Some(n) => println!("Index 1 = {n}"),
        None => println!("There is no 100th element"),
    }

    // ----- can't have immutable and mutable reference in the same scope -----
    let mut v = vec![1, 34, 5, 9, 102, 100];

    // var third holds a reference, thus it is an (immutable) borrower
    let third = &v[2]; // we have a immutable borrow here!
    println!("Third element = {third}");

    // push() holds a reference, thus it is a (mutable) borrower
    // v.push(69); 

    // This program won’t work if we also try to refer to v element later in the function.
    // println!("Third element = {third}"); // immutable borrow here - we can't do this!

    // ------ loop through vector ------
    for i in &v {
        println!("{i}");
    }
    for (i, &val) in v.iter().enumerate() {
        println!("[{}] = {}", i, val);
    }
    // we can also loop through mutable reference to change values
    for i in &mut v {
        *i = 0;
    }
    // re-printing
    for (i, &val) in v.iter().enumerate() {
        println!("[{}] = {}", i, val);
    }


    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let rows: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(420.69),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    {
        let mut v = vec![532, 23, 10, 999];
        v.pop();
        println!("{:?}", v);
    } // <- v goes out of scope and is freed here

    to_string();
    concat_string();
    init_hashmap();
    iterate_hashmap();

    let median = median(&vec![1, 3]);
    let mode = mode(&vec![1, 1, 5, 69, 0, 420, 9, 69, 69]);
    println!("Median = {}", median);
    println!("Mode = {}", mode);

    println!("{}", pig_latin("apple"));
    println!("{}", pig_latin("fay"));

    let mut ledger = Ledger::new();
    ledger.add_employee("Engineering", "Allice");
    ledger.add_employee("Engineering", "Bob");
    ledger.add_employee("Engineering", "Bob");
    ledger.add_employee("Sales", "Allen");
    ledger.add_employee("HR", "Karen");


    println!("All employeed for Engineering: {:?}", ledger.retrieve("Engineering"));
    println!("All employeed for Sales: {:?}", ledger.retrieve("Sales"));
    println!("All employeed for Hr: {:?}", ledger.retrieve("HR"));
}
