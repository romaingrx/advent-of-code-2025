use std::fs;

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test { "test_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(format!("src/day_05/{}", input_file))
        .expect("Failed to read input file");

    let (ranges, items) = parse_input(&input);

    let result = match part {
        1 => part1(&ranges, items),
        _ => {
            println!("Part {} not implemented for day 5", part);
            return;
        },
    };

    println!("Day 5 Part {}: {}", part, result);
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (ranges_str, items_str) = input.split_once("\n\n").unwrap();

    let ranges = ranges_str
        .lines()
        .filter_map(|line| {
            let (start, end) = line.split_once('-')?;
            Some((start.parse().ok()?, end.parse().ok()?))
        })
        .collect();

    let items = items_str.lines().filter_map(|line| line.parse().ok()).collect();

    (ranges, items)
}

fn list_fresh_items(fresh_ranges: &[(usize, usize)], items: Vec<usize>) -> Vec<usize> {
    items
        .into_iter()
        .filter(|item| {
            for (start, end) in fresh_ranges {
                if start <= item && item <= end {
                    return true;
                }
            }
            false
        })
        .collect()
}

fn part1(fresh_ranges: &[(usize, usize)], items: Vec<usize>) -> usize {
    list_fresh_items(fresh_ranges, items).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let fresh_ranges = [(3, 5), (10, 14), (16, 20), (12, 18)];
        let items = vec![1, 5, 8, 11, 17, 32];
        assert_eq!(part1(&fresh_ranges, items), 3);
    }
}
