use pocketpoker::get_prng_seed;
use pocketpoker::card::{Deck, Hand};
use pocketpoker::score;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // prng is short for Pseudo-Random Number Generator
    let mut prng = prng::Prng64::new(get_prng_seed());

    let mut hands_dealt = 0;
    let mut hands_matched = 0;
    let mut total_score = 0;

    let mut deck = Deck::new();

    for _ in 0..500 {
        deck.shuffle(&mut prng);

        for index in 0..5 {
            let hand1 = Hand::new(deck.peek_five_cards(index * 5));

            let score = score::score_hand(&hand1);

            //          let hand2 = deck.peek_poker_hand(index * 10, 5).unwrap();

            hands_dealt += 1;

            total_score += score as isize - 5;

            //            if hand1.compare(&hand2) != hand1.old_compare(&hand2) {
            if score >= 10 {
                hands_matched += 1;

                println!("{} {}", hand1, hand1.display_hand_debug_info());
                //                println!("{}", hand2.display_hand_with_info());
                //                println!();
            }
        }
    }

    println!("hands dealt:     {:>10}", hands_dealt);
    println!("matching hands:  {:>10}", hands_matched);

    println!(
        "percent:         {:>13.02}%",
        (hands_matched as f64 / hands_dealt as f64) * 100.0
    );

    println!(
        "average score:   {:>13.02}",
        (total_score as f64 / hands_dealt as f64)
    );

    Ok(())
}