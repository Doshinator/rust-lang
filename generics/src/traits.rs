pub struct Article {
    pub author: String,
    pub content: String,
    pub headline: String,
    pub location: String,
}

pub struct Tweet {
    pub author: String,
    pub post: String,
    pub reply: bool,
    pub response: bool,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.post)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub trait Summary {
    // you have to have which ever type implements Summary trait implement this trait
    fn summarize(&self) -> String;

    // this is a default implementation, so other types which do not implement summerize will have this default
    fn summarize_default(&self) -> String {
        format!("Read more from... {}", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize_default());
}

// can have different types passed in, item1 being Article, item2 being Tweet
pub fn notifity(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news1! {}", item1.summarize_default());
    println!("Breaking news2! {}", item2.summarize_default());
}

// we can also enforce SAME TYPE
// now item1 and item2 have to be of same type (either Article or Tweet)
// ie, notify_same_type(&article, &article);
pub fn notify_same_type<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news1! {}", item1.summarize_default());
    println!("Breaking news2! {}", item2.summarize_default());
}

// ---------- function returning trait type
pub fn returns_summary_trait() -> impl Summary {
    Tweet {
        author: String::from("dixon dallas"),
        post: String::from("Wiskey :)"),
        reply: true,
        response: true
    }
}

//  Rust can’t tell whether the reference being returned refers to x or y
// pub fn longer_string(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// &i32 reference
// &'a i32 // reference with explicit lifetime
// &'a mut i32 // mutable reference with an explicit lifetime

// properly annotiating lifetimes

// we are binding a contract:
// lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in.
pub fn longer_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}