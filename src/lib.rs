use itertools::Itertools;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    /// # Safety
    ///
    /// This must be called with value in the range [0, 3]
    #[must_use]
    pub unsafe fn from_u8_unchecked(value: u8) -> Self {
        std::mem::transmute(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    /// Will panic if value is not in the interval [2, 14]
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

    /// # Safety
    ///
    /// This must be called with value in the range [2, 14]
    #[must_use]
    pub unsafe fn from_u8_unchecked(value: u8) -> Self {
        std::mem::transmute(value)
    }

    #[must_use]
    pub const fn as_bit(self) -> u16 {
        1 << (self as u8)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
        unsafe { Number::from_u8_unchecked(self.value & 0xF) }
    }

    #[must_use]
    pub fn suit(self) -> Suit {
        unsafe { Suit::from_u8_unchecked(self.value >> 4) }
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
    pub const fn new_four_of_a_kind(high_card: Number, kicker: Number) -> Self {
        Self {
            kind: HandKind::FourOfAKind,
            values: [high_card as u8, kicker as u8, 0],
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
    pub const fn new_flush(cards: u16) -> Self {
        Self {
            kind: HandKind::Flush,
            values: [(cards >> 8) as u8, (cards & 0xFF) as u8, 0],
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
    pub const fn new_three_of_a_kind(high_card: Number, kickers: u16) -> Self {
        Self {
            kind: HandKind::ThreeOfAKind,
            values: [
                high_card as u8,
                (kickers >> 8) as u8,
                (kickers & 0xFF) as u8,
            ],
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

#[must_use]
fn check_for_straight(mut card_bitset: u16) -> Option<Number> {
    // Duplicate the ace at the bottom of the bitset, if it is present.
    if card_bitset & Number::Ace.as_bit() != 0 {
        card_bitset |= 2;
    }

    // Check for straights by using 5 bit windows, and seeing if all bits
    // in the mask are present.
    let mask = 0b11111;
    for shift_index in (1..11).rev() {
        if (card_bitset & (mask << shift_index)) >> shift_index == mask {
            unsafe {
                return Some(Number::from_u8_unchecked(shift_index + 4));
            }
        }
    }
    None
}

#[must_use]
fn check_for_three_of_a_kind(count_by_number: &[i32; 15]) -> Option<Number> {
    for number in (Number::Two as u8..=Number::Ace as u8).rev() {
        if count_by_number[number as usize] == 3 {
            unsafe { return Some(Number::from_u8_unchecked(number)) }
        }
    }
    None
}

#[must_use]
fn check_for_pair(count_by_number: &[i32; 15]) -> Option<Number> {
    for number in (Number::Two as u8..=Number::Ace as u8).rev() {
        if count_by_number[number as usize] == 2 {
            unsafe { return Some(Number::from_u8_unchecked(number)) }
        }
    }
    None
}

#[must_use]
fn highest_card_in_set(cards: u16) -> Number {
    #[allow(clippy::cast_possible_truncation)]
    unsafe {
        Number::from_u8_unchecked((15 - cards.leading_zeros()) as u8)
    }
}

#[must_use]
pub fn evaluate_hand(cards: [Card; 7]) -> HandEvaluation {
    let mut count_by_suit = [0, 0, 0, 0];
    let mut count_by_number = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut number_bitset: u16 = 0;
    let mut number_by_suit_bitset: [u16; 4] = [0, 0, 0, 0];

    for card in cards {
        let (suit, number) = (card.suit(), card.number());
        count_by_suit[suit as usize] += 1;
        count_by_number[number as usize] += 1;
        number_bitset |= card.number().as_bit();
        number_by_suit_bitset[suit as usize] |= card.number().as_bit();
    }

    // Check for straight flushes.
    for suit_bitset in number_by_suit_bitset {
        if let Some(high_card) = check_for_straight(suit_bitset) {
            return HandEvaluation::new_straight_flush(high_card);
        }
    }

    // Check for four of a kind.
    for number in (Number::Two as u8..=Number::Ace as u8).rev() {
        if count_by_number[number as usize] == 4 {
            let high_card = unsafe { Number::from_u8_unchecked(number) };
            let kicker = highest_card_in_set(number_bitset & !high_card.as_bit());
            return HandEvaluation::new_four_of_a_kind(high_card, kicker);
        }
    }

    // Check for full house.
    let three_of_a_kind = check_for_three_of_a_kind(&count_by_number);
    if let Some(three_of_a_kind_number) = three_of_a_kind {
        for number in (Number::Two as u8..=Number::Ace as u8).rev() {
            if number != three_of_a_kind_number as u8 && count_by_number[number as usize] >= 2 {
                unsafe {
                    return HandEvaluation::new_full_house(
                        three_of_a_kind_number,
                        Number::from_u8_unchecked(number),
                    );
                }
            }
        }
    }

    // Check for flush.
    for suit in 0..4 {
        if count_by_suit[suit] >= 5 {
            let mut suited_cards = number_by_suit_bitset[suit];
            while count_by_suit[suit] > 5 {
                suited_cards &= suited_cards - 1;
                count_by_suit[suit] -= 1;
            }
            return HandEvaluation::new_flush(suited_cards);
        }
    }

    // Check for straight.
    if let Some(high_card) = check_for_straight(number_bitset) {
        return HandEvaluation::new_straight(high_card);
    }

    // Check for three of a kind.
    if let Some(high_card) = three_of_a_kind {
        let mut kickers = number_bitset;
        kickers &= !high_card.as_bit();
        kickers &= kickers - 1;
        kickers &= kickers - 1;
        return HandEvaluation::new_three_of_a_kind(high_card, kickers);
    }

    // Check for two pair and pair.
    if let Some(high_card) = check_for_pair(&count_by_number) {
        for number in (Number::Two as u8..high_card as u8).rev() {
            if count_by_number[number as usize] == 2 {
                let low_card = unsafe { Number::from_u8_unchecked(number) };

                let mut bitset = number_bitset;
                bitset &= !high_card.as_bit();
                bitset &= !low_card.as_bit();

                let kicker = highest_card_in_set(bitset);
                return HandEvaluation::new_two_pair(high_card, low_card, kicker);
            }
        }

        // There is only a single pair.
        // So, remove the bottom 2 cards from the hand and return.
        let mut kickers = number_bitset;
        kickers &= !high_card.as_bit();
        kickers &= kickers - 1;
        kickers &= kickers - 1;
        return HandEvaluation::new_pair(high_card, kickers);
    }

    // At this point, the only thing left is a high card hand.
    // So, remove the bottom 2 cards from the hand and return.
    let mut five_highest_cards = number_bitset;
    five_highest_cards &= five_highest_cards - 1;
    five_highest_cards &= five_highest_cards - 1;
    HandEvaluation::new_high_card(five_highest_cards)
}

pub struct ComputeResult {
    pub win_count: u64,
    pub loss_count: u64,
    pub tie_count: u64,
    pub count: u64,
}

#[must_use]
pub fn compute_result(hand1: [Card; 2], hand2: [Card; 2]) -> ComputeResult {
    let mut deck = vec![];
    for suit in 0..4 {
        for number in Number::Two as u8..=Number::Ace as u8 {
            let card = Card::new(Suit::from_u8(suit), Number::from_u8(number));
            if card != hand1[0] && card != hand1[1] && card != hand2[0] && card != hand2[1] {
                deck.push(card);
            }
        }
    }

    let mut tie_count = 0;
    let mut win_count = 0;
    let mut loss_count = 0;
    let mut count = 0;

    for (c1, c2, c3, c4, c5) in deck.into_iter().tuple_combinations() {
        let hand_a = [c1, c2, c3, c4, c5, hand1[0], hand1[1]];
        let hand_b = [c1, c2, c3, c4, c5, hand2[0], hand2[1]];

        let a_result = evaluate_hand(hand_a);
        let b_result = evaluate_hand(hand_b);
        match a_result.cmp(&b_result) {
            std::cmp::Ordering::Equal => tie_count += 1,
            std::cmp::Ordering::Greater => win_count += 1,
            std::cmp::Ordering::Less => loss_count += 1,
        }
        count += 1;
    }

    ComputeResult {
        win_count,
        loss_count,
        tie_count,
        count,
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

    #[test]
    fn test_straight_check() {
        let ace_high_mask = Number::Ace.as_bit()
            | Number::King.as_bit()
            | Number::Queen.as_bit()
            | Number::Jack.as_bit()
            | Number::Ten.as_bit();
        let five_high_mask = Number::Ace.as_bit()
            | Number::Two.as_bit()
            | Number::Three.as_bit()
            | Number::Four.as_bit()
            | Number::Five.as_bit();

        assert_eq!(check_for_straight(ace_high_mask), Some(Number::Ace));
        assert_eq!(check_for_straight(five_high_mask), Some(Number::Five));
    }

    #[test]
    fn test_hand_evaluator() {
        let royal_flush = [
            Card::new(Suit::Clubs, Number::Ace),
            Card::new(Suit::Clubs, Number::King),
            Card::new(Suit::Clubs, Number::Queen),
            Card::new(Suit::Clubs, Number::Jack),
            Card::new(Suit::Clubs, Number::Ten),
            Card::new(Suit::Hearts, Number::Eight),
            Card::new(Suit::Hearts, Number::Five),
        ];

        let four_of_a_kind = [
            Card::new(Suit::Clubs, Number::Ace),
            Card::new(Suit::Clubs, Number::Ten),
            Card::new(Suit::Spades, Number::Ten),
            Card::new(Suit::Hearts, Number::Ten),
            Card::new(Suit::Diamonds, Number::Ten),
            Card::new(Suit::Hearts, Number::Eight),
            Card::new(Suit::Hearts, Number::Five),
        ];

        let full_house = [
            Card::new(Suit::Clubs, Number::King),
            Card::new(Suit::Hearts, Number::King),
            Card::new(Suit::Clubs, Number::Nine),
            Card::new(Suit::Clubs, Number::Eight),
            Card::new(Suit::Spades, Number::Eight),
            Card::new(Suit::Hearts, Number::Eight),
            Card::new(Suit::Hearts, Number::Five),
        ];

        let ace_high_flush = [
            Card::new(Suit::Clubs, Number::Ace),
            Card::new(Suit::Clubs, Number::King),
            Card::new(Suit::Clubs, Number::Queen),
            Card::new(Suit::Clubs, Number::Jack),
            Card::new(Suit::Clubs, Number::Nine),
            Card::new(Suit::Clubs, Number::Eight),
            Card::new(Suit::Clubs, Number::Five),
        ];

        let king_high_flush = [
            Card::new(Suit::Clubs, Number::King),
            Card::new(Suit::Clubs, Number::Queen),
            Card::new(Suit::Clubs, Number::Jack),
            Card::new(Suit::Clubs, Number::Nine),
            Card::new(Suit::Clubs, Number::Eight),
            Card::new(Suit::Diamonds, Number::Eight),
            Card::new(Suit::Clubs, Number::Five),
        ];

        let straight = [
            Card::new(Suit::Clubs, Number::Ace),
            Card::new(Suit::Clubs, Number::King),
            Card::new(Suit::Clubs, Number::Queen),
            Card::new(Suit::Clubs, Number::Jack),
            Card::new(Suit::Hearts, Number::Ten),
            Card::new(Suit::Hearts, Number::Eight),
            Card::new(Suit::Hearts, Number::Five),
        ];

        let three_of_a_kind = [
            Card::new(Suit::Clubs, Number::Ace),
            Card::new(Suit::Hearts, Number::King),
            Card::new(Suit::Clubs, Number::Nine),
            Card::new(Suit::Clubs, Number::Eight),
            Card::new(Suit::Spades, Number::Eight),
            Card::new(Suit::Hearts, Number::Eight),
            Card::new(Suit::Hearts, Number::Five),
        ];

        let two_pair_ace_high = [
            Card::new(Suit::Diamonds, Number::Ace),
            Card::new(Suit::Clubs, Number::King),
            Card::new(Suit::Diamonds, Number::King),
            Card::new(Suit::Diamonds, Number::Ten),
            Card::new(Suit::Clubs, Number::Nine),
            Card::new(Suit::Clubs, Number::Six),
            Card::new(Suit::Hearts, Number::Six),
        ];
        let two_pair_queen_high = [
            Card::new(Suit::Clubs, Number::King),
            Card::new(Suit::Diamonds, Number::King),
            Card::new(Suit::Diamonds, Number::Queen),
            Card::new(Suit::Diamonds, Number::Ten),
            Card::new(Suit::Clubs, Number::Nine),
            Card::new(Suit::Clubs, Number::Six),
            Card::new(Suit::Hearts, Number::Six),
        ];
        let ace_pair = [
            Card::new(Suit::Clubs, Number::Ace),
            Card::new(Suit::Diamonds, Number::Ace),
            Card::new(Suit::Clubs, Number::King),
            Card::new(Suit::Diamonds, Number::Ten),
            Card::new(Suit::Clubs, Number::Nine),
            Card::new(Suit::Clubs, Number::Eight),
            Card::new(Suit::Hearts, Number::Six),
        ];
        let ace_pair_lower_kicker = [
            Card::new(Suit::Clubs, Number::Ace),
            Card::new(Suit::Diamonds, Number::Ace),
            Card::new(Suit::Diamonds, Number::Queen),
            Card::new(Suit::Diamonds, Number::Ten),
            Card::new(Suit::Clubs, Number::Nine),
            Card::new(Suit::Clubs, Number::Eight),
            Card::new(Suit::Hearts, Number::Six),
        ];
        let queen_pair = [
            Card::new(Suit::Clubs, Number::Ace),
            Card::new(Suit::Clubs, Number::Queen),
            Card::new(Suit::Diamonds, Number::Queen),
            Card::new(Suit::Diamonds, Number::Ten),
            Card::new(Suit::Clubs, Number::Nine),
            Card::new(Suit::Clubs, Number::Eight),
            Card::new(Suit::Hearts, Number::Six),
        ];

        let ace_high = [
            Card::new(Suit::Clubs, Number::Ace),
            Card::new(Suit::Clubs, Number::King),
            Card::new(Suit::Diamonds, Number::Queen),
            Card::new(Suit::Diamonds, Number::Ten),
            Card::new(Suit::Clubs, Number::Nine),
            Card::new(Suit::Clubs, Number::Eight),
            Card::new(Suit::Hearts, Number::Six),
        ];

        let ace_high_slightly_lower = [
            Card::new(Suit::Clubs, Number::Ace),
            Card::new(Suit::Clubs, Number::King),
            Card::new(Suit::Diamonds, Number::Queen),
            Card::new(Suit::Diamonds, Number::Ten),
            Card::new(Suit::Clubs, Number::Nine),
            Card::new(Suit::Clubs, Number::Eight),
            Card::new(Suit::Hearts, Number::Five),
        ];

        let king_high = [
            Card::new(Suit::Clubs, Number::King),
            Card::new(Suit::Diamonds, Number::Queen),
            Card::new(Suit::Diamonds, Number::Ten),
            Card::new(Suit::Clubs, Number::Nine),
            Card::new(Suit::Clubs, Number::Eight),
            Card::new(Suit::Hearts, Number::Six),
            Card::new(Suit::Clubs, Number::Four),
        ];

        assert!(evaluate_hand(royal_flush) == HandEvaluation::new_straight_flush(Number::Ace));
        assert!(
            evaluate_hand(four_of_a_kind)
                == HandEvaluation::new_four_of_a_kind(Number::Ten, Number::Ace)
        );
        assert!(
            evaluate_hand(full_house)
                == HandEvaluation::new_full_house(Number::Eight, Number::King)
        );
        assert!(
            evaluate_hand(ace_high_flush)
                == HandEvaluation::new_flush(
                    Number::Ace.as_bit()
                        | Number::King.as_bit()
                        | Number::Queen.as_bit()
                        | Number::Jack.as_bit()
                        | Number::Nine.as_bit()
                )
        );
        assert!(
            evaluate_hand(king_high_flush)
                == HandEvaluation::new_flush(
                    Number::King.as_bit()
                        | Number::Queen.as_bit()
                        | Number::Jack.as_bit()
                        | Number::Nine.as_bit()
                        | Number::Eight.as_bit()
                )
        );
        assert!(evaluate_hand(straight) == HandEvaluation::new_straight(Number::Ace));
        assert!(
            evaluate_hand(three_of_a_kind)
                == HandEvaluation::new_three_of_a_kind(
                    Number::Eight,
                    Number::Ace.as_bit() | Number::King.as_bit()
                )
        );
        assert!(
            evaluate_hand(ace_pair)
                == HandEvaluation::new_pair(
                    Number::Ace,
                    Number::King.as_bit() | Number::Ten.as_bit() | Number::Nine.as_bit()
                )
        );
        assert!(
            evaluate_hand(ace_pair_lower_kicker)
                == HandEvaluation::new_pair(
                    Number::Ace,
                    Number::Queen.as_bit() | Number::Ten.as_bit() | Number::Nine.as_bit()
                )
        );
        assert!(
            evaluate_hand(queen_pair)
                == HandEvaluation::new_pair(
                    Number::Queen,
                    Number::Ace.as_bit() | Number::Ten.as_bit() | Number::Nine.as_bit()
                )
        );
        assert!(
            evaluate_hand(two_pair_ace_high)
                == HandEvaluation::new_two_pair(Number::King, Number::Six, Number::Ace)
        );
        assert!(
            evaluate_hand(two_pair_queen_high)
                == HandEvaluation::new_two_pair(Number::King, Number::Six, Number::Queen)
        );
        assert!(
            evaluate_hand(ace_high)
                == HandEvaluation::new_high_card(
                    Number::Ace.as_bit()
                        | Number::King.as_bit()
                        | Number::Queen.as_bit()
                        | Number::Ten.as_bit()
                        | Number::Nine.as_bit()
                )
        );
        assert!(
            evaluate_hand(ace_high_slightly_lower)
                == HandEvaluation::new_high_card(
                    Number::Ace.as_bit()
                        | Number::King.as_bit()
                        | Number::Queen.as_bit()
                        | Number::Ten.as_bit()
                        | Number::Nine.as_bit()
                )
        );
        assert!(
            evaluate_hand(king_high)
                == HandEvaluation::new_high_card(
                    Number::King.as_bit()
                        | Number::Queen.as_bit()
                        | Number::Ten.as_bit()
                        | Number::Nine.as_bit()
                        | Number::Eight.as_bit()
                )
        );

        let all_hands = [
            evaluate_hand(royal_flush),
            evaluate_hand(four_of_a_kind),
            evaluate_hand(full_house),
            evaluate_hand(ace_high_flush),
            evaluate_hand(king_high_flush),
            evaluate_hand(straight),
            evaluate_hand(three_of_a_kind),
            evaluate_hand(two_pair_ace_high),
            evaluate_hand(two_pair_queen_high),
            evaluate_hand(ace_pair),
            evaluate_hand(ace_pair_lower_kicker),
            evaluate_hand(queen_pair),
            evaluate_hand(ace_high),
            evaluate_hand(ace_high_slightly_lower),
            evaluate_hand(king_high),
        ];
        assert!((0..all_hands.len() - 1).all(|i| all_hands[i] >= all_hands[i + 1]));
    }
}
