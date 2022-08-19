use crate::card;

use enum_iterator::Sequence;

#[derive(Sequence)]
enum HandRankingElement {
    Two = 0b1,
    Pair = 0b10,
    Four = 0b100,
    FullHouse = 0b1000,
    Royal = 0b1_0000,
    Three = 0b10_0000,
    OfAKind = 0b100_0000,
    Straight = 0b1000_0000,
    Flush = 0b1_0000_0000,
    Empty = 0b10_0000_0000,
}

/*
use HandRankingElement::*;
const DISPLAY_ROWS: [[HandRankingElement; 5]; 2] = [
    [Two, Pair, Four, FullHouse, Royal],
    [Three, OfAKind, Straight, Flush, Empty],
];
*/

impl std::fmt::Display for HandRankingElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use HandRankingElement::*;
        write!(
            f,
            "{}",
            match self {
                Two => "TWO",
                Pair => "PAIR",
                Four => "FOUR",
                FullHouse => "FULL HOUSE",
                Royal => "ROYAL",
                Three => "THREE",
                OfAKind => "of a KIND",
                Straight => "STRAIGHT",
                Flush => "FLUSH",
                Empty => "",
            }
        )
    }
}

fn hand_ranking_to_display_bitmap(ranking: &card::HandRanking) -> u16 {
    use card::HandRanking;
    use HandRankingElement::*;

    match ranking {
        HandRanking::HighCard => 0,
        HandRanking::Pair => Pair as u16,
        HandRanking::TwoPair => Two as u16 | Pair as u16,
        HandRanking::ThreeOfAKind => Three as u16 | OfAKind as u16,
        HandRanking::Straight => Straight as u16,
        HandRanking::Flush => Flush as u16,
        HandRanking::FullHouse => FullHouse as u16,
        HandRanking::FourOfAKind => Four as u16 | OfAKind as u16,
        HandRanking::StraightFlush => Straight as u16 | Flush as u16,
        HandRanking::RoyalFlush => Royal as u16 | Flush as u16,
        HandRanking::FiveOfAKind => 0,
    }
}

pub struct HandRankingDisplay(pub Option<card::HandRanking>);

impl std::fmt::Display for HandRankingDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let blank_style = console::Style::new().dim();
        let lit_style = console::Style::new();

        for (i, element) in enum_iterator::all::<HandRankingElement>().enumerate() {
            let label = format!("{}", element);

            let blank = blank_style.apply_to(".".repeat(label.len()));

            let label = match &self.0 {
                Some(ranking) => {
                    let bitmap = hand_ranking_to_display_bitmap(ranking);

                    if bitmap >> i & 1 == 1 {
                        lit_style.apply_to(label)
                    } else {
                        blank
                    }
                }
                None => blank,
            };

            // end of the first line
            if i == 5 {
                write!(f, "\n")?;
            }

            if i == 0 || i == 5 || i == 9 {
                write!(f, "{}", label)?;
            } else {
                write!(f, " {}", label)?;
            }
        }
        Ok(())
    }
}
