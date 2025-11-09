pub fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest: &i32 = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest_char(list: &[char]) -> &char {
    let mut largest: &char = &list[0];
    for c in list {
        if c > largest {
            largest = c;
        }
    }

    largest
}


// ----- remove duplicate by introducing generics
pub fn largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// generics in struct
// pub struct Point<T> {
//     x: T,
//     y: T,
// }


// ----- enum also can be generics
// When you recognize situations in your code with multiple struct or enum definitions 
// that differ only in the types of the values they hold, you can avoid duplication by using generic types instead.
enum Option<T> {
    Some(T),
    None,
}

enum Reuslt<T, E> {
    Ok(T),
    Err(E),
}