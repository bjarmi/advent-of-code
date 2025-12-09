use std::time::Instant;

use crate::common::bank::Bank;

pub fn solve_part1(_input: &[String]) -> u64 {
    let banks: Vec<Bank> = _input.iter().map(|line| Bank::new(line)).collect();

    let start_time = Instant::now();

    let total_output_joltage: u64 = banks.iter().map(|bank| bank.get_max_joltage() as u64).sum();

    let end_time = Instant::now();

    println!(
        "\tCalculation time: {:?}",
        end_time.duration_since(start_time)
    );

    total_output_joltage
}
