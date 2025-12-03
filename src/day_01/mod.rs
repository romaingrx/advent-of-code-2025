use std::fs;

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test {
        "test_input.txt"
    } else {
        "input.txt"
    };
    let input = fs::read_to_string(format!("src/day_01/{}", input_file))
        .expect("Failed to read input file");

    let rotations: Vec<i32> = input
        .lines()
        .into_iter()
        .map(|line| {
            let (rotation_str, number_str) = line.split_at(1);
            let rotation = rotation_str.chars().next().expect("Invalid rotation");
            let number = number_str.parse::<i32>().expect("Invalid number");
            match rotation {
                'R' => number,
                'L' => -number,
                _ => panic!(),
            }
        })
        .collect();

    let result = match part {
        1 => part1(rotations.clone()),
        _ => {
            println!("Part {} not implemented for day 1", part);
            return;
        }
    };

    println!("Day 1 Part {}: {}", part, result);
}

fn part1(rotations: Vec<i32>) -> i32 {
    let starts_at: i32 = 50;

    let mut count: i32 = 0;
    std::iter::once(starts_at)
        .chain(rotations.into_iter())
        .reduce(|a, b| {
            let result = (a + b).rem_euclid(100);
            if result == 0 {
                count += 1;
            }
            result
        });

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let rotations = vec![1, 5, -56];
        assert_eq!(part1(rotations), 1);
    }

    #[test]
    fn test_example() {
        let rotations = vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
        assert_eq!(part1(rotations), 3);
    }
}
