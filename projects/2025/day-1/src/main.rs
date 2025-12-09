mod solutions;

use aoc_common::read_input_lines;
use solutions::solution1;
use solutions::solution2;

fn main() {
    match read_input_lines("projects/2025/day-1/input.txt") {
        Ok(input) => {
            println!("Part 1: {}", solution1::solve_part1(&input));
            println!("Part 2: {}", solution2::solve_part2(&input));
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }
}
