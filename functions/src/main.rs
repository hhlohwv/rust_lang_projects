// Functions tutorial page on rust

// fn main() {
//     println!("Hello, World!");

//     another_function();
// }

// fn another_function() { // rust doesn't care where functions are defined, could be before or after main()
//     println!("Another function.");
// }

// functions with parameters
// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is : {}", x);
// }

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {}{}", value, unit_label);
// }

// // Statements and Expressions
// fn main() {
//     let y = 6;

//     let z = {  // working with a second scope
//         let a = 3;
//         a + 2  // including a semicolon does not allow a value to be return to z
//     };

//     println!("The value of z is: {}", z);
// }

// Returning values from functions
fn five() -> i32 {  // declaring the data type of the return value
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);

    let y = plus_one(12);

    println!("The value of y plus one is: {}", y)
}

// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {}", x);
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1;  // this gives an error because () is returned with the semicolon repressing the output
//     // function output expects an i32 value
// }