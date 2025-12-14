use std::{fs, iter::zip};

mod utils;
use utils::{Point, Rectangle};

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test { "test_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(format!("src/day_09/{}", input_file))
        .expect("Failed to read input file");

    let points: Vec<Point> = input
        .lines()
        .filter_map(|line| {
            let c: Vec<u64> = line.split(',').filter_map(|n| n.parse().ok()).collect();
            Some(Point { x: *c.first()?, y: *c.get(1)? })
        })
        .collect();

    let result = match part {
        1 => part1(&points),
        2 => part2(&points),
        _ => {
            println!("Part {} not implemented for day 9", part);
            return;
        },
    };

    println!("Day 9 Part {}: {}", part, result);
}

fn construct_rectangles(points: &[Point]) -> Vec<Rectangle> {
    let rectangles = points.iter().enumerate().flat_map(|(i, point_0)| {
        points.iter().skip(i).map(move |point_1| Rectangle::from_points(point_0, point_1))
    });

    rectangles.collect()
}

fn part1(points: &[Point]) -> u64 {
    let rectangles = construct_rectangles(points);
    rectangles.iter().map(|rectangle| rectangle.area()).max().unwrap_or(0)
}

fn part2(points: &[Point]) -> u64 {
    let mut rectangles = construct_rectangles(points);

    rectangles.sort_by_key(|r| -(r.area() as i64));

    let lines: Vec<Rectangle> = zip(points, points.iter().cycle().skip(1))
        .take(points.len())
        .map(|(point_0, point_1)| Rectangle::from_points(point_0, point_1))
        .collect();

    rectangles
        .iter()
        .find(|rectangle| {
            let inner_rec = rectangle.inner();
            !lines.iter().any(|l| inner_rec.overlaps(l))
        })
        .map(|r| r.area())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const POINTS: [Point; 8] = [
        Point { x: 7, y: 1 },
        Point { x: 11, y: 1 },
        Point { x: 11, y: 7 },
        Point { x: 9, y: 7 },
        Point { x: 9, y: 5 },
        Point { x: 2, y: 5 },
        Point { x: 2, y: 3 },
        Point { x: 7, y: 3 },
    ];

    #[test]
    fn test_example_part_1() {
        assert_eq!(part1(&POINTS), 50);
    }

    #[test]
    fn test_example_part_2() {
        assert_eq!(part2(&POINTS), 24);
    }
}
