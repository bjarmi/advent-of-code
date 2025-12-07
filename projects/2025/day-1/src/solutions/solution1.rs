pub fn solve_part1(instructions: &[String]) -> u32 {
    let mut current_position: i32 = 50;
    let mut zero_count: u32 = 0;

    for instruction in instructions {
        let direction = instruction.chars().next().unwrap();
        let distance = instruction[1..].parse::<i32>().unwrap();

        match direction {
            'L' => {
                current_position = ((current_position) - distance) % 100;
                if current_position < 0 {
                    current_position = 100 + current_position;
                }
            }
            'R' => {
                current_position = ((current_position) + distance) % 100;
            }
            _ => {
                panic!("Invalid direction: {}", direction);
            }
        }
        if current_position == 0 {
            zero_count += 1;
        }
    }

    zero_count as u32
}
