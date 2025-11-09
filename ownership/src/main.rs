use std::string;

fn main() {

    /**
     * 
     *  SCOPE + MEMORY ALLOCATION
     */
    // str literals are IMMUTABLE - there is no s.push_str() method for string literals (&str)
    let mut s = "Hello"; //  string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable
    
    { // s comes in scope here
        let mut s = String::from("Hello"); // Allocated on the heap
        s.push_str("_World");
        println!("{s}");
    } // s out of scope after this
    // s no longer valid here


    
    // STACK behaviore different when Moving variables
    // integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack.
    // types such as integers that have a known size at compile time are stored entirely on the stack, 
    // so copies of the actual values are quick to make.. 
    // there’s no reason we would want to prevent x from being valid after we create the variable y
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");


    /**
     * 
     * MOVE
     */
    // with string is DIFFERENT
    // A String is made up of three parts
    // 1) a pointer to the memory that holds the contents of the string
    // 2) a length
    // 3) a capacity
    let s1 = String::from("Hello");

    let s2 = s1; // copy the ptr, len and cap that are on the stack
    // if s2 AND s1 are pointing at the same memory and both are called to free, there will be memory issue**

    // s2 = s1 -----> s1 was MOVED into s2. s1 is NO longer in scope after move (because it's on heap)


    /**
     * 
     * SCOPE & ASSIGNMENTS
     * 
     */
    // When you assign a completely new value to an existing variable, Rust will call drop and free the original value’s memory immediately

    let mut s = String::from("Hello");
    // At this point, nothing is referring to the original value on the heap at all.So original s containing "hello" is freed
    s = String::from("ahoy"); // original s pointing to "hello" is freed.
    println!("{s} world.");


    /**
     * CLONES - deep copy
     * Variables and Data Interacting with Clone
     */

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");


    /**
     * 
     * COPY
     */
    // copy trait, variables that use it do not move, 
    // but rather are trivially copied, making them still valid after assignment to another variable.
    // any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy



    /**
     * OWNERSHIP + FUNCTIONS
     */

    // passing values down to function is simliar to passing values to variable
    // Passing a variable to a function will move or copy, just as assignment does

    let s = String::from("Hello");
    take_ownership(s); // we MOVE s into this function and this function takes ownership
    // println!("This should give compile error becuase s is out of scope and freed after function call ends {s}");

    let x = 5;
    makes_copy(x);
    println!("No problem printing integer x because it's an integer on stack {x}");


    /**
     * 
     * Return Values and Scope
     */

    // Returning values can also transfer ownership
    let s1 = give_ownership(); // ownership is given here

    let s2 = String::from("Hello - scoping"); // init new owner (s2)
    let s3 = take_and_give_back(s2); // moving ownership into this function and moving ownership of (new or old value) into s3
    println!("{s3}");


    // What if we want to let a function use a value but not take ownership?
    /**
     * 
     * BORRWING via.. REFERENCE (guarantees to point to a valid data)
     */
    // *note we can't modify value we are borrowing, unless we have a mutable reference

    let s1 = String::from("Hello World!");
    let len_s1 = calculate_len(&s1); // we aren't transferring ownership, we are borrowing
    println!("Size of {s1} = {len_s1}"); // so we can use s1 later down the road if we need to!

    let mut s1 = String::from("Hello");
    change_str(&mut s1);
    println!("We can still use s1! because it was borrowed, not given ownership of! and it's mutable!");

    // restriction: if you have a MUTABLE reference to a value, you can have no other references to that value
    /* 
        let mut s = String::from("Some random string");
        let reference_to_s = &mut s;
        let second_reference_to_s = &mut s;
        println!("{reference_to_s}, {second_reference_to_s}");
        // fix - use scope to let 1 mut reference go out of scope, then use delacre another reference
    */

    // We also cannot have a mutable reference while we have an immutable one to the same value. 
    // but.... multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.
    mutable_ref_while_having_immutable_ref();

    // Dangling reference
    // let reference_to_nothing = dangle();



    /**
     * 
     * SLICE -> a type of reference, therefore it doesn't have ownership
     */
    array_slice();



}


// ------------------------------

fn take_ownership(s: String) { // s comes into scope
    println!("{s}");
} // s is out of scope and gets dropped. s is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens. (b/c integers are on STACK)

fn give_ownership() -> String {
    String::from("giving this string")
}

fn take_and_give_back(s: String) -> String {
    s // moving ownership out by retuning
}

fn calculate_len(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
// it refers to, the String is not dropped.


fn change_str(s: &mut String) {
    s.push_str("_world is pushed");
    println!("{s}");
}

fn mutable_ref_while_having_immutable_ref() {
    // 1)
    let mut s = String::from("Halo");
    let r1 = &s; // okay!
    let r2 = &s; // okay!
    let mut r3 = &mut s; // BIG PROBLEM
    // println!("{r1}, {r2}, and {r3}"); can't have both mut and immuatable reference

    // 2) r1 and r2 dropping scope
    let mut s = String::from("Halo");
    {
        let r1 = &s; // okay!
        let r2 = &s; // okay!
    } // r1 and r2 has left the scope and are dropped

    let mut r3 = &mut s; // this is now a valid mutable reference because there are no other immutable ref to s
    println!("{r3}");

    // 3) r1 and r2 dropped by ownership
    let mut s = String::from("Halo");
    let r1 = &s; // okay!
    let r2 = &s; // okay!
    println!("{r1} and {r2}"); // r1 and r2 has been dropped after this because of ownership
    let mut r3 = &mut s; // okay! (because r1 and r2 were consumed by println and dropped)

}

// fn dangle() -> &String {
//     let s = String::from("dangling");
//     &s // we return address to the s
// } // s goes out of scope here, so if you give reference of an object that is dropped, that is DANGER.
// // can not return a reference of a dropped object / variable!

// so just return -> String.... ownership is moved out and no reference to a dropped variable

/**
 * SLICE - &str are slices! both of which are immutable
 */

fn first_word(s: &String) -> &str {
    let byte = s.as_bytes();
    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


// problematic
fn first_word_BAD(s: &String) -> usize {
    let mut word = String::from("Hello world");
    let word_index = first_word(&word);
    
    word.clear();
    word.len()
    // VERY BAD
    // what does word_index mean if word is cleared?
    // you're accessing word[5] where word no longer exists
    // VERY BAD
}

// slice fixes problem
fn first_word_slice() {
    let s = String::from("Hello world");
    let s0 = &s[0..5]; // &s[0..5] == &s[..5]; you can drop the beginning number (starts from index 0)
    let s1 = &s[6..11]; // &s[6..11] == &s[6..] you can drop the trailing number (ends at last index)
}

// because you're passing a slice anyways... might as well just use &str as arg
fn first_word_improved(s: &str) -> &str {
    let byte = s.as_bytes();
    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
        return &s[0..1];
        }
    }
    &s[..]
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[..3];

    println!("slice = {:?} , {:?}", slice, &a[..3]);
}