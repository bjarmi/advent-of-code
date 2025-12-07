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
mkdir -p "$PROJECT_PATH/src"

# Create Cargo.toml
cat > "$PROJECT_PATH/Cargo.toml" << EOF
[package]
name = "$PACKAGE_NAME"
version = "0.1.0"
edition = "2021"

[dependencies]
EOF

# Create main.rs with a template
cat > "$PROJECT_PATH/src/main.rs" << 'EOF'
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    match read_input() {
        Ok(input) => {
            println!("Part 1: {}", solve_part1(&input));
            // println!("Part 2: {}", solve_part2(&input));
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }
}

fn solve_part1(input: &[String]) -> i32 {
    // TODO: Implement part 1
    0
}

fn solve_part2(input: &[String]) -> i32 {
    // TODO: Implement part 2
    0
}

fn read_input() -> io::Result<Vec<String>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    reader.lines().collect()
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

