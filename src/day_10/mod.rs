use std::{fs, ops::BitXor};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
struct BitMask(u64);

impl BitMask {
    fn from_string(pattern: String) -> Option<BitMask> {
        Some(BitMask(pattern.chars().rev().fold(0, |acc, c| (acc << 1) | (c == '#') as u64)))
    }

    fn from_indices(indices: &[u64]) -> BitMask {
        BitMask(indices.iter().map(|&i| 1u64 << i).sum())
    }
}

impl BitXor for BitMask {
    type Output = BitMask;

    fn bitxor(self, other: BitMask) -> BitMask {
        BitMask(self.0 ^ other.0)
    }
}

#[derive(Debug, Clone)]
struct Puzzle {
    target_indicators: BitMask,
    buttons: Vec<BitMask>,
    #[allow(dead_code)]
    joltages: Vec<i64>,
}

fn parse_line(
    line: &str,
    re_target_indicators: &Regex,
    re_buttons: &Regex,
    re_joltages: &Regex,
) -> Option<Puzzle> {
    let target_indicators =
        BitMask::from_string(re_target_indicators.captures(line)?[1].to_string())?;

    let buttons = re_buttons
        .captures_iter(line)
        .map(|c| {
            let indices: Vec<u64> = c[1].split(',').map(|n| n.trim().parse().unwrap()).collect();
            BitMask::from_indices(&indices)
        })
        .collect();

    let joltages =
        re_joltages.captures(line)?[1].split(',').map(|n| n.trim().parse().unwrap()).collect();

    Some(Puzzle { target_indicators, buttons, joltages })
}

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test { "test_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(format!("src/day_10/{}", input_file))
        .expect("Failed to read input file");

    let re_pattern = Regex::new(r"\[([#.]+)\]").unwrap();
    let re_constraint = Regex::new(r"\(([^)]+)\)").unwrap();
    let re_values = Regex::new(r"\{([^}]+)\}").unwrap();

    let puzzles: Vec<Puzzle> = input
        .lines()
        .filter_map(|line| parse_line(line, &re_pattern, &re_constraint, &re_values))
        .collect();

    let result = match part {
        1 => part1(&puzzles),
        2 => part2(&puzzles),
        _ => {
            println!("Part {} not implemented for day 10", part);
            return;
        },
    };

    println!("Day 10 Part {}: {}", part, result);
}

fn part1(puzzles: &[Puzzle]) -> usize {
    puzzles
        .iter()
        .filter_map(|puzzle| {
            for r in 1..=puzzle.buttons.len() {
                for combo in puzzle.buttons.iter().combinations(r) {
                    let mut state = BitMask::default();
                    for cons in combo {
                        state = state ^ *cons;
                    }
                    if state == puzzle.target_indicators {
                        return Some(r);
                    }
                }
            }
            None
        })
        .sum()
}

fn part2(_puzzles: &[Puzzle]) -> usize {
    // TODO: Implement part 2
    println!("Part 2 not yet implemented");
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_puzzles() -> Vec<Puzzle> {
        [
            Puzzle {
                target_indicators: BitMask::from_string(".##.".to_string()).unwrap(),
                buttons: vec![
                    BitMask::from_indices(&[3]),
                    BitMask::from_indices(&[1, 3]),
                    BitMask::from_indices(&[2]),
                    BitMask::from_indices(&[2, 3]),
                    BitMask::from_indices(&[0, 2]),
                    BitMask::from_indices(&[0, 1]),
                ],
                joltages: vec![3, 5, 4, 7],
            },
            Puzzle {
                target_indicators: BitMask::from_string("...#..".to_string()).unwrap(),
                buttons: vec![
                    BitMask::from_indices(&[0, 2, 3, 4]),
                    BitMask::from_indices(&[2, 3]),
                    BitMask::from_indices(&[0, 4]),
                    BitMask::from_indices(&[0, 1, 2]),
                    BitMask::from_indices(&[1, 2, 3, 4]),
                ],
                joltages: vec![7, 5, 12, 7, 2],
            },
            Puzzle {
                target_indicators: BitMask::from_string(".###.#.".to_string()).unwrap(),
                buttons: vec![
                    BitMask::from_indices(&[0, 1, 2, 3, 4]),
                    BitMask::from_indices(&[0, 3, 4]),
                    BitMask::from_indices(&[0, 1, 2, 4, 5]),
                    BitMask::from_indices(&[1, 2]),
                ],
                joltages: vec![10, 11, 11, 5, 10, 5],
            },
        ]
        .to_vec()
    }

    #[test]
    fn test_example() {
        let puzzles = get_puzzles();
        assert_eq!(part1(&puzzles), 7);
    }
}
