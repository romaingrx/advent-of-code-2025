mod day_01;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <day_number> [--test]", args[0]);
        println!("Example: {} 1", args[0]);
        println!("Example: {} 1 --test", args[0]);
        return;
    }

    let day = &args[1];
    let is_test = args.contains(&"--test".to_string());

    match day.as_str() {
        "1" => day_01::run(is_test),
        _ => println!("Day {} not implemented yet", day),
    }
}
