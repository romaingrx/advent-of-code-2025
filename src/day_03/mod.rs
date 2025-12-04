use std::{cmp::Ordering, fs};

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test {
        "test_input.txt"
    } else {
        "input.txt"
    };
    let input = fs::read_to_string(format!("src/day_03/{}", input_file))
        .expect("Failed to read input file");

    // Parse input here
    let banks: Vec<&str> = input.lines().collect();

    let result = match part {
        1 => part1(&banks),
        _ => {
            println!("Part {} not implemented for day 3", part);
            return;
        }
    };

    println!("Day 3 Part {}: {}", part, result);
}

fn part1(banks: &[&str]) -> u64 {
    banks
        .iter()
        .enumerate()
        .map(|(bank_idx, bank)| {
            let (first_max_idx, first_max_char) = bank
                .chars()
                .take(bank.chars().count() - 1)
                .enumerate()
                .max_by(|(_, a), (_, b)| a.cmp(b).then(Ordering::Greater))
                .unwrap();

            let (_, second_max_char) = bank
                .chars()
                .skip(first_max_idx + 1)
                .enumerate()
                .max_by(|(_, a), (_, b)| a.cmp(b).then(Ordering::Greater))
                .unwrap();
            println!("[bank {bank_idx}]: {first_max_char}{second_max_char}");
            format!("{}{}", first_max_char, second_max_char)
                .parse::<u64>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        assert_eq!(part1(&input), 357);
    }

    #[test]
    fn test_tricky() {
        let input = vec!["91111111191"];
        assert_eq!(part1(&input), 99);
    }
}
