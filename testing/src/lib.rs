pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(i: u64) -> u64 {
    i + 2
}

pub fn greeting(name: &str) -> String {
    format!("hello {name}")
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 3);
        assert_eq!(5, result);
    }

    #[test]
    // we can add should panic if the function we are testing indeed should panic
    #[should_panic]
    // test will fail if we did not panic
    fn another() {
        panic!("intentional panic");
    }

    #[test]
    fn add_two_passes() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let name: String = greeting("carol");
        assert!(name.contains("carol"));
        // assert with custom output for easier debugging
        // assert!(name.contains("carols"), 
        // "Greeting did not contain name, value was `{name}`");
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(0, 1);
    }
}
