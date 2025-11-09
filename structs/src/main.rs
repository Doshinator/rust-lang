// & to borrow for read purposes 
// ownership for taking onwership + write to it

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}


impl Rectangle {
    // we can pass self; which represents the instance of the struct the method is being called on.
    // this is called a method (because instance is referred here)
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // when we give a method the same name as a field we want it to only return the value in the field and do nothing else
    fn height(&self) -> u32 {
        self.height
    }

    // method
    fn width(&self) -> bool {
        self.width > 0
    }

    // method
    fn can_hold(&self, inner: &Rectangle) -> bool {
        self.area() > inner.area()
    }


    // --- CONSTUCTOR (associated functions) ---
    // We can define associated functions that don’t have self as their first parameter (and thus are not methods) 
    // because they don’t need an instance of the type to work with
    fn square(size: u32) -> Self { // associated funcitons are often used for constructors that will return a new instance of the struct
        // Self = aliases for the type that appears after the impl keyword, which in this case is Rectangle
        Self { 
            height: size, 
            width: size 
        }
    }

}

fn main() {
    let user1 = create_user(String::from("deez"), String::from("Deeznuts@gmail.com"));
    print_user(&user1);

    // we can create another instance with values from other instances
    let user2 = User {
        // user1's values here are MOVED! (only some are)
        active: user1.active,
        username: user1.username, // we are moving username from user1 to user2 here
        email: String::from("anotheremail@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };
    print_user(&user2);

    
    // let user3 = User {
    //     email: String::from("the_...@gmail.com"),
    //     ..user1 // that's why this is not possible because user1's values were moved from above!
    //  but we can still assign bool, u64 from user1. those are primitives and will be copied on the stack 
    // (b/c bool & u64 both implement copy trait)
    // };

    let blue = Color(0, 255, 180);
    // destructor via index
    let x = blue.0;
    println!("{x}");
    // destructor via tuple
    let Color(x, y, z) = blue;
    println!("{x}, {y}, {z}");
    let scale = 2;
    let rec1 = Rectangle {
        height: dbg!(30 * scale),
        width: 50,
    };
    dbg!(&rec1);
    // accessing fields of a borrowed struct instance does NOT move the field values
    let area = area(&rec1);
    println!("area of {:#?} = {area}", rec1);


    let rec2 = Rectangle {
        height: 420,
        width: 69,
    };

    println!("area of rec2 using METHOD = {} ", rec2.area());


    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };

    let rect2 = Rectangle {
        height: 40,
        width: 10,
    };

    let rect3 = Rectangle {
        height: 45,
        width: 60,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // calling constructor, aka, associated function
    let sq = Rectangle::square(3);

}

fn create_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // don't need username: username because fields is same as parameters of fn
        email, // don't need email: email because fields is same as parameters of fn
        sign_in_count: 1
    }
}

fn print_user(user: &User) {
    println!("{:?}", user);
}

/**
 * 
 * TUPLE STRUCT
 */

struct Color(i32, i32, i32);

/**
 * 
 * UNIT-LIKE STRUCT
 */

struct AlwaysEqual;


/**
 * 
 * LIFETIME - 
 */
struct FixInChapter10;
// struct UserLifeTime {
//     active: bool,
//     username: &str, // this is referencing data owned by someone else
//     email: &str, // referencing data owned by someone else, thus we need to create a ... LIFETIME
//     sign_in_count: u64,
// };

// accessing fields of a borrowed struct instance does not move the field values, which is why you often see borrows of structs
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}