#![allow(unused_variables)]

/// Common Collections
/// hash map:  `HasMap<K, V>`


use std::collections::HashMap;

fn main() {
    // creating and filling a hash map
    {
        let mut m = HashMap::new();
        m.insert(String::from("A"), 123);
        m.insert(String::from("B"), 42);
        for v in m.values().filter(|&&e| e == 42) {
            println!("{}", v)
        }
    }


    // creating and filling using zip function
    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let mut map: HashMap<_, _> =
            teams.into_iter().zip(initial_scores.into_iter()).collect();
    }


    // getter
    {
        let v = map.get("B");

        let b = String::from("B");
        let v = map.get(&b);

        let v = map.get_mut("B"); // gets a mutable reference to the value corresponding to the key
        let kv: Option<(_, _)> = map.get_key_value("B");
    }

    // remove
    {
        let v = map.remove("A");
        let kv: Option<(_, _)> = map.remove_entry("A");

        map.clear();
    }

    map.len();
    map.is_empty();


    // iterators

    {
        let map: HashMap<String, usize> = HashMap::new();
        map.iter();
        map.into_iter(); // consuming iterator in arbitrary order
    }

    {
        let mut map: HashMap<String, usize> = HashMap::new();
        map.iter_mut();  // iterator with mutable reference to the values
    }



    // consuming iterators
    {
        let map: HashMap<String, usize> = HashMap::new();
        map.keys(); // iterator over keys
        map.into_keys(); // consuming iterator over all keys
    }
    {
        let map: HashMap<String, usize> = HashMap::new();
        map.values(); // iterator over values
        map.into_values(); // consuming iterator over all values
    }


    // retain
    {
        let mut map: HashMap<String, usize> = HashMap::new();
        map.retain(|k, v| k.starts_with("A")); // retains only the elements specified by the predicate
    }








    // Hash Maps and Ownership
    //
    // For types implementing the Copy trait, like i32, the values are copied into the hash map.
    // For owned values like String, the values will be moved so the hash map will be the owner.
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        let mut m = HashMap::new();
        m.insert(field_name, field_value);
    }




    // iterate over key/value pairs
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }


        // common use case:
        // only inserting a value if the key has no value, using the entry API
        scores.entry(String::from("Red")).or_insert(90);
    }


    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }


    // Hashing Function: SipHash: not the fastest hashing algorithm available, but providing
    // resistance to DoS attacks involving hash tables
}