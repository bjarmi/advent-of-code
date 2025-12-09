#[derive(PartialEq, PartialOrd, Copy, Clone)]
#[repr(u8)]
pub enum Battery {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl Battery {
    pub fn from_char(joltage: char) -> Self {
        match joltage {
            '1' => Battery::One,
            '2' => Battery::Two,
            '3' => Battery::Three,
            '4' => Battery::Four,
            '5' => Battery::Five,
            '6' => Battery::Six,
            '7' => Battery::Seven,
            '8' => Battery::Eight,
            '9' => Battery::Nine,
            _ => panic!("Invalid joltage: {}", joltage),
        }
    }
}
