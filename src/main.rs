use poker_calculator::{compute_result, Card, ComputeResult, Number, Suit};

fn main() {
    let ComputeResult {
        win_count,
        loss_count,
        tie_count,
        count,
    } = compute_result(
        [
            Card::new(Suit::Hearts, Number::Queen),
            Card::new(Suit::Hearts, Number::King),
        ],
        [
            Card::new(Suit::Spades, Number::Two),
            Card::new(Suit::Hearts, Number::Two),
        ],
    );

    #[allow(clippy::cast_precision_loss)]
    let win_percent = win_count as f64 / count as f64 * 100.;

    #[allow(clippy::cast_precision_loss)]
    let loss_percent = loss_count as f64 / count as f64 * 100.;

    #[allow(clippy::cast_precision_loss)]
    let tie_percent = tie_count as f64 / count as f64 * 100.;

    println!(
        "Win: {:.2}\nLose: {:.2}\nTie: {:.2}\n",
        win_percent, loss_percent, tie_percent
    );

    println!(
        "Win: {}\nLose: {}\nTie: {}\nTotal: {}",
        win_count, loss_count, tie_count, count
    );
}
