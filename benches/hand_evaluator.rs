#![feature(test)]

extern crate test;

#[cfg(test)]
mod bench {
    use itertools::Itertools;

    use ::poker_calculator::{evaluate_hand, Card, Number, Suit};

    #[bench]
    fn bench_evaluate_hand_1000000(b: &mut test::Bencher) {
        let mut deck = vec![];
        for suit in 0..4 {
            for number in Number::Two as u8..=Number::Ace as u8 {
                deck.push(Card::new(Suit::from_u8(suit), Number::from_u8(number)));
            }
        }

        let all_hands = deck
            .into_iter()
            .tuple_combinations()
            .map(|(a, b, c, d, e, f, g)| [a, b, c, d, e, f, g])
            .take(1000000)
            .collect::<Vec<_>>();

        b.iter(|| {
            for hand in &all_hands {
                test::black_box(evaluate_hand(*hand));
            }
        })
    }
}
