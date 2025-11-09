fn create_empty_string() -> String {
    let s = String::new();
    s
}

pub fn to_string() {
    let data: &'static str = "some &str string";
    let s = data.to_string();
    // or 
    let s = "some string".to_string(); // directly using to_string() on a literal
    let s = String::from("some string literal"); // this is also equivalent using string from
    let hello = String::from("नमस्ते");
    println!("{hello}");
}

pub fn concat_string() {
    let mut s = "foo".to_string();
    s.push_str(" bar");

    print!("{s}");

    let s2 = "()";
    s.push_str(s2); // we'll make sure push_str will take a reference, (str literal), rather than String - which would
    // give ownership
    println!("{s2}"); // still can use s2 here because we're not giving up ownership of s2

    let mut s3 = String::from("lo");
    s3.push('l');
    println!("{s3}");


    // using + operator for concat
    let s1 = String::from("hello");
    let s2 = String::from("_world");
    let s3 = s1 + &s2; // we moved s1 here, so s1 can NO longer be used later! print({s1}) would give an error!
    // s2 (String) -> converts into &s[..] slice literaly and appended..

    /**
     * Adding multiple concat (3+)
     */
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3; this gets too cumbersome
    let s = format!("{s1}-{s2}-{s3}"); // equal and esier to write
    println!("{s}. {s1}. {s2}. {s2}."); // format does NOT take ownership! it uses reference to borrow the String!

    // you can NOT index into a string
    // &s[0] is INVALID
    // &s[0..1] VALID - BUT BE CAUTIOUS. 
    // let hello = "Здравствуйте"; &hello[0..1] is INVALID! there is no 0 to 1st index you can slice

    /**
     * TO ITERATE OVER A STRING - use .chars() for individual chars or .byte() for the UTF-8 byte
     */

    // ---- using chars()
    let s = "Здравствуйте";
    for c in s.chars() {
        println!("{c}");
    }

    // ---- using bytes()
    for b in s.bytes() {
        println!("{b}");
    }

    // combining both chars & bytes
    for (c, b) in s.chars().zip(s.bytes()) {
        println!("char {c} : byte {b}");
    }

}