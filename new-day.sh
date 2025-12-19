#!/bin/bash

# Usage: ./new-day.sh <year> <day>
# Example: ./new-day.sh 2024 4

if [ $# -ne 2 ]; then
    echo "Usage: $0 <year> <day>"
    echo "Example: $0 2024 4"
    exit 1
fi

YEAR=$1
DAY=$2
PROJECT_PATH="projects/$YEAR/day-$DAY"
PACKAGE_NAME="day-$DAY-$YEAR"

# Check if project already exists
if [ -d "$PROJECT_PATH" ]; then
    echo "Error: Project $PROJECT_PATH already exists"
    exit 1
fi

# Create directory structure
mkdir -p "$PROJECT_PATH/src/solutions"

# Create Cargo.toml
cat > "$PROJECT_PATH/Cargo.toml" << EOF
[package]
name = "$PACKAGE_NAME"
version = "0.1.0"
edition = "2021"

[dependencies]
aoc-common = { path = "../../../common" }
EOF

# Create main.rs with a template
cat > "$PROJECT_PATH/src/main.rs" << EOF
mod solutions;

use std::time::Instant;

use aoc_common::read_input_lines;
use solutions::solution1;
use solutions::solution2;

fn main() {
    match read_input_lines("$PROJECT_PATH/input.txt") {
        Ok(input) => {
            // ---- Part 1 ----
            println!("Part 1: ");

            let start_time = Instant::now();
            let answer = solution1::solve_part1(&input);
            let end_time = Instant::now();

            println!("\tTotal time: {:?}", end_time.duration_since(start_time));
            println!("\tAnswer: {}", answer);

            // ---- Part 2 ----

            println!("Part 2: ");

            let start_time = Instant::now();
            let answer = solution2::solve_part2(&input);
            let end_time = Instant::now();

            println!("\tTotal time: {:?}", end_time.duration_since(start_time));
            println!("\tAnswer: {}", answer);
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }
}
EOF

# Create solutions/mod.rs
cat > "$PROJECT_PATH/src/solutions/mod.rs" << 'EOF'
pub mod solution1;
pub mod solution2;
EOF

# Create solutions/solution1.rs
cat > "$PROJECT_PATH/src/solutions/solution1.rs" << 'EOF'
pub fn solve_part1(instructions: &[String]) -> i32 {
    // TODO: Implement part 1
    0
}
EOF

# Create solutions/solution2.rs
cat > "$PROJECT_PATH/src/solutions/solution2.rs" << 'EOF'
pub fn solve_part2(instructions: &[String]) -> i32 {
    // TODO: Implement part 2
    0
}
EOF

# Create README.md
cat > "$PROJECT_PATH/README.md" << EOF
# Advent of Code $YEAR - Day $DAY

## Part 1

TODO: Add problem description

## Part 2

TODO: Add problem description
EOF

# Create empty input.txt
touch "$PROJECT_PATH/input.txt"

# Add to workspace members in root Cargo.toml
if grep -q "members = \[" Cargo.toml; then
    # Update the members array - add new project
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' "s|members = \[|members = [\"$PROJECT_PATH\", |" Cargo.toml
    else
        # Linux
        sed -i "s|members = \[|members = [\"$PROJECT_PATH\", |" Cargo.toml
    fi
fi

echo "Created new project: $PROJECT_PATH"
echo "Package name: $PACKAGE_NAME"
echo ""
echo "Run with: cargo run -p $PACKAGE_NAME"

