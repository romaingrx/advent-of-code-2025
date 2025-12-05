use std::fs;

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test { "test_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(format!("src/day_05/{}", input_file))
        .expect("Failed to read input file");

    let (ranges, items) = parse_input(&input);

    let result = match part {
        1 => part1(&ranges, items),
        2 => part2(&ranges),
        _ => {
            println!("Part {} not implemented for day 5", part);
            return;
        },
    };

    println!("Day 5 Part {}: {}", part, result);
}

fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
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

fn part1(fresh_ranges: &[(i64, i64)], items: Vec<i64>) -> i64 {
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
        .count() as i64
}

fn part2(fresh_ranges: &[(i64, i64)]) -> i64 {
    let mut ranges = fresh_ranges.to_vec();
    ranges.sort_unstable();

    let merged = ranges.into_iter().fold(Vec::<(i64, i64)>::new(), |mut acc, (start, end)| {
        match acc.last_mut() {
            Some(last) if start <= last.1 => last.1 = last.1.max(end),
            _ => acc.push((start, end)),
        }
        acc
    });

    merged.iter().map(|(s, e)| e - s + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let fresh_ranges = [(3, 5), (10, 14), (16, 20), (12, 18)];
        let items = vec![1, 5, 8, 11, 17, 32];
        assert_eq!(part1(&fresh_ranges, items), 3);
    }

    #[test]
    fn test_example_part_2() {
        let fresh_ranges = [(3, 5), (10, 14), (16, 20), (12, 18)];
        assert_eq!(part2(&fresh_ranges), 14);
    }
}
