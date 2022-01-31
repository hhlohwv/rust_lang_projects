use std::io::BufRead;

// Rust book 3.2 - Data Types
// https://doc.rust-lang.org/book/ch03-02-data-types.html

// Scalar data types

// fn main() {
//     // "statically typed language", must know data type of variables at runtime
//     // "signed" or "unsigned" refer to if the number can be negative
//     // Rust default integer type is i32, 32-bit signed integer

//     let x = 2.0; // f64, float 64 data type

//     let y: f32 = 3.0; // f32, float 32 value, explicit designation of data type

//     println!("f64 value x = {}", x);
//     println!("f32 value y = {}", y);

//     let sum = 5 + 10; // addition

//     let difference = 95.5 - 4.3;

//     let product = 4 * 21;

//     let quotient = 56.7 / 32.2;
//     println!("Quotient is {}", quotient);

//     let floored =  2 / 3; // results in zero?
//     println!("floored value is {}", floored);

//     let remainder = 43 % 5;
// }   


// fn main() {
//     // Booleans, data type defined using 'bool' designation

//     let t = true;

//     let f: bool = false; // explicit type annotation
// }


// fn main() {
//     // The character type

//     let c = 'z'; // character type with single quotation marks, string literals with double quotation marks
//     let c = 'ℤ';
//     let heart_eye_cat = '😻';
// }


// Compound Data Types, grouping multiple values into one data type, tuples and arrays

// fn main() {
//     // The Tuple Type, data types can be different
//     let tup: (i32, f64, u8) = (500, 6.4, 1);

//     let (x, y, z) = tup; // destructuring the tuple into values

//     println!("The value of y is: {}", y);

//     // another way of accessing elements of a tuple
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0; // '.' with index number, starts from zero

//     let six_point_four = x.1;

//     let one = x.2;
// }


// fn main() {
//     // The array data type, elements of data types must be the same
//     let a: [i32; 5] = [1, 2, 3, 4, 5]; // with explicit data type and number of array elements

//     let b = [3; 10];  // for copying the first value the number of times in the 2nd slot

//     // Accessing elements of the array
//     let first = a[0];
//     let second = a[1];


// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("PLease enter an array index.");

    let mut index = String::new();

    io::stdin()  // portion which accepts user input
        .read_line(&mut index) // needed to actually read the line
        .expect("Failed to read line");  // for error handling, displays the passed message if error occurs

    let index: usize = index
        .trim()  // trim removes whitespace before and after
        .parse() // converts, or "parses", a string into a number, can easily cause errors
        .expect("Index entered was not a number"); // again, a message to display if an error occurs during execution

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}