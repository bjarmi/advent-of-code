mod common;
mod solutions;

use std::time::Instant;

use aoc_common::read_input_lines;
use solutions::solution1;
use solutions::solution2;

fn main() {
    match read_input_lines("projects/2025/day-3/input.txt") {
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
