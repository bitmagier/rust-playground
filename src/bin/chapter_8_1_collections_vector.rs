#![allow(unused_variables)]

/// Common Collections
/// vector:   `Vec<T>`

fn main() {


    {
        // create
        let v: Vec<i32> = Vec::new();

        // create with initial values
        let v = vec![1, 2, 3];
    }








    // updating
    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
    }







    // dropping
    {
        let v = vec![1, 2, 3, 4];
    }  // <- v goes out of scope and is freed here
    // dropping a vector drops its elements

    // When the vector gets dropped, all of its contents are also dropped,
    // meaning those integers it holds will be cleaned up.

    // dropping single elements
    {
        let mut v = vec![1, 2, 3, 4, 5];
        v.remove(1);
        v.pop();
    }






    // reading
    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        let does_not_exist = &v[100];
        // ^^^ panics!


        // better:
        match v.get(3) {
            Some(third) => println!("The third element is {}", third),
            None => println!("no third element"),
        }
        // ^^^ so use .get(), when out of range access can happen under normal circumstances




        // some further nice access methods (from slice type)
        let e = v.first();
        let e = v.last();
        let yes = v.starts_with(&[1, 2]);
        let yes = v.ends_with(&[5]);
        let yes = v.contains(&2);
    }






    // compile error here - why?
    //
    // fn main() {
    //     let mut v = vec![1, 2, 3, 4, 5];
    //
    //     let first = &v[0];
    //
    //     v.push(6);
    //
    //     println!("The first element is: {}", first);
    // }
    //
    // for details see: https://doc.rust-lang.org/book/ch08-01-vectors.html#reading-elements-of-vectors









    // iterating over values in a vector
    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
    }



    // iterate over mutable references
    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
    }










    // Using Enum to store multiple types in the same collection
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }


    // see API doc: https://doc.rust-lang.org/std/vec/struct.Vec.html
}


