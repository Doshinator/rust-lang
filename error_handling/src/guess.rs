pub struct Guess {
    val: i32,
}

impl Guess {
    pub fn new(val: i32) -> Self {
        if val < 1 || val > 100 {
            panic!("{val} is not between 1-100. Value must be between 1-100.");
        }

        Guess { val }
    }

    pub fn value(&self) -> i32 {
        self.val
    }
}