use std::fs;
use std::io::{self, Error};
use std::net::IpAddr;
use std::{fs::File, io::{ErrorKind, Read}};

pub fn panic() {
    panic!("Crash and Burn.");
}

pub fn handle_result_using_match() {
    let file_handler = File::open("hello.txt");
    let file_ = match file_handler {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {e:?}"),
    };
}

pub fn file_handle() {
    let file_handler = File::open("hello.txt");
    let file = match file_handler {
        Ok(fc) => fc,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(err) => panic!("Problem creating the file: {err:?}"),
            },
            _ => panic!("Problem creating the file {e:?}"),
        },
    };
}

pub fn closure() {
    let file_hander = File::open("hello.txt")
        .unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|err| {
                    panic!("Problem creating the file. {err:?}");
                })
            } else {
                panic!("Problem opening the file: {error:?}");
            }
        });
}

pub fn unwrap() {
    // returns success or panics
    let file_handler = File::open("hello.txt").unwrap();
}


// use this in production for specific panic errors
pub fn expect() {
    //  to return the file handle or call the panic! macro
    let file_handler = File::open("hello.txt")
        .expect("No file 'hello.txt' found in directory. Unable to open."); 
}

// Longer way to error handle

pub fn file_handle_with_return_result() -> Result<String, io::Error> {
    let file_handler = File::open("hello.txt");

    let mut file = match file_handler {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


// success - returns value that is held inside Ok() 
// failure - receive an Err value that holds an instance of io::Error 
// (because both the error from file::open and create are both of type io::Error)
pub fn open_and_return_file_content() -> Result<String, io::Error> {
    let file_handler = File::open("hello.txt");
    
    let mut file = file_handler.unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
            .unwrap_or_else(|error| {
                panic!("Unable to create file. {error:?}");
            })
        } else {
            panic!("Problem opening the file: {err:?}");
        }
    });

    let mut file_bytes = String::new();

    match file.read_to_string(&mut file_bytes) {
        Ok(_) => Ok(file_bytes),
        Err(e) => Err(e),
    }
}

pub fn read_file_but_shorter_fn() -> Result<String, io::Error> {
    // if value of Result is Ok, '?' will return the value INSIDE Ok.
    // if Err, the Err will be returned from the whole function
    let mut file = File::open("hello.txt")?;
    let mut file_bytes = String::new();
    file.read_to_string(&mut file_bytes)?;
    // ? will convert Err into whatever error type is inside Result<T,E> using from automatically
    Ok(file_bytes)
    // function below is even a shorter verison of this
    
    // *** ? also performs early return if there are error
}

pub fn even_shorter_version() -> Result<String, io::Error> {
    let mut file_content = String::new();
    File::open("hello.txt")?.read_to_string(&mut file_content)?;
    Ok(file_content)
}

pub fn built_reading_file_fn() -> Result<String, io::Error> {
    // creates new mutable string
    // opens file
    // reads the file
    // writes file content into mut string 
    // returns the string
    fs::read_to_string("hello.txt")
    // tldr; 
    //opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it
}

pub fn ip() {
    let localhost: IpAddr = "127.0.0.1"
        .parse()
        // document the reason you think you’ll never have an Err variant in the argument text (use expect)
        .expect("Hardcoded IP address should alway be valid");
}

// Use expect in almost ALL cases
    // Document these things in the expect custom message
    // What failed?
    // Why was it unexpected?
    // What's the fix?
// use unwrap in prototype phase / test cases


// --------------------------
// good substitue for checking conditions (ie, number between 1-100 w/ if condition everywhere in our program)
// ----- NEW TYPE
// make a new type in a dedicated module and put the validations in a function to create an instance of the 
// type rather than repeating the validations everywhere

