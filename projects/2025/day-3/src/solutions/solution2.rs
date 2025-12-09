use std::time::Instant;

use crate::common::bank::Bank;

pub fn solve_part2(_input: &[String]) -> u64 {
    let banks: Vec<Bank> = _input.iter().map(|line| Bank::new(line)).collect();
    let num_batteries = 12;

    let start_time = Instant::now();

    let total_output_joltage = banks
        .iter()
        .map(|bank| bank.get_max_joltage_with_n_batteries(num_batteries))
        .sum();

    let end_time = Instant::now();
    println!(
        "\tCalculation time: {:?}",
        end_time.duration_since(start_time)
    );

    total_output_joltage
}
