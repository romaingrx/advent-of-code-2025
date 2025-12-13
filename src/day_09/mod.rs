use std::fs;

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test { "test_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(format!("src/day_09/{}", input_file))
        .expect("Failed to read input file");

    let points: Vec<[u64; 2]> = input
        .lines()
        .filter_map(|line| {
            let c: Vec<u64> = line.split(',').filter_map(|n| n.parse().ok()).collect();
            Some([*c.first()?, *c.get(1)?])
        })
        .collect();

    let result = match part {
        1 => part1(&points),
        2 => part2(&input),
        _ => {
            println!("Part {} not implemented for day 9", part);
            return;
        },
    };

    println!("Day 9 Part {}: {}", part, result);
}

fn part1(points: &[[u64; 2]]) -> u64 {
    let rectangles = points.iter().enumerate().flat_map(|(i, point_0)| {
        points.iter().skip(i).map(move |point_1| {
            let area: u64 = (0..2).map(|k| point_0[k].abs_diff(point_1[k]) + 1).product();
            (point_0, point_1, area)
        })
    });

    let biggest_rectangle = rectangles.max_by_key(|(_, _, area)| *area);
    match biggest_rectangle {
        Some((_, _, area)) => area,
        None => 0,
    }
}

fn part2(_input: &str) -> u64 {
    // TODO: Implement part 2
    println!("Part 2 not yet implemented");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const POINTS: [[u64; 2]; 8] =
        [[7, 1], [11, 1], [11, 7], [9, 7], [9, 5], [2, 5], [2, 3], [7, 3]];

    #[test]
    fn test_example() {
        assert_eq!(part1(&POINTS), 50);
    }
}
