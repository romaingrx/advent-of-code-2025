use std::{fs, iter::zip};

fn parse_input(input: &str) -> Result<(Vec<Vec<u64>>, Vec<&str>), String> {
    let mut vecs = input.lines().rev().map(|line| line.split_whitespace().collect());
    let operators: Vec<&str> = vecs.next().ok_or("Missing operators line")?;
    let numbers = vecs.try_fold(
        vec![vec![]; operators.len()],
        |mut acc, elements| -> Result<Vec<Vec<u64>>, String> {
            for (idx, elem) in elements.into_iter().enumerate() {
                let digit = elem
                    .parse::<u64>()
                    .map_err(|err| format!("Invalid number ({}): {}", elem, err))?;
                acc[idx].push(digit);
            }
            Ok(acc)
        },
    )?;

    Ok((numbers, operators))
}

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test { "test_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(format!("src/day_06/{}", input_file))
        .expect("Failed to read input file");

    let (numbers, operators) = parse_input(input.as_str()).expect("Failed to parse input");

    let result = match part {
        1 => part1(&numbers, &operators),
        _ => {
            println!("Part {} not implemented for day 6", part);
            return;
        },
    };

    println!("Day 6 Part {}: {:?}", part, result);
}

fn part1(numbers: &Vec<Vec<u64>>, operators: &Vec<&str>) -> Result<u64, String> {
    let results: Result<Vec<u64>, String> = zip(numbers, operators)
        .map(|(nums, &op)| match op {
            "+" => nums
                .iter()
                .cloned()
                .reduce(|a, b| a + b)
                .ok_or("Empty list for addition".to_string()),
            "*" => nums
                .iter()
                .cloned()
                .reduce(|a, b| a * b)
                .ok_or("Empty list for multiplication".to_string()),
            _ => Err("Unknown operator".to_string()),
        })
        .collect();

    let sum = results?.into_iter().sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let numbers =
            vec![vec![123, 45, 6], vec![328, 64, 98], vec![51, 387, 215], vec![64, 23, 314]];
        let operators = vec!["*", "+", "*", "+"];
        assert_eq!(part1(&numbers, &operators), Ok(4277556));
    }
}
