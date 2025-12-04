use std::fs;

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test {
        "test_input.txt"
    } else {
        "input.txt"
    };
    let input = fs::read_to_string(format!("src/day_03/{}", input_file))
        .expect("Failed to read input file");

    // Parse input here
    // let lines: Vec<&str> = input.lines().collect();

    let result = match part {
        1 => part1(&input),
        _ => {
            println!("Part {} not implemented for day 3", part);
            return;
        }
    };

    println!("Day 3 Part {}: {}", part, result);
}

fn part1(_input: &str) -> i32 {
    // Implement your solution here
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "";
        assert_eq!(part1(input), 0);
    }
}
