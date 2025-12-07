use std::{collections::HashSet, fs};

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test { "test_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(format!("src/day_07/{}", input_file))
        .expect("Failed to read input file");

    // Parse input here
    // let lines: Vec<&str> = input.lines().collect();

    let result = match part {
        1 => part1(&input),
        _ => {
            println!("Part {} not implemented for day 7", part);
            return;
        },
    };

    println!("Day 7 Part {}: {:?}", part, result);
}

fn part1(input: &str) -> Result<u64, String> {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let (entrypoint_row, entrypoint_col) = grid
        .iter()
        .enumerate()
        .find_map(|(row_idx, line)| {
            line.iter().position(|&c| c == b'S').map(|col_idx| (row_idx, col_idx))
        })
        .ok_or("Could not find entrypoint")?;

    let width = grid[0].len();

    let mut splits: u64 = 0;
    let mut beams: HashSet<usize> = HashSet::from([entrypoint_col]);

    for line in &grid[entrypoint_row + 1..] {
        let mut next_beams: HashSet<usize> = HashSet::new();

        for beam_col_idx in beams {
            if line[beam_col_idx] == b'^' {
                splits += 1;
                if beam_col_idx > 0 {
                    next_beams.insert(beam_col_idx - 1);
                }
                if beam_col_idx + 1 < width {
                    next_beams.insert(beam_col_idx + 1);
                }
            } else {
                next_beams.insert(beam_col_idx);
            }
        }

        beams = next_beams;
    }

    Ok(splits)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r##".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"##;

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE_INPUT), Ok(21));
    }
}
