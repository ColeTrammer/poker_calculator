use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Suit {
    Hearts = 0,
    Diamonds = 1,
    Clubs = 2,
    Spades = 3,
}

impl Suit {
    /// # Panics
    ///
    /// Will panic if value is in the interval [0, 3]
    #[must_use]
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Hearts,
            1 => Self::Diamonds,
            2 => Self::Clubs,
            3 => Self::Spades,
            _ => panic!("Invalid suite number: {}", value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Number {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

impl Number {
    /// # Panics
    ///
    /// Will panic if value is in the interval [1, 13]
    #[must_use]
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Ace,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            _ => panic!("Invalid card number: {}", value),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Card {
    value: u8,
}

impl Card {
    #[must_use]
    pub const fn new(suit: Suit, number: Number) -> Self {
        let value = (suit as u8) << 4 | (number as u8);
        Self { value }
    }

    #[must_use]
    pub fn number(self) -> Number {
        Number::from_u8(self.value & 0xF)
    }

    #[must_use]
    pub fn suit(self) -> Suit {
        Suit::from_u8(self.value >> 4)
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Card")
            .field("suit", &self.suit())
            .field("number", &self.number())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_format_string() {
        assert_eq!(
            format!("{:?}", Card::new(Suit::Hearts, Number::Ace)),
            "Card { suit: Hearts, number: Ace }"
        )
    }
}
