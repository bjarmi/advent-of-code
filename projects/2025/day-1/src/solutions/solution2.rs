pub fn solve_part2(instructions: &[String]) -> u32 {
    let mut current_position: i32 = 50;
    let mut zero_count: u32 = 0;
    let mut new_position: i32 = 0;

    for instruction in instructions {
        let direction = instruction.chars().next().unwrap();
        let distance = instruction[1..].parse::<i32>().unwrap();
        let mut zero_change_count = 0;

        match direction {
            'L' => {
                let full_rotations = distance / 100;
                zero_change_count += full_rotations as u32;

                let extra_distance_after_full_rotations = distance % 100;

                new_position = ((current_position) - extra_distance_after_full_rotations) % 100;
                if new_position < 0 {
                    if current_position != 0 {
                        zero_change_count += 1;
                    }
                    new_position = 100 + new_position;
                } else if new_position == 0 {
                    zero_change_count += 1;
                }
            }
            'R' => {
                let full_rotations = ((current_position) + distance) / 100;
                zero_change_count += full_rotations as u32;

                new_position = ((current_position) + distance) % 100;
            }
            _ => panic!("Invalid direction: {}", direction),
        }

        // After this line, current_position == new_position
        current_position = new_position.clone();
        zero_count += zero_change_count;
    }

    zero_count as u32
}
