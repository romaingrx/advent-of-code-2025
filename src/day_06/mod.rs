use std::{fs, iter::zip};

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test { "test_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(format!("src/day_06/{}", input_file))
        .expect("Failed to read input file");

    let result = match part {
        1 => part1(input.as_str()),
        2 => part2(input.as_str()),
        _ => {
            println!("Part {} not implemented for day 6", part);
            return;
        },
    };

    println!("Day 6 Part {}: {:?}", part, result);
}

fn parse_input_row_wise(input: &str) -> Result<(Vec<Vec<u64>>, Vec<&str>), String> {
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

fn parse_input_column_wise(input: &str) -> Result<(Vec<Vec<u64>>, Vec<&str>), String> {
    let lines: Vec<&str> = input.lines().collect();
    let last_line = lines.last().ok_or("Input is empty")?;
    let data_lines = &lines[..lines.len() - 1];

    let operators: Vec<&str> = last_line.split_whitespace().collect();
    if operators.is_empty() {
        return Err("No operators found".to_string());
    }

    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let columns: Vec<String> = (0..max_len)
        .map(|col_idx| {
            data_lines.iter().map(|line| line.chars().nth(col_idx).unwrap_or(' ')).collect()
        })
        .collect();

    let mut buckets: Vec<Vec<u64>> = vec![vec![]];

    for col_str in columns.iter() {
        let cleaned = col_str.trim();
        if cleaned.is_empty() {
            buckets.push(vec![]);
            continue;
        }

        let number: u64 =
            cleaned.parse().map_err(|e| format!("Could not parse '{}': {}", cleaned, e))?;

        buckets.last_mut().ok_or("Could not get the last bucket")?.push(number);
    }

    Ok((buckets, operators))
}

fn apply_operators(numbers: &Vec<Vec<u64>>, operators: &Vec<&str>) -> Result<Vec<u64>, String> {
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

    results
}

fn part1(input: &str) -> Result<u64, String> {
    let (numbers, operators) = parse_input_row_wise(input)?;
    let results = apply_operators(&numbers, &operators);
    let sum = results?.iter().sum();
    Ok(sum)
}

fn part2(input: &str) -> Result<u64, String> {
    let (numbers, operators) = parse_input_column_wise(input)?;
    dbg!(&numbers);

    let results = apply_operators(&numbers, &operators);
    let sum = results?.iter().sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r##"
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"##;

    #[test]
    fn test_example_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT), Ok(4277556));
    }
    #[test]
    fn test_example_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT), Ok(3263827));
    }
}
