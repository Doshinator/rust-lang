mod error;
mod guess;

use std::fs::File;

use crate::error::{closure, expect, file_handle, file_handle_with_return_result, handle_result_using_match, open_and_return_file_content, panic, read_file_but_shorter_fn, unwrap};
use crate::guess::Guess;
fn main() {
    println!("Hello, world!");
    // panic();
    // let v = [1, 99];
    // v[99];
    // file_handle();
    // closure();
    // let result = file_handle_with_return_result();
    // let file_content = result.expect("unable to read file");
    // println!("content of file: {file_content}");


    // long version
    // let file_content = open_and_return_file_content()
    //     .expect("Unable to open & extract content from file.");
    // println!("File content of hello.txt: {file_content}");

    // short version
    // println!("short version to read file content: {}", read_file_but_shorter_fn().expect("error reading file"));

    // can only use in fn that reutrns Result<T,E>. Main() does NOT return Result<T,E>
    // let file_handler = File::open("hello.txt")?;  use match or change the return type of the fn

    let g = Guess::new(10);
    let v = g.value();
    println!("value of your guess: {}", v);

}
