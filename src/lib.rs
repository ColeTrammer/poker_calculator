use std::fmt;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
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
#[repr(u8)]
pub enum Number {
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
    Ace = 14,
}

impl Number {
    /// # Panics
    ///
    /// Will panic if value is in the interval [2, 14]
    #[must_use]
    pub fn from_u8(value: u8) -> Self {
        match value {
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
            14 => Self::Ace,
            _ => panic!("Invalid card number: {}", value),
        }
    }

    #[must_use]
    pub const fn as_bit(self) -> u16 {
        1 << (self as u8)
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
pub enum HandKind {
    StraightFlush = 8,
    FourOfAKind = 7,
    FullHouse = 6,
    Flush = 5,
    Straight = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    Pair = 1,
    HighCard = 0,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandEvaluation {
    kind: HandKind,
    values: [u8; 3],
}

impl HandEvaluation {
    #[must_use]
    pub const fn new_straight_flush(high_card: Number) -> Self {
        Self {
            kind: HandKind::StraightFlush,
            values: [high_card as u8, 0, 0],
        }
    }

    #[must_use]
    pub const fn new_four_of_a_kind(high_card: Number) -> Self {
        Self {
            kind: HandKind::FourOfAKind,
            values: [high_card as u8, 0, 0],
        }
    }

    #[must_use]
    pub const fn new_full_house(high_card: Number, low_card: Number) -> Self {
        Self {
            kind: HandKind::FullHouse,
            values: [high_card as u8, low_card as u8, 0],
        }
    }

    #[must_use]
    pub const fn new_flush(high_card: Number) -> Self {
        Self {
            kind: HandKind::Flush,
            values: [high_card as u8, 0, 0],
        }
    }

    #[must_use]
    pub const fn new_straight(high_card: Number) -> Self {
        Self {
            kind: HandKind::Straight,
            values: [high_card as u8, 0, 0],
        }
    }

    #[must_use]
    pub const fn new_three_of_a_kind(high_card: Number) -> Self {
        Self {
            kind: HandKind::ThreeOfAKind,
            values: [high_card as u8, 0, 0],
        }
    }

    #[must_use]
    pub const fn new_two_pair(high_card: Number, low_card: Number, kicker: Number) -> Self {
        Self {
            kind: HandKind::TwoPair,
            values: [high_card as u8, low_card as u8, kicker as u8],
        }
    }

    #[must_use]
    pub const fn new_pair(high_card: Number, kickers: u16) -> Self {
        Self {
            kind: HandKind::Pair,
            values: [
                high_card as u8,
                (kickers >> 8) as u8,
                (kickers & 0xFF) as u8,
            ],
        }
    }

    #[must_use]
    pub const fn new_high_card(cards: u16) -> Self {
        Self {
            kind: HandKind::HighCard,
            values: [(cards >> 8) as u8, (cards & 0xFF) as u8, 0],
        }
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

    #[test]
    fn test_card_evaluations() {
        let royal_flush = HandEvaluation::new_straight_flush(Number::Ace);
        let straight_flush = HandEvaluation::new_straight_flush(Number::King);
        let ace_ten_full_house = HandEvaluation::new_full_house(Number::Ace, Number::Ten);
        let ace_nine_full_house = HandEvaluation::new_full_house(Number::Ace, Number::Nine);
        let ace_pair_ten_kicker = HandEvaluation::new_pair(
            Number::Ace,
            Number::Ten.as_bit() | Number::Eight.as_bit() | Number::Seven.as_bit(),
        );
        let ace_pair_nine_kicker = HandEvaluation::new_pair(
            Number::Ace,
            Number::Nine.as_bit() | Number::Eight.as_bit() | Number::Seven.as_bit(),
        );
        let ace_high = HandEvaluation::new_high_card(
            Number::Ace.as_bit()
                | Number::Ten.as_bit()
                | Number::Eight.as_bit()
                | Number::Seven.as_bit()
                | Number::Six.as_bit(),
        );
        let queen_high = HandEvaluation::new_high_card(
            Number::Queen.as_bit()
                | Number::Ten.as_bit()
                | Number::Eight.as_bit()
                | Number::Seven.as_bit()
                | Number::Six.as_bit(),
        );

        assert!(royal_flush > straight_flush);
        assert!(royal_flush > ace_ten_full_house);
        assert!(ace_ten_full_house == ace_ten_full_house);
        assert!(ace_ten_full_house > ace_nine_full_house);
        assert!(ace_ten_full_house > ace_high);
        assert!(ace_pair_ten_kicker > ace_pair_nine_kicker);
        assert!(ace_high > queen_high);
    }
}
