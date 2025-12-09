use std::io::BufReader;
use std::io::Bytes;

pub struct InvalidIdCumSum {
    pub sum: u64,

    // Current parced range
    current_from: u64,
    current_to: u64,

    parsing_from: bool,
}

impl InvalidIdCumSum {
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
            if InvalidIdCumSum::is_invalid_id(id) {
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
        if length % 2 != 0 {
            return false;
        }
        let half_length = length / 2;

        let left = &id_string[0..half_length];
        let right = &id_string[half_length..];

        return left == right;
    }
}
