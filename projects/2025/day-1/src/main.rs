mod solutions;

use solutions::solution1;
use solutions::solution2;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    match read_input() {
        Ok(input) => {
            println!("Part 1: {}", solution1::solve_part1(&input));
            println!("Part 2: {}", solution2::solve_part2(&input));
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }
}

fn read_input() -> io::Result<Vec<String>> {
    let file = File::open("projects/2025/day-1/input.txt")?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}
