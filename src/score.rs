use super::card::{Hand, HandRanking, Rank};

pub fn score_hand(hand: &Hand) -> usize {
    use HandRanking::*;

    match hand.ranking() {
        HighCard => 0,
        Pair => {
            if hand.runs()[0][0].rank >= Rank::Jack {
                5
            } else {
                0
            }
        }
        TwoPair => 10,
        ThreeOfAKind => 15,
        Straight => 20,
        Flush => 25,
        FullHouse => 40,
        FourOfAKind => 125,
        StraightFlush => 250,
        RoyalFlush => 3000,
        FiveOfAKind => 8000,
    }
}
