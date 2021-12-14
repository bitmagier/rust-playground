#![allow(unused_variables)]

/// Common Collections
/// vector:   `Vec<T>`

fn main() {
    // create
    {
        let v: Vec<i32> = Vec::new();
    }


    // create with initial values
    {
        let v = vec![1,2,3];
    }



    // updating
    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
    }

    // dropping
    // .. a vector drops its elements
    {
        let v = vec![1, 2, 3, 4];
        println!("{:?}", v)
    }  // <- v goes out of scope and is freed here


    //
    // When the vector gets dropped, all of its contents are also dropped,
    // meaning those integers it holds will be cleaned up.
    // This may seem like a straightforward point but can get a bit more complicated when
    // you start to introduce references to the elements of the vector. Let’s tackle that next!
    //


    // reading
    {
        let v = vec![1 ,2, 3, 4, 5];


        let third: &i32 = &v[2];
        println!("The third element is {}", third);


        let does_not_exist = &v[100]; // panics


        // so use this, when out of range access can happen under normal circumstances:
        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("no third element"),
        }

    }


    // compile error here - why?
    // (see: https://doc.rust-lang.org/book/ch08-01-vectors.html#reading-elements-of-vectors)
    // fn main() {
    //     let mut v = vec![1, 2, 3, 4, 5];
    //
    //     let first = &v[0];
    //
    //     v.push(6);
    //
    //     println!("The first element is: {}", first);
    // }


}