use std::env;
use std::process::exit;

fn nth_fibonacci(n: u32) -> u32 {
    match n {
        0 => 0, // practically useless
        1 => 1,
        2 => 1,
        x => nth_fibonacci(x - 1) + nth_fibonacci(x - 2)
    }
}

fn print_nth_fibonacci(n: &str) {
    let n = n.parse::<u32>().expect("please provide a positive integer number!");
    let result = nth_fibonacci(n);
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
