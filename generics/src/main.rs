mod generics;
mod traits;
mod lifetime;

use crate::{generics::{largest, largest_char, largest_generic}, traits::{longer_string, notifity, notify, notify_same_type, returns_summary_trait, Article, Summary, Tweet}};

struct Point<T> {
    x: T,
    y: T,
}

struct Point_Multi<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

struct Points<T, U> {
    x: T,
    y: U,
}

impl <T, U> Points<T, U> {
    pub fn mixup<V, W>(self, point: Points<V, W>) -> Points<T, W> {
        Points {
            x: self.x,
            y: point.y
        }
    }
}

// we can also constraint type; constraint Point to only accept f32
impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest_result: &i32 = largest(&number_list);

    println!("Largest = {largest_result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result: &i32 = largest(&number_list);
    println!("The largest number is {result}");



    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    // ----------- generics
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_generic(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_generic(&char_list);
    println!("The largest char is {result}");

    // generic struct
    let int = Point {x: 1, y: 5};
    let float = Point { x: 1.0, y: 4.0 };
    println!("x={}",int.x());


    // Point struct where x and y are both generics but could have different types, we can use multiple generic type parameters
    // ----- multiple generic types
    let both_integer = Point_Multi { x: 5, y: 10 };
    let both_float = Point_Multi { x: 1.0, y: 4.0 };
    let integer_and_float = Point_Multi { x: 5, y: 4.0 };
    
    // this function is constraint to only f32 Point types
    println!("Distance from origin: {}", float.distance_from_origin());

    // ------- TRAITS -------------
    
        // pub struct Article,
        // pub author: String,
        // pub content: String,
        // pub headline: String,
        // pub location: String,

        let article = Article {
            author: String::from("united states millitary"),
            content: String::from("nein nein nein"),
            headline: String::from("9 - nein"),
            location: String::from("germany"),
        };

        // pub struct Tweet,
        // pub author: String,
        // pub post: String,
        // pub reply: bool,
        // pub response: bool,
    let post = Tweet {
        author: String::from("bob jones"),
        post: String::from("United we stand, devided we fall"),
        reply: true,
        response: true
    };

    println!("Summary of the new aritcle: {}", article.summarize());
    println!("Summary of the new post: {}", post.summarize());
    println!("default behaviore of summarize_default(): {}", article.summarize_default());

    // notify(&article);
    notifity(&article, &post);


    let t = returns_summary_trait();
    println!("returning traits result: {}", t.summarize_author());
    
    let x = String::from("This is obviously a longer string.");
    
    {
        let y = "Hello world.";
        let result: &str = longer_string(&x, y);
        println!("The longest string is {result}");
        // The generic lifetime ’a FORCES the compiler to ensure that
        // result will NOT live longer than the shortest-lived input (y in this case).
        // generic makes this possible, else we wouldn't be able to call longer_string
        // because compiler doesn't know what reference its returning, which COULD be out of scope,
        // resulting in a dangling refernece
    }
    // this will error out because even though x is longer and is valid becaues it's still in scope...
    // we still broke the contract we declared w/ generics
    // println!("The longest string is {result}");


}
