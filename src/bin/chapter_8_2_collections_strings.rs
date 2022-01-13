#![allow(unused_variables, unused_mut, dead_code, unused_doc_comments)]

/// Common Collections
/// UTF-8 Encoded Text: `String`
///
/// Strings are implemented as a collection of bytes, plus some methods to provide useful functionality
/// when those bytes are interpreted as text.
///
/// Rust has only one string type in the core language: string slice `str`  (`&str`).
/// The standard library provided a String type, which is a growable, mutable and owned UTF-8 encoded string type.
/// When Rustaceans refer to “strings” in Rust, they usually mean the String and the string slice &str types, not just one of those types.
///
/// More (owned and borrowed) variants for different encoding or in memory representations:
/// OsString, OsStr
/// CString, CStr
///

fn main() {

    // creating empty
    let mut s = String::new();

    // creating with content
    let data = "initial contents"; // a string literal - or reference to string slice stored in program binary
    let s = data.to_string();
    // or
    let s = "initial data".to_string();
    // or
    let s = String::from("initial contents");
    /// ^^^ they all do the same thing


    // Thanks to UTF8!
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");





    // updating
    let mut s = String::from("foo");
    // append
    s.push_str("bar"); // <- parameter is a reference to a string slice - no ownership transfer


    // append a single character
    s.push('z');









    // combine two existing strings with the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    /// ^^^ uses the add method => `fn add(self, s: &str) -> String {`


    /// Some magic involved here:
    ///
    ///_from the rust book:_
    /// First, s2 has an &, meaning that we’re adding a reference of the second string to the first string
    /// because of the s parameter in the add function: we can only add a &str to a String;
    /// we can’t add two String values together.
    /// But wait — the type of &s2 is &String, not &str, as specified in the second parameter to add.
    /// So why does Listing 8-18 compile?
    ///
    /// The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str.
    /// When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..].
    /// We’ll discuss deref coercion in more depth in Chapter 15.
    /// Because add does not take ownership of the s parameter, s2 will still be a valid String after this operation.
    ///
    /// Second, we can see in the signature that add takes ownership of self, because self does not have an &.
    /// This means s1 in Listing 8-18 will be moved into the add call and no longer be valid after that.
    /// So although let s3 = s1 + &s2; looks like it will copy both strings and create a new one,
    /// this statement actually takes ownership of s1, appends a copy of the contents of s2,
    /// and then returns ownership of the result.
    /// In other words, it looks like it’s making a lot of copies but isn’t; the implementation is more efficient than copying.




    // doing complex string concatenation
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);












    // Indexing into Strings (=> a bad idea)
    /// `
    /// let s1 = String::from("hello");
    /// let h = s1[0];
    /// `
    // Short version: Rust does not support indexing of UTF-8 encoded text.
    // Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.
    // Why?
    // Internal representation:
    // A String is a wrapper over a Vec<u8>
    let hello = String::from("Здравствуйте"); // how long is that string? (12 chars, 24 bytes)

    // A final reason Rust doesn’t allow us to index into a String to get a character
    // is that indexing operations are expected to always take constant time (O(1)).
    // But it isn’t possible to guarantee that performance with a String,
    // because Rust would have to walk through the contents from the beginning to the index
    // to determine how many valid characters there were.


    // So Rust asks you to be more specific if you really need to use indices to create string slices.
    let s = &hello[0..4];
    // ^^^ Here, s will be a &str that contains the first 4 bytes of the string = "Зд"

    // What would happen if we used &hello[0..1]?
    // The answer: Rust would panic at runtime in the same way as if an invalid index were accessed in a vector.
    // You should use ranges to create string slices with caution, because doing so can crash your program.








    // Methods for iterating over Strings
    // chars(), bytes()

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // Result (unicode scalar values):
    // न
    // म
    // स
    // ्
    // त
    // े
    // ^^^ Hindi word in Devanagari script





    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    // This code will print the 18 bytes that make up this String:
    // 224
    // 164
    // --snip--
    // 165
    // 135


    // Summary (from the rust book)
    // ============================
    // To summarize, strings are complicated.
    // Different programming languages make different choices about how to present this complexity to the programmer.
    // Rust has chosen to make the correct handling of String data the default behavior for all Rust programs,
    // which means programmers have to put more thought into handling UTF-8 data upfront.
    // This trade-off exposes more of the complexity of strings than is apparent in other programming languages,
    // but it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle.
}
