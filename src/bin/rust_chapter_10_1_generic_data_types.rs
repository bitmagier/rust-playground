use std::env;

//  Generics are abstract stand-ins for concrete types or other properties

fn f1() {

}

fn dispatch(function: &str) {
    match function {
        "f1" => f1(),
        _ => println!("unknown function name '{}'", function)
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(function) => dispatch(function.as_str()),
        None => println!("please provide a function name")
    }
}
