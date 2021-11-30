use std::collections::HashMap;
use std::env;
use std::process::exit;

#[allow(dead_code)]
fn fibonacci_plain(n: u32) -> u32 {
    match n {
        0 => 0, // practically useless
        1 => 1,
        2 => 1,
        x => fibonacci_plain(x - 1) + fibonacci_plain(x - 2)
    }
}

fn fibonacci_cached(n: usize, cache: &mut HashMap<usize, u128>) -> u128 {
    match cache.get(&n) {
        Some(&value) => value,
        None => {
            let result = match n {
                0 => 0,
                1 => 1,
                2 => 1,
                _ => fibonacci_cached(n - 1, cache) + fibonacci_cached(n - 2, cache),
            };
            cache.insert(n, result);
            result
        }
    }
}

fn fibonacci_with_cache(n: usize) -> u128 {
    let mut cache: HashMap<usize, u128> = HashMap::new();
    fibonacci_cached(n, &mut cache)
}


fn print_nth_fibonacci(n: &str) {
    let n = n.parse::<usize>().expect("please provide a positive integer number!");
    // let result = fibonacci_plain(n);
    let result = fibonacci_with_cache(n);
    println!("{}th fibonacci number is: {}", n, result);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <number>", args[0]);
        exit(1)
    }
    print_nth_fibonacci(&args[1]);
}
