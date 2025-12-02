#!/bin/bash

# Script to create a new Advent of Code day
# Usage: ./new_day.sh <day_number>

if [ $# -ne 1 ]; then
    echo "Usage: $0 <day_number>"
    echo "Example: $0 2"
    exit 1
fi

DAY=$1
DAY_DIR="src/day_$(printf "%02d" $DAY)"
MOD_NAME="day_$(printf "%02d" $DAY)"

# Create directory
mkdir -p "$DAY_DIR"

# Create mod.rs
cat > "$DAY_DIR/mod.rs" << EOF
use std::fs;

pub fn run(is_test: bool) {
    let input_file = if is_test { "test_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(format!("src/${MOD_NAME}/{}", input_file))
        .expect("Failed to read input file");

    // Parse input here
    // let lines: Vec<&str> = input.lines().collect();

    let result = solve(&input);
    println!("Day ${DAY}: {}", result);
}

fn solve(input: &str) -> i32 {
    // Implement your solution here
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "";
        assert_eq!(solve(input), 0);
    }
}
EOF

# Create input.txt placeholder
echo "# Put your input here" > "$DAY_DIR/input.txt"

# Create test_input.txt placeholder
echo "# Put your test input here" > "$DAY_DIR/test_input.txt"

# Create README.md
cat > "$DAY_DIR/README.md" << EOF
# Day ${DAY}: [Title]

[Problem description will go here]

## Part 1

[Part 1 description]

## Part 2

[Part 2 description]

## Input

[Input description]
EOF

# Update main.rs to include the new module
MAIN_RS="src/main.rs"
if grep -q "mod ${MOD_NAME};" "$MAIN_RS"; then
    echo "Module ${MOD_NAME} already exists in main.rs"
else
    # Add the mod declaration after the existing day_01 mod
    sed -i.bak "/mod day_01;/a\\
mod ${MOD_NAME};" "$MAIN_RS"
fi

# Update the match statement in main.rs
if grep -q "\"${DAY}\" => ${MOD_NAME}::run(is_test)," "$MAIN_RS"; then
    echo "Day ${DAY} already exists in main.rs match statement"
else
    # Add the case to the match statement
    sed -i.bak "s/        \"1\" => day_01::run(is_test),/        \"1\" => day_01::run(is_test),\\
        \"${DAY}\" => ${MOD_NAME}::run(is_test),/" "$MAIN_RS"
fi

echo "Created day ${DAY} in ${DAY_DIR}"
echo "Don't forget to:"
echo "1. Update the README.md with the actual problem description"
echo "2. Add your real input to ${DAY_DIR}/input.txt"
echo "3. Add test input to ${DAY_DIR}/test_input.txt"
echo "4. Implement the solve function in ${DAY_DIR}/mod.rs"
echo "5. Run with: cargo run -- ${DAY}"
