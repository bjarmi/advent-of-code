use std::io::BufReader;
use std::io::Bytes;

pub struct InvalidIdCumSumPart2 {
    pub sum: u64,

    // Current parced range
    current_from: u64,
    current_to: u64,

    parsing_from: bool,
}

impl InvalidIdCumSumPart2 {
    pub fn new(input: Bytes<BufReader<std::fs::File>>) -> Self {
        let mut instance = Self {
            sum: 0,
            current_from: 0,
            current_to: 0,
            parsing_from: true,
        };

        for byte in input {
            match byte {
                Ok(b) => {
                    instance.add_byte(b);
                }
                Err(e) => {
                    // Sould never happen, but just in case
                    instance.add_byte(',' as u8);
                    return instance;
                }
            }
        }
        instance.add_byte(',' as u8);

        instance
    }

    fn add_byte(&mut self, byte: u8) {
        let ch = byte as char;

        if ch == ',' {
            self.handle_end_of_current_range();
            return;
        }
        if ch == '-' {
            self.parsing_from = false;
            return;
        }

        match self.parsing_from {
            true => {
                self.current_from = self.current_from * 10 + ch.to_digit(10).unwrap() as u64;
            }
            false => {
                self.current_to = self.current_to * 10 + ch.to_digit(10).unwrap() as u64;
            }
        }
    }

    fn handle_end_of_current_range(&mut self) {
        // Check if current range contains any invalid ids

        for id in self.current_from..=self.current_to {
            if InvalidIdCumSumPart2::is_invalid_id_alexander(id) {
                self.sum += id as u64;
            }
        }

        // Reset for next range
        self.parsing_from = true;
        self.current_from = 0;
        self.current_to = 0;
    }

    fn is_invalid_id(id: u64) -> bool {
        let id_string = id.to_string();
        let length = id_string.len();

        let half_length = length / 2;
        let mut is_invalid = false;

        for pattern_size in 1..=half_length {
            // Skip if the pattern size is not a divisor of the length
            if length % pattern_size != 0 {
                continue;
            }

            let pattern = &id_string[0..pattern_size];
            let mut idx = pattern_size;

            while idx + pattern_size <= length {
                let current_pattern = &id_string[idx..idx + pattern_size];
                if current_pattern == pattern {
                    is_invalid = true;
                    idx += pattern_size;
                } else {
                    is_invalid = false;
                    break;
                }
            }

            // Early return if we found an invalid pattern
            if is_invalid {
                return true;
            }
        }

        return is_invalid;
    }

    // Alexander's solution
    fn is_invalid_id_alexander(id: u64) -> bool {
        let id_string = id.to_string();
        let double_string = id_string.repeat(2);
        let stripped_string = &double_string[1..double_string.len()];

        return !stripped_string.contains(&id_string);
    }
}
// 15704845910
