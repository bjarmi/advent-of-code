use crate::common::battery::Battery;

pub struct Bank {
    batteries: Vec<Battery>,
}

impl Bank {
    pub fn new(input: &str) -> Self {
        let batteries = input.chars().map(|c| Battery::from_char(c)).collect();
        Self { batteries }
    }

    /// Get the maximum joltage that can be produced by turning on exactly two batteries in the bank
    pub fn get_max_joltage(&self) -> u8 {
        let mut highest_joltage_left: Option<&Battery> = None;
        let mut highest_joltage_right: Option<&Battery> = None;

        let length = self.batteries.len();

        for (idx, battery) in self.batteries.iter().enumerate() {
            if idx != length - 1 {
                match highest_joltage_left {
                    None => highest_joltage_left = Some(battery),
                    Some(current) if battery > current => {
                        highest_joltage_left = Some(battery);
                        highest_joltage_right = None; // Reset right side
                        continue;
                    }
                    _ => {}
                }
            }

            if idx != 0 {
                match highest_joltage_right {
                    None => highest_joltage_right = Some(battery),
                    Some(current) if battery > current => highest_joltage_right = Some(battery),
                    _ => {}
                }
            }
        }

        (*highest_joltage_left.unwrap() as u8) * 10 + (*highest_joltage_right.unwrap() as u8)
    }

    /// Get the maximum joltage that can be produced by turning on exactly num_batteries batteries in the bank
    pub fn get_max_joltage_with_n_batteries(&self, num_batteries: usize) -> u64 {
        let mut batteries_to_turn_on: Vec<Option<&Battery>> = vec![None; num_batteries];

        let length = self.batteries.len();

        for (idx, battery) in self.batteries.iter().enumerate() {
            let mut reset_rest = false;

            let starting_idx = if (length - idx) < num_batteries {
                num_batteries - (length - idx)
            } else {
                0
            };

            for turn_on_battery in batteries_to_turn_on[starting_idx..].iter_mut() {
                if reset_rest {
                    *turn_on_battery = None;
                    continue;
                }

                match turn_on_battery {
                    None => {
                        *turn_on_battery = Some(battery);
                        break;
                    }
                    Some(current) if battery > current => {
                        *turn_on_battery = Some(battery);
                        reset_rest = true;
                    }
                    _ => {}
                }
            }
        }

        let mut result: u64 = 0;
        for battery in batteries_to_turn_on {
            result = result * 10 + (*battery.unwrap() as u64);
        }
        result
    }
}
