fn main() {
    println!("Hello, world!");
    let f: f32 = 32.0;
    let c = f_to_c(f);
    println!("{f} f = {c} c");
    
    let f = 0;
    let c = c_to_f(f);
    println!("{c} c = {f} f");


    let fib = n_fib(3);
    println!("fib seqeuence of 3 = {fib}");

    christmas_carol();
    final_count_down();
}

// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn f_to_c(f: f32) -> f32 {
    let mut f: f32 = f;
    f = f - 32.0;
    f * (5.0/9.0)
}

fn c_to_f(c: i32) -> f32 {
    c as f32 * (9.0/5.0) + 32.0
}

fn n_fib(mut n: i32) -> i32 {
    let mut t1 = 0;
    let mut t2 = 1;
    let mut next_term = 0;

    for i in 0..=n {
        if i == 1 {
            continue;
        }
        if i == 2 {
            continue;
        }

        next_term = t1 + t2;
        t1 = t2;
        t2 = next_term;
    }

    next_term
}


fn christmas_carol() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    for days in days {
        println!("On the {days} of Christmas.");
    }
}

fn final_count_down() {
    for i in (1..=5).rev() {
        println!("index: {i}");
    }
}