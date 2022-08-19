mod card;
mod score;
mod ui;

use card::{Card, Deck, Hand};
use ui::HandRankingDisplay;

pub fn run_console_game() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = GameState::new();

    state.main_loop()
}

pub struct GameState {
    prng: prng::Prng64,
    deck: Deck,
    score: usize,
    high_score: usize,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            prng: prng::Prng64::new(get_prng_seed()),
            deck: Deck::new(),
            score: 100,
            high_score: 100,
        }
    }

    pub fn main_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut input = String::new();

        let blank_ranking_display = HandRankingDisplay(None);

        'main: loop {
            println!("shuffling...");

            self.deck.shuffle(&mut self.prng);

            let mut held_cards = [false; 5];

            let hand = loop {
                input.clear();

                let initial_cards = self.deck.peek_five_cards(0);

                display_cards_and_holds(initial_cards, &held_cards[..]);
                println!();
                println!("{}", blank_ranking_display);
                println!();

                println!();
                println!();
                println!("Enter a number from 1 to 5 to hold / unhold. Enter 'go' to finish: ");
                std::io::stdin().read_line(&mut input)?;
                for i in 0..5 {
                    if input.contains(&(i + 1).to_string()) {
                        held_cards[i] = !held_cards[i];
                    }
                }

                if input.contains("quit") {
                    break 'main;
                }

                if input.contains("go") || input.contains("deal") || input.clone().trim() == "" {
                    break do_hold(&self.deck, initial_cards, &held_cards[..]);
                }
            };

            let hand = Hand::new(&hand[..]);

            input.clear();

            let score_this_hand = score::score_hand(&hand);

            self.score += score_this_hand;

            if self.score >= self.high_score {
                self.high_score = self.score;
            }

            display_cards_and_holds(&hand.cards()[..], &[false; 5]);
            println!("");

            let ranking_display = HandRankingDisplay(Some(hand.ranking().clone()));
            println!("{}", ranking_display);
            println!();

            println!("Score this hand: {:>6} points", score_this_hand);

            if self.score < 5 {
                println!("Sorry, You Lose!");
                println!("High score:      {:>6} points", self.high_score);
                break 'main;
            }

            self.score = self.score - 5;

            println!("Total score:     {:>6} points", self.score);
            println!("Press Enter to continue: ");
            std::io::stdin().read_line(&mut input)?;

            if input.contains("quit") {
                break 'main;
            }
        }

        Ok(())
    }
}

fn do_hold(deck: &Deck, initial_hand: &[Card], held: &[bool]) -> Vec<Card> {
    assert_eq!(
        initial_hand.len(),
        held.len(),
        "do_hold: arguments not the same length"
    );

    let mut index = initial_hand.len();

    assert!(
        deck.len() >= index,
        "do_hold: deck doesn't have enough cards left to deal"
    );

    let mut new_hand = vec![];

    for i in 0..index.clone() {
        if held[i] {
            new_hand.push(initial_hand[i].clone());
        } else {
            new_hand.push(deck.index(index).clone());
            index += 1;
        }
    }

    new_hand
}

fn display_cards_and_holds(cards: &[Card], held: &[bool]) {
    assert_eq!(cards.len(), held.len());
    for i in 1..=5 {
        print!("  {}   ", i);
    }
    println!();
    for (card, is_held) in cards.iter().zip(held.iter()) {
        if *is_held {
            print!("[{:+}]", card);
        } else {
            print!(" {:+} ", card);
        }
    }
    println!();
}

fn _old_main() -> Result<(), Box<dyn std::error::Error>> {
    // prng is short for Pseudo-Random Number Generator
    let mut prng = prng::Prng64::new(get_prng_seed());

    let mut hands_dealt = 0;
    let mut hands_matched = 0;
    let mut total_score = 0;

    let mut deck = card::Deck::new();

    for _ in 0..500 {
        deck.shuffle(&mut prng);

        for index in 0..5 {
            let hand1 = deck._peek_poker_hand(index * 5, 5).unwrap();

            let score = score::score_hand(&hand1);

            //          let hand2 = deck.peek_poker_hand(index * 10, 5).unwrap();

            hands_dealt += 1;

            total_score += score as isize - 5;

            //            if hand1.compare(&hand2) != hand1.old_compare(&hand2) {
            if score >= 10 {
                hands_matched += 1;

                println!("{}", hand1._display_hand_with_info());
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

fn get_prng_seed() -> [u64; 4] {
    use std::time::SystemTime;

    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get duration since UNIX_EPOCH.");

    let number0 = duration.as_secs();
    let number1 = duration.subsec_nanos();
    // let number1 = duration.subsec_nanos() & 0xffff0000 >> 16;

    [0, 1, number0 as u64, number1 as u64]
}
