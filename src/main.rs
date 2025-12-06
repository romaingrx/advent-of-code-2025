mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <day[.part]> [--test]", args[0]);
        println!("Example: {} 1      # runs day 1 part 1", args[0]);
        println!("Example: {} 1.2    # runs day 1 part 2", args[0]);
        println!("Example: {} 1 --test", args[0]);
        return;
    }

    let day_arg = &args[1];
    let is_test = args.contains(&"--test".to_string());

    // Parse day and part (default to part 1)
    let (day, part) = if day_arg.contains('.') {
        let parts: Vec<&str> = day_arg.split('.').collect();
        (parts[0], parts.get(1).and_then(|p| p.parse::<u8>().ok()).unwrap_or(1))
    } else {
        (day_arg.as_str(), 1)
    };

    match day {
        "1" => day_01::run(part, is_test),
        "2" => day_02::run(part, is_test),
        "3" => day_03::run(part, is_test),
        "4" => day_04::run(part, is_test),
        "5" => day_05::run(part, is_test),
        "6" => day_06::run(part, is_test),
        _ => println!("Day {} not implemented yet", day),
    }
}
