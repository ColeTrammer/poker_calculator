use poker_calculator::{compute_equity, Card, EquityResult, Number, Suit};

fn main() {
    let hands = [
        [
            Card::new(Suit::Spades, Number::Ace),
            Card::new(Suit::Clubs, Number::King),
        ],
        [
            Card::new(Suit::Hearts, Number::Queen),
            Card::new(Suit::Spades, Number::Queen),
        ],
    ];
    let dead_cards = [];
    let result = compute_equity(&hands, &dead_cards);

    for (
        EquityResult {
            win_count,
            loss_count,
            tie_count,
            count,
        },
        hand,
    ) in result.into_iter().zip(hands)
    {
        #[allow(clippy::cast_precision_loss)]
        let win_percent = win_count as f64 / count as f64 * 100.;

        #[allow(clippy::cast_precision_loss)]
        let loss_percent = loss_count as f64 / count as f64 * 100.;

        #[allow(clippy::cast_precision_loss)]
        let tie_percent = tie_count as f64 / count as f64 * 100.;

        println!("Hand: {:?}, {:?}", hand[0], hand[1]);

        println!(
            "Win: {:.2}%\nLose: {:.2}%\nTie: {:.2}%\n",
            win_percent, loss_percent, tie_percent
        );

        println!(
            "Win: {}\nLose: {}\nTie: {}\nTotal: {}",
            win_count, loss_count, tie_count, count
        );
        println!("===============================");
    }
}
