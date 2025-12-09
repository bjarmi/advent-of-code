mod solutions;

use std::time::Instant;

use aoc_common::utils::read_input_bytes;
use solutions::solution1;
use solutions::solution2;

fn main() {
    match read_input_bytes("projects/2025/day-2/input.txt") {
        Ok(input) => {
            println!("Part 1: ");
            let start_time = Instant::now();
            let invalid_id_cum_sum = solution1::InvalidIdCumSum::new(input);
            // End time stamp
            let end_time = Instant::now();
            // Print time taken
            println!("\t Cum Sum: {}", invalid_id_cum_sum.sum);
            println!("\t Time: {:?}", end_time.duration_since(start_time));
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }

    match read_input_bytes("projects/2025/day-2/input.txt") {
        Ok(input) => {
            println!("Part 2: ");
            let start_time = Instant::now();
            let invalid_id_cum_sum = solution2::InvalidIdCumSumPart2::new(input);
            // End time stamp
            let end_time = Instant::now();
            // Print time taken
            println!("\t Cum Sum: {}", invalid_id_cum_sum.sum);
            println!("\t Time: {:?}", end_time.duration_since(start_time));
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }
}
