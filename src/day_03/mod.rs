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
        2 => part2(&banks),
        _ => {
            println!("Part {} not implemented for day 3", part);
            return;
        }
    };

    println!("Day 3 Part {}: {}", part, result);
}

fn part1(banks: &[&str]) -> u64 {
    get_max_joltage(banks, 2)
}

fn part2(banks: &[&str]) -> u64 {
    get_max_joltage(banks, 12)
}

fn get_max_joltage(banks: &[&str], n_batteries: usize) -> u64 {
    banks
        .iter()
        .enumerate()
        .map(|(bank_idx, bank)| {
            let mut current_idx = 0;
            let mut batteries = "".to_owned();

            for _ in 0..n_batteries {
                let n_take = bank.len() + batteries.len() + 1 - n_batteries - current_idx;
                let (max_idx, max_char) = bank
                    .chars()
                    .skip(current_idx)
                    .take(n_take)
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.cmp(b).then(Ordering::Greater))
                    .unwrap();

                current_idx += max_idx + 1;
                batteries.push(max_char);
            }

            println!("[bank {bank_idx}]: {batteries}");

            batteries.parse::<u64>().unwrap()
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

    #[test]
    fn test_simple() {
        let input = vec!["911118"];
        assert_eq!(get_max_joltage(&input, 3), 918);
    }

    #[test]
    fn test_part_2() {
        let input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        assert_eq!(part2(&input), 3121910778619);
    }
}
