#![allow(dead_code)]

use std::env;

// Generics are abstract stand-ins for concrete types or other properties

// [i32] specific function
fn largest_basic(list: &[i32]) -> i32 {
    let mut largest = list[0];

    // Don't worry about the syntax of the for loop for now.
    // We aren't referencing a reference to an i32 here;
    // we're pattern matching and destructuring each &i32 that the for loop gets
    // so that item will be an i32 inside the loop body.
    // We'll cover pattern matching in detail in Chapter 18.
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn print_largest_basic() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_basic(&number_list);
    println!("largest number of {:?} is {:?}", number_list, result)
}


// Generic Data Types
//
// We can use generics to create definitions for items like function signatures or structs,
// which we can then use with many different concrete data types.
// Let’s first look at how to define functions, structs, enums, and methods using generics.
// Then we’ll discuss how generics affect code performance.

// Generic Data Types
// => in Function Definitions:
//
// First, please see the two similar functions in listing 10-4 in https://doc.rust-lang.org/book/ch10-01-syntax.html#in-function-definitions


// First try to make it generic: does not compile this way -> see compiler output

/*
fn largest<T>(list: &[T]) -> T {
// We read this definition as: the function largest is generic over some type T.
// This function has one parameter named list, which is a slice of values of type T.
// The largest function will return a value of the same type T.
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
*/

// Conclusion:
// Because we want to compare values of type T in the body, we can only use types whose values can be ordered
// To enable comparisons, the standard library has the std::cmp::PartialOrd trait that you can implement on types
// fn largest<T>(list: &[T])


// By the way, here is how the completely fixed version looks like:
fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn print_largest_generic() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("largest number of {:?} is {:?}", number_list, result)
}


// Generic Data Types
// => in Struct Definitions:

// The Point<T> struct is generic over some type T,
// and the fields x and y are both that same type, whatever that type may be
struct PointA<T> {
    x: T,
    y: T,
}

struct PointB<T, U> {
    x: T,
    y: U,
}

// Advice:
// You can use as many generic type parameters in a definition as you want,
// but using more than a few makes your code hard to read.
// When you need lots of generic types in your code,
// it could indicate that your code needs restructuring into smaller pieces.


// Generic Data Types
// => in Enum Definitions:

// 2 examples from standard library:

// Option<T> is an enum that is generic over type T and has two variants:
// Some, which holds one value of type T,
// and a None variant that doesn’t hold any value
enum Option<T> {
    Some(T),
    None,
}

// The Result enum is generic over two types, T and E, and has two variants:
// Ok, which holds a value of type T,
// and Err, which holds a value of type E
enum Result<T, E> {
    Ok(T),
    Err(E),
}


// Generic Data Types
// => in Method Definitions:
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// ^^^ Note that we have to declare T just after impl so we can use it to specify
// that we’re implementing methods on the type Point<T>.
// By declaring T as a generic type after impl, Rust can identify that
// the type in the angle brackets in Point is a generic type rather than a concrete type.


// method only for Point<f32> instances:
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// This code means the type Point<f32> will have a method named distance_from_origin
// and other instances of Point<T> where T is not of type f32 will not have this method defined.
// The method measures how far our point is from the point at coordinates (0.0, 0.0)
// and uses mathematical operations that are available only for floating point types.


// Generic type parameters in a struct definition aren’t always the same
// as those you use in that struct’s method signatures.
// ...
impl<T, U> PointB<T, U> {
    fn mixup<V, W>(self, other: PointB<V, W>) -> PointB<T, W> {
        PointB {
            x: self.x,
            y: other.y,
        }
    }
}
// What for?
// => Please get enlightened using description around Listing 10-11 in https://doc.rust-lang.org/book/ch10-01-syntax.html#in-method-definitions


// Performance:
// The good news is that Rust implements generics in such a way that your code
// doesn’t run any slower using generic types than it would with concrete types.
// Rust accomplishes this by performing monomorphization of the code that is using
// generics at compile time. Monomorphization is the process of turning generic code into
// specific code by filling in the concrete types that are used when compiled.
//
// => more details: https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics


///////////////// MAIN /////////////////

fn dispatch(function: &str) {
    match function {
        "largest_basic" => print_largest_basic(),
        "largest_generic" => print_largest_generic(),
        _ => println!("unknown function name '{}'", function)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(function) => dispatch(function.as_str()),
        None => println!("Please provide a function name:\nlargest_basic\nlargest_generic")
    }
}
