#![allow(dead_code, unused)]

// => A trait tells the Rust compiler about functionality a particular type has
// and can share with other types.
//
// We can use traits to define shared behavior in an abstract way.
// We can use trait bounds to specify that a generic type can be any type that
// has certain behavior


// Defining a Trait

// Trait definitions are a way to group method signatures together
// to define a set of behaviors necessary to accomplish some purpose

use std::fmt::Debug;

pub trait Summary {
    fn summarize(&self) -> String;
}
// ^^^ Here, we declare a trait using the trait keyword.
//
// Inside the curly brackets, we declare the method signatures that describe the behaviors
// of the types that implement this trait.
// After the method signature, instead of providing an implementation within curly brackets, we use a semicolon.
//
// A trait can have multiple methods in its body:
//   the method signatures are listed one per line and each line ends in a semicolon.


// Implementing a Trait on a Type

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
// After implementing the trait, we can call the methods on instances of NewsArticle [...]
// in the same way we call regular methods

// (!) One restriction to note with trait implementations is that we can implement a trait
//     on a type only if either the trait or the type is local to our crate.
// => So we can’t implement external traits on external types
//
// This restriction is part of a property of programs called coherence,
// and more specifically the orphan rule, so named because the parent type is not present.
// This rule ensures that other people’s code can’t break your code and vice versa.



// Default Implementations
//   Sometimes it’s useful to have default behavior for some or all of the methods in a trait
//   instead of requiring implementations for all methods on every type.
pub trait SummaryB {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl SummaryB for NewsArticle {}


// Default implementations can call other methods in the same trait, even if those other methods
// don’t have a default implementation.

pub trait SummaryC {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// (!) Note that it isn’t possible to call the default implementation from
//     an overriding implementation of that same method.


// Traits as Parameters
// => use traits to define functions that accept many different types


// We can define a notify function that calls the summarize method on its item parameter,
// which is of some type that implements the Summary trait

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// ^^^ Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name
//     This parameter accepts any type that implements the specified trait.


// Trait Bound Syntax

// The impl Trait syntax works for straightforward cases but is actually syntax sugar
//   for a longer form, which is called a trait bound; it looks like this:
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}


// Specifying Multiple Trait Bounds with the + Syntax
pub trait Display {}

pub fn notify3(item: &(impl Summary + Display)) {
    // ...
}

// The + syntax is also valid with trait bounds on generic types:
pub fn notify4<T: Summary + Display>(item: &T) {
    // ...
}


// Clearer Trait Bounds with where Clauses
// => https://doc.rust-lang.org/book/ch10-02-traits.html#clearer-trait-bounds-with-where-clauses

// instead of writing this:
fn some_function_a<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32
{ 0 }

// we can use a where clause, like this:
fn some_function_b<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{ 0 }


// Returning Types that Implement Traits
// => https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits



// (!) However, you can only use impl Trait if you’re returning a single type ...
// (see the book for not working example)
//
// => how to do this is explained here:
//   “Using Trait Objects That Allow for Values of Different Types” section of Chapter 17.
//   https://doc.rust-lang.org/book/ch10-02-traits.html#:~:text=We%E2%80%99ll%20cover%20how%20to%20write%20a%20function%20with%20this%20behavior%20in%20the%20%E2%80%9CUsing%20Trait%20Objects%20That%20Allow%20for%20Values%20of%20Different%20Types%E2%80%9D%20section%20of%20Chapter%2017.


// Fixing the largest Function with Trait Bounds
// see code in https://doc.rust-lang.org/book/ch10-02-traits.html#fixing-the-largest-function-with-trait-bounds


// Using Trait Bounds to Conditionally Implement Methods
// => By using a trait bound with an impl block that uses generic type parameters,
//    we can implement methods conditionally for types that implement the specified traits
// see example in https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods






// We can also conditionally implement a trait for any type that implements another trait.
// Implementations of a trait on any type that satisfies the trait bounds are called
// blanket implementations and are extensively used in the Rust standard library.

// The standard library implements the ToString trait on any type that implements the Display trait:
/*
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        todo!()
    }
}
*/
// Because the standard library has this blanket implementation,
// we can call the to_string method defined by the ToString trait
// on any type that implements the Display trait.

// => For example, we can turn integers into their corresponding String values like this
//    because integers implement Display
fn x() {
    let s = 3.to_string();
}



// [last paragraph from the book chapter]
// Traits and trait bounds let us write code that uses generic type parameters to reduce
// duplication but also specify to the compiler that we want the generic type to have
// particular behavior. The compiler can then use the trait bound information to check that
// all the concrete types used with our code provide the correct behavior. In dynamically
// typed languages, we would get an error at runtime if we called a method on a type which
// didn’t define the method. But Rust moves these errors to compile time so we’re forced to
// fix the problems before our code is even able to run.
//
// Additionally, we don’t have to write code that checks for behavior at runtime because
// we’ve already checked at compile time. Doing so improves performance without having to give up
// the flexibility of generics.






fn main() {}