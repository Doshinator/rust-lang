// If mod is declared in main
// pub mod garden;
// use crate::garden::vegetables::potato;
// then you can just use potato() or vegetables::potato()



// if mod is declared in lib.rs
use modules::garden::vegetables;
// then you can just use potato() or vegetables::potato() - 


fn main() {
    vegetables::potato();
}
