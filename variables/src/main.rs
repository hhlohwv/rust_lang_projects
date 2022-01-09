// Bookmark: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

// fn main() {
//     let x = 5;  // not sure why the : i32, but I guess these things need to be explicitly declared? Just an autofill from VS Code, so will ignore for now
//     println!("The value of x is: {}", x);

//     x = 6; // error thrown because the immutable x is attempted to be set as another value.
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let mut x = 5; // let keyword gives mutability designation
//     println!("The value of x is: {}", x);

//     x = 6;  // because x was designated as mutable above ('mut') its value can now be modified
//     println!("The value of x is now: {}", x);
// }

// fn main() {
//     // data type must always be 'annotated'? I guess defined?
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // u32 means unsigned 32-bit integer(?), common type for constants?

//     // Note: naming convention for constants is all capital letters. No error if not, just a warning during compiling

//     println!("{}", THREE_HOURS_IN_SECONDS)
// }

fn main() {
    // first scope
    let x = 5; // immutable variable assignment
    println!("Initial x value is: {}", x);

    let x = x + 1; // 'shadowing' the variable x, incrementing it by 1. Also works without 'let' if initial x is modified by 'mut'
    // allows a variable to be changed but still immutable
    println!("New value of x plus 1: {}", x);

    // second scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);

        let y = 10;
        println!("Initial setting 2nd scope variable 'y' as: {}", y);
    }

    println!("The value of x post 2nd scope is: {}", x);
    // println!("To reiterate, the value of 2nd scope variable 'y' is: {}", y);

    // first time working with scopes, but essentially it looks like whatever is available in the outer scope is usable by the inner one
    // if a variable is defined only in an inner scope, then outer scopes don't have access to it
}
