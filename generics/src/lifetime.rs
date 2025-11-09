/**
 * pub fn out_of_scope_reference() {
    let r: &i32;
    {
        let x = 5;
        r = &x;
        
    } // reference assigned to r is dropped here

    // r refers to a memory with a lifetime of &x; therefor INVALID
    println!("{r}");
    }
 */


/**
 *  fn main() {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {r}");   //          |
    }                         // ---------+
 */

// At compile time, Rust compares the size of the two lifetimes and sees that r has a 
// lifetime of 'a but that it refers to memory with a lifetime of 'b.




// ----- VALID because it's in scope and in scope of the lifetime
// fn main() {
//     let x = 5;            // ----------+-- 'b
//                           //           |
//     let r = &x;           // --+-- 'a  |
//                           //   |       |
//     println!("r: {r}");   //   |       |
//                           // --+       |
// }                         // ----------+

pub fn reference_in_sope() {
    let x = 5;
    let r = &x;
    println!("r = {r}");
}

