// A struct or structure is a custom data type that lets you
// name and package together multiple related values that make up a
// meaningful group

// Structures and Enumerators are the building blocks for creating new
// types in your program's domain to take full advantage of Rust's
// compile time type checking

// Defining and Instantiating Structures
fn dai_structs() {
    // structs are like tuples because you can have different types
    // each piece of data however has it's own name (unlike tuples)
    struct User {
        username: String, // these are called fields (name: Type)
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // To use a struct we have to create an instance of that struct
    // Fields don't have to be specified in the same order they were declared in
    // We create an instance and it's data with (key: value) pairs
    let mut user1 = User {
        // to initiate a struct we use the name of the struct then a block with field declarations in them
        email: String::from("random@email.com"),
        username: String::from("random"),
        active: true,
        sign_in_count: 1,
    };

    // To get a value from our new struct
    // Use structure name and the value to access (user1.email)
    user1.email = String::from("random2@email.com"); // changes value in a struct because the struct was initiated as mutable

    // entire struct must be mutable (can't make certain fields mutable)

    // structs can be returned from a function
    fn build_user(email: String, username: String) -> User {
        // takes an email and username string and returns a User struct
        User {
            email: email, // you can use shorthand when variables
            username,     // and fields have the same name
            active: true,
            sign_in_count: 1,
        }
        // a "User" struct instance with set fields is returned from this function
    }

    let user2 = build_user(String::from("random3@email.com"), String::from("random2"));
    println!("Username of User 2 is: {}", user2.username);

    // You can create instances from other instances with struct update syntax
    let user3 = User {
        email: String::from("random4@email.com"),
        username: String::from("random4"),
        // instead of active: user1.active,
        // instead of sign_in_count: user1.sign_in_count
        ..user1 // replacement for past two comments (struct update syntax)
                // the syntax .. specifies that the remaining fields not explicitly set should have the same values as the fields in the given instance
    };
    println!("Is user 3 active? {}", user3.active);
    println!("What's user 3 sign in count? {}", user3.sign_in_count);

    // Tuple structs are structs that look similar to tuples
    // They have a name for the struct and the value types that are stored in the struct (no field names)
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("Black is {}{}{} in RGB.", black.0, black.1, black.2);

    // Structs without any fields are called unit-like structs
    // unit-like structs behave like unit types "()"
    // useful in situations in which you need to implement a trait no some type but don't have any data that you want to store in the type itself
    // "String" type is used instead of slice &str types because the str type becomes invalid at the end of the struct definition
    // String stays valid throughout the entire struct validity
}

fn method_syntax() {
    // methods are similar to functions but the only difference is that:
    //  they're defined in the context of a struct, enum or trait object
    //  they're first parameter is always self which represents the instance of the struct the method is being called on

    struct Rectangle {
        width: u32,
        height: u32,
    }

    // implementation blocks are used to define a function within the context of a struct
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height // self parameter allows operations on the same struct that the method was declared in
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            // self is the struct that used the method and other is another rectangle struct passed to check if it fits inside the first one
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 50,
        height: 100,
    };

    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect1));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // Associated functions are function in impl blocks that don't take self as a parameter
    // They are called associated functions because they're associated with the struct
    // They're still functions not methods because they don't have an instance of the struct to work with
    // For example String::from is an associated function
    // :: is used to access associated functions and namespaces created by modules
    // multiple impl blocks are allowed
    // Multiple impl blocks are useful for separating generic types and traits

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            // takes a size parameter
            // and creates a square by returning a rectangle with
            // equal size for both width and height
            Rectangle {
                width: size,
                height: size,
            }
        }
    }
    let rect5 = Rectangle::square(2); // returns a square "Rectangle" struct that can be stored on a new instance of Rectangle
    println!(
        "Square width and height are {}x{}.",
        rect5.width, rect5.height
    )
    // associated functions are often used for constructors that will return a new instance of the struct

    // Rust has automatic referencing and dereferencing
    //  p1.distance(&p2);
    //  (&p1).distance(&p2);
    // same thing because of the ar&d rust has
}

fn main() {
    dai_structs();
    method_syntax();
}
