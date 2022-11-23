use poker_calculator::*;

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

    println!(
        "Win: {:.2}\nLose: {:.2}\nTie: {:.2}\n",
        win_count as f64 / count as f64 * 100.,
        loss_count as f64 / count as f64 * 100.,
        tie_count as f64 / count as f64 * 100.
    );

    println!(
        "Win: {}\nLose: {}\nTie: {}\nTotal: {}",
        win_count, loss_count, tie_count, count
    );
}
