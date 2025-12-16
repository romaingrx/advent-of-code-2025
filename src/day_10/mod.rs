use indicatif::{ProgressBar, ProgressStyle};
use std::{fs, ops::BitXor};

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;

const EPSILON: f64 = 1e-9;

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
    joltages: Vec<usize>,
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

struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
    dependent: Vec<usize>,   // Columns with pivots
    independent: Vec<usize>, // Free variables
}

impl Matrix {
    fn from_puzzle(puzzle: &Puzzle) -> Self {
        let rows = puzzle.joltages.len();
        let cols = puzzle.buttons.len();

        // Build augmented matrix [A | b]
        let mut data = vec![vec![0.0; cols + 1]; rows];
        for (col, &mask) in puzzle.buttons.iter().enumerate() {
            for (row, data_row) in data.iter_mut().enumerate().take(rows) {
                if (mask.0 >> row) & 1 == 1 {
                    data_row[col] = 1.0;
                }
            }
        }
        for (row, &val) in puzzle.joltages.iter().enumerate() {
            data[row][cols] = val as f64;
        }

        let mut matrix =
            Matrix { data, rows, cols, dependent: Vec::new(), independent: Vec::new() };

        matrix.gaussian_elimination();
        matrix
    }

    fn gaussian_elimination(&mut self) {
        let mut pivot = 0;

        for col in 0..self.cols {
            // Find best pivot row (largest absolute value)
            let best = (pivot..self.rows).max_by(|&a, &b| {
                self.data[a][col].abs().partial_cmp(&self.data[b][col].abs()).unwrap()
            });

            let Some(best_row) = best else { continue };

            // If best value is ~0, this column is a free variable
            if self.data[best_row][col].abs() < EPSILON {
                self.independent.push(col);
                continue;
            }

            // Swap and mark as dependent
            self.data.swap(pivot, best_row);
            self.dependent.push(col);

            // Normalize pivot row
            let pivot_val = self.data[pivot][col];
            for val in &mut self.data[pivot][col..=self.cols] {
                *val /= pivot_val;
            }

            // Eliminate column in all other rows
            for row in 0..self.rows {
                if row != pivot {
                    let factor = self.data[row][col];
                    if factor.abs() > EPSILON {
                        for c in col..=self.cols {
                            self.data[row][c] -= factor * self.data[pivot][c];
                        }
                    }
                }
            }

            pivot += 1;
        }

        // Remaining columns are free variables
        self.independent.extend(pivot..self.cols);
    }

    // Check if independent variable assignment gives valid solution
    fn valid(&self, free_vals: &[usize]) -> Option<usize> {
        let mut total = free_vals.iter().sum::<usize>();

        // Calculate each dependent variable
        for (row, _) in self.dependent.iter().enumerate() {
            let val = self.data[row][self.cols]
                - self
                    .independent
                    .iter()
                    .zip(free_vals)
                    .map(|(&col, &v)| self.data[row][col] * v as f64)
                    .sum::<f64>();

            // Must be non-negative integer
            if val < -EPSILON {
                return None;
            }
            let rounded = val.round();
            if (val - rounded).abs() > EPSILON {
                return None;
            }

            total += rounded as usize;
        }

        Some(total)
    }
}

fn search(matrix: &Matrix, idx: usize, vals: &mut [usize], best: &mut usize, max: usize) {
    if idx == matrix.independent.len() {
        if let Some(total) = matrix.valid(vals) {
            *best = (*best).min(total);
        }
        return;
    }

    let current: usize = vals[..idx].iter().sum();
    for v in 0..max {
        if current + v >= *best {
            break;
        } // Prune
        vals[idx] = v;
        search(matrix, idx + 1, vals, best, max);
    }
}

fn part2(puzzles: &[Puzzle]) -> usize {
    // Heavily inspired by https://gist.github.com/icub3d/16eea2a8b4a94d193a148fef908779a9
    let pb = ProgressBar::new(puzzles.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );

    let result: usize = puzzles
        .iter()
        .par_bridge()
        .map(|puzzle| {
            let matrix = Matrix::from_puzzle(puzzle);

            // Much better bound: sum of joltages instead of max
            let max = puzzle.joltages.iter().sum::<usize>() / matrix.independent.len().max(1);

            let mut best = usize::MAX;
            let mut vals = vec![0; matrix.independent.len()];
            search(&matrix, 0, &mut vals, &mut best, max);
            pb.inc(1);
            best
        })
        .sum();

    pb.finish_with_message("Complete");
    result
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
    fn test_example_part1() {
        let puzzles = get_puzzles();
        assert_eq!(part1(&puzzles), 7); // 2 + 3 + 2 = 7
    }

    #[test]
    fn test_example_part2() {
        let puzzles = get_puzzles();
        assert_eq!(part2(&puzzles), 33); // 10 + 12 + 11 = 33
    }
}
