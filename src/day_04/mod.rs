use std::fs;

const NEIGHBORS: [(isize, isize); 8] =
    [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

pub fn run(part: u8, is_test: bool) {
    let input_file = if is_test { "test_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(format!("src/day_04/{}", input_file))
        .expect("Failed to read input file");

    // Parse input here
    let rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let result = match part {
        1 => part1(&rows),
        2 => part2(&rows),
        _ => {
            println!("Part {} not implemented for day 4", part);
            return;
        },
    };

    println!("Day 4 Part {}: {}", part, result);
}

fn part1(rows: &[Vec<char>]) -> usize {
    let rolls = find_rolls_to_remove(rows);
    rolls.len()
}
fn find_rolls_to_remove(rows: &[Vec<char>]) -> Vec<(usize, usize)> {
    let height = rows.len() as isize;
    let width = rows[0].len() as isize;
    let mut rolls: Vec<(usize, usize)> = vec![];
    for (i, row) in rows.iter().enumerate() {
        for (j, _) in row.iter().enumerate().filter(|(_, c)| **c == '@') {
            let n_adjacent_paper_rolls = NEIGHBORS
                .iter()
                .filter(|(dr, dc)| {
                    let (row, col) = (i as isize + dr, j as isize + dc);
                    if (0..height).contains(&row) && (0..width).contains(&col) {
                        return rows[row as usize][col as usize] == '@';
                    }
                    false
                })
                .count();
            println!("[{},{}]: {}", i, j, n_adjacent_paper_rolls);
            if n_adjacent_paper_rolls < 4 {
                rolls.push((i, j));
            }
        }
    }
    rolls
}

fn part2(rows: &[Vec<char>]) -> usize {
    let mut current_rows = rows.to_vec();
    let mut current_number_of_rolls = 0;
    loop {
        let rolls = find_rolls_to_remove(&current_rows);
        if rolls.is_empty() {
            break;
        }
        current_number_of_rolls += rolls.len();
        for (i, j) in rolls {
            current_rows[i][j] = '*';
        }
    }
    current_number_of_rolls
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_example() {}
}
