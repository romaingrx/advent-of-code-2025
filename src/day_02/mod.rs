use std::fs;

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test {
        "test_input.txt"
    } else {
        "input.txt"
    };
    let input = fs::read_to_string(format!("src/day_02/{}", input_file))
        .expect("Failed to read input file");

    // Parse input here
    let lines: Vec<(u64, u64)> = input
        .split(",")
        .map(|range| match range.split_once('-') {
            Some((start, end)) => (start.trim().parse().unwrap(), end.trim().parse().unwrap()),
            None => (0, 0),
        })
        .collect();

    dbg!(&lines);

    let result = match part {
        1 => part1(&lines),
        2 => part2(&lines),
        _ => {
            println!("Part {} not implemented for day 2", part);
            return;
        }
    };

    println!("Day 2 Part {}: {}", part, result);
}

fn is_repeating_twice(number: u64) -> bool {
    let string = number.to_string();
    if string.len().rem_euclid(2) != 0 {
        return false;
    }
    let (left, right) = string.split_at(string.len() / 2);
    left == right
}

fn is_repeating(number: u64) -> bool {
    let string = number.to_string();
    for len in 1..string.len() {
        if string.len().rem_euclid(len) == 0 {
            let first_part = &string[..len];
            let all_equal = string
                .as_bytes()
                .chunks(len)
                .all(|chunk| chunk == first_part.as_bytes());
            if all_equal {
                return true;
            }
        }
    }
    false
}

fn part1(ranges: &[(u64, u64)]) -> u64 {
    ranges
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&n| is_repeating_twice(n))
        .sum()
}

fn part2(ranges: &[(u64, u64)]) -> u64 {
    ranges
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&n| is_repeating(n))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let ranges = vec![(11, 22)];
        assert_eq!(part1(&ranges), 33);
    }

    #[test]
    fn test_example() {
        let ranges = vec![
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
        ];
        assert_eq!(part1(&ranges), 1227775554);
    }

    #[test]
    fn test_simple_part_2() {
        let ranges = vec![(11, 22)];
        assert_eq!(part2(&ranges), 33);
    }

    #[test]
    fn test_simple_2_part_2() {
        let ranges = vec![(2121212118, 2121212124)];
        assert_eq!(part2(&ranges), 2121212121);
    }

    #[test]
    fn test_example_part_2() {
        let ranges_extended = vec![
            (11, 22, 33),
            (95, 115, 210),
            (998, 1012, 2009),
            (1188511880, 1188511890, 1188511885),
            (222220, 222224, 222222),
            (1698522, 1698528, 0),
            (446443, 446449, 446446),
            (38593856, 38593862, 38593859),
            (565653, 565659, 565656),
            (824824821, 824824827, 824824824),
            (2121212118, 2121212124, 2121212121),
        ];
        let ranges: Vec<(u64, u64)> = ranges_extended
            .iter()
            .map(|(start, end, _)| (*start, *end))
            .collect();
        for (start, end, total) in ranges_extended {
            assert_eq!(part2(&vec![(start, end)]), total);
        }
        assert_eq!(part2(&ranges), 4174379265);
    }
}
