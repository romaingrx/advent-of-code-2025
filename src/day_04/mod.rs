use std::fs;

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test { "test_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(format!("src/day_04/{}", input_file))
        .expect("Failed to read input file");

    // Parse input here
    let rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let result = match part {
        1 => part1(&rows),
        2 => part2(&input),
        _ => {
            println!("Part {} not implemented for day 4", part);
            return;
        },
    };

    println!("Day 4 Part {}: {}", part, result);
}

fn part1(rows: &[Vec<char>]) -> i32 {
    let height = rows.len() as i32;
    let width = rows[0].len() as i32;
    rows.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().map(move |(j, _c)| {
                if rows[i][j] != '@' {
                    return 0;
                }
                let deltas = [
                    (-1i32, -1i32),
                    (-1i32, 0i32),
                    (-1i32, 1i32),
                    (0i32, -1i32),
                    (0i32, 1i32),
                    (1i32, -1i32),
                    (1i32, 0i32),
                    (1i32, 1i32),
                ];
                let n_adjacent_paper_rolls: i32 = deltas
                    .iter()
                    .map(|(dx, dy)| {
                        let x = (i as i32) + dx;
                        let y = (j as i32) + dy;
                        if x < 0 || y < 0 || x >= height || y >= width {
                            return 0;
                        }
                        (rows[x as usize][y as usize] == '@') as i32
                    })
                    .sum();
                println!("[{},{}]: {}", i, j, n_adjacent_paper_rolls);
                (n_adjacent_paper_rolls < 4) as i32
            })
        })
        .sum()
}

fn part2(_input: &str) -> i32 {
    // TODO: Implement part 2
    println!("Part 2 not yet implemented");
    0
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_example() {}
}
