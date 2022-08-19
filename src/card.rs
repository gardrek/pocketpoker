// const SUIT_CHAR: [char; 8] = ['♠', '♥', '♣', '♦', '♤', '♡', '♧', '♢'];
// const SUIT_CHAR_BW: [char; 8] = ['♠', '♡', '♣', '♢', '♤', '♥', '♧', '♦'];
// const SUIT_CHAR_HOLLOW: [char; 4] = ['♤', '♡', '♧', '♢'];
const SUIT_CHAR_SOLID: [char; 4] = ['♠', '♥', '♣', '♦'];
// const SUIT_CHAR_MIXED: [char; 4] = ['♤', '♥', '♧', '♦'];
const SUIT_CHAR: [char; 4] = SUIT_CHAR_SOLID;

// const RED_SUIT_STYLE: console::Style = console::Style::new().red();

use num_enum::IntoPrimitive;

use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive, Sequence)]
#[repr(usize)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn to_int(&self) -> usize {
        match self {
            // Aces are rank 14 by default (one more than King)
            // here we set them to rank 1
            Rank::Ace => 1,
            _ => self.to_int_aces_high(),
        }
    }

    fn to_int_aces_high(&self) -> usize {
        (*self).into()
    }
}

/*
const RANK_SYMBOL: [char; 13] = [
    'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K',
];
*/

const RANK_SYMBOL_WIDE: [&'static str; 13] = [
    " A", " 2", " 3", " 4", " 5", " 6", " 7", " 8", " 9", "10", " J", " Q", " K",
];

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", RANK_SYMBOL_WIDE[self.to_int() - 1])
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Sequence)]
pub enum Suit {
    Spade = 0,
    Heart,
    Club,
    Diamond,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", SUIT_CHAR[*self as usize])
    }
}

use Rank::*;

const ACE_HIGH_STRAIGHT: [&Rank; 5] = [&Ten, &Jack, &Queen, &King, &Ace];

const FIVE_HIGH_STRAIGHT: [&Rank; 5] = [&Two, &Three, &Four, &Five, &Ace];

#[derive(Clone, Copy, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

/// use {:+} to display suits with four colors
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let default = console::Style::new();
        let red = console::Style::new().red();
        let club_four_color = console::Style::new().blue();
        let diamond_four_color = console::Style::new().yellow().bright();

        use Suit::*;

        /*
         */

        let style = if f.sign_plus() {
            match self.suit {
                Spade => default,
                Heart => red,
                Club => club_four_color,
                Diamond => diamond_four_color,
            }
        } else {
            match self.suit {
                Spade | Club => default,
                Heart | Diamond => red,
            }
        };

        write!(
            f,
            "{}{} ",
            style.apply_to(&self.rank),
            style.apply_to(&self.suit)
        )
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.rank.cmp(&other.rank))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        &self.rank == &other.rank
    }
}

// TODO: add a pointer to the "Top" of the deck and add a function to "draw" 5 cards
// that is, move the pointer forward 5 and return a slice of length 5
pub struct Deck {
    deck: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = vec![];

        for suit in enum_iterator::all::<Suit>() {
            for rank in enum_iterator::all::<Rank>() {
                deck.push(Card { rank, suit });
            }
        }

        Deck { deck }
    }

    pub fn len(&self) -> usize {
        self.deck.len()
    }

    pub fn index(&self, index: usize) -> &Card {
        &self.deck[index]
    }

    // Fisher-Yates shuffle
    pub fn shuffle(&mut self, prng: &mut prng::Prng64) {
        let deck = &mut self.deck;
        for i in (1..deck.len()).rev() {
            let j = prng.next().unwrap() as usize % (i + 1);
            deck.swap(i, j);
        }
    }

    // return a hand made from a reference to five cards
    // TODO: actually make a version of Hand that uses references
    // then this function can instead make that
    pub fn _peek_poker_hand(&self, offset: usize, length: usize) -> Option<Hand> {
        if self.deck.len() < length {
            return None;
        }

        Some(Hand::new(&self.deck[offset..(offset + length)]))
    }

    pub fn peek_five_cards(&self, offset: usize) -> &[Card] {
        &self.deck[offset..(offset + 5)]
    }
}

impl std::fmt::Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.deck)
    }
}

#[derive(Eq)]
pub struct Hand {
    hand: Vec<Card>,
    runs: Vec<Vec<Card>>,
    kickers: Vec<Card>,
    ranking: HandRanking,
}

impl Hand {
    pub fn cards(&self) -> &[Card] {
        &self.hand[..]
    }

    pub fn runs(&self) -> &Vec<Vec<Card>> {
        &self.runs
    }

    pub fn ranking(&self) -> &HandRanking {
        &self.ranking
    }

    pub fn new(hand: &[Card]) -> Hand {
        assert_eq!(hand.len(), 5);

        let mut copy = hand.to_vec();

        copy.sort();

        let sorted = copy;

        let mut runs = vec![];

        let mut previous_card: Option<Card> = None;
        let mut run = vec![];
        for &card in &sorted {
            match previous_card {
                None => (),
                Some(inner) => {
                    run.push(inner);
                    if &inner.rank != &card.rank {
                        runs.push(run);
                        run = vec![];
                    }
                }
            }
            previous_card = Some(card.clone());
        }

        match previous_card {
            None => (),
            Some(inner) => {
                run.push(inner);
                runs.push(run);
            }
        }

        let mut runs = runs.into_iter().collect::<Vec<_>>();

        runs.sort_by(|a, b| a.len().cmp(&b.len()));

        let mut kickers = runs
            .clone()
            .into_iter()
            .filter(|run| run.len() == 1)
            .map(|run| run[0])
            .collect::<Vec<_>>();

        let runs = runs
            .into_iter()
            .filter(|run| run.len() > 1)
            .collect::<Vec<_>>();

        let is_straight = Hand::eval_is_straight(&sorted);

        let is_five_high_straight =
            sorted.iter().map(|card| &card.rank).collect::<Vec<_>>() == FIVE_HIGH_STRAIGHT;

        if is_straight & is_five_high_straight {
            // Fix order of "kicker" section for five-high straights
            let ace = kickers.pop().unwrap();
            kickers.insert(0, ace);
            let kickers_ranks = kickers.iter().map(|card| &card.rank).collect::<Vec<_>>();
            assert_eq!(kickers_ranks, [&Ace, &Two, &Three, &Four, &Five]);
        }

        let is_flush = Hand::eval_is_flush(&sorted);

        let ranks = sorted.iter().map(|card| &card.rank).collect::<Vec<_>>();

        let is_royal = ranks == ACE_HIGH_STRAIGHT;

        let ranking = Self::rank_hand(&runs, is_straight, is_flush, is_royal);

        Hand {
            // TODO: attempt to convert this struct to hold a reference instead
            hand: hand.to_vec(),
            runs,
            kickers,
            ranking,
        }
    }

    pub fn _high_rank(&self) -> Option<Rank> {
        self.kickers.last().map(|card| card.rank)
    }

    pub fn eval_is_flush(hand: &Vec<Card>) -> bool {
        let mut same_suit = None;
        for &card in hand {
            let &suit = &card.suit;

            match same_suit {
                None => same_suit = Some(suit),
                Some(inner) => {
                    if inner != suit {
                        return false;
                    }
                }
            }
        }

        true
    }

    // given a list of five cards sorted by rank, aces high, return whether it is a straight
    pub fn eval_is_straight(sorted: &Vec<Card>) -> bool {
        let ranks = sorted.iter().map(|card| &card.rank).collect::<Vec<_>>();

        if ranks == FIVE_HIGH_STRAIGHT {
            return true;
        }

        let mut straight_rank = None;
        for card in sorted {
            let rank = card.rank.to_int_aces_high();
            match straight_rank {
                None => straight_rank = Some(rank),
                Some(inner) => {
                    if rank != inner + 1 {
                        return false;
                    }
                    straight_rank = Some(rank)
                }
            }
        }

        true
    }

    fn runs_ranking(runs: &Vec<Vec<Card>>) -> Option<HandRanking> {
        use HandRanking::*;

        Some(
            match runs.iter().map(|run| run.len()).collect::<Vec<_>>()[..] {
                [2] => Pair,
                [2, 2] => TwoPair,
                [3] => ThreeOfAKind,
                [2, 3] => FullHouse,
                [4] => FourOfAKind,
                [5] => FiveOfAKind,
                [] => return None,
                _ => unreachable!(),
            },
        )
    }

    fn rank_hand(
        runs: &Vec<Vec<Card>>,
        is_straight: bool,
        is_flush: bool,
        is_royal: bool,
    ) -> HandRanking {
        use HandRanking::*;

        if let Some(ranking) = Self::runs_ranking(runs) {
            return ranking;
        }

        if is_straight {
            if is_flush {
                if is_royal {
                    return RoyalFlush;
                }
                return StraightFlush;
            }
            return Straight;
        }

        if is_flush {
            return Flush;
        }

        HighCard
    }

    pub fn compare(&self, other: &Hand) -> std::cmp::Ordering {
        let order = self.ranking.cmp(&other.ranking);

        match order {
            std::cmp::Ordering::Equal => (),
            _ => return order,
        }

        // this is the way it should be
        // it would work if i went through and changed the internal formats to be reversed
        // (&self.runs, &self.kickers).cmp(&(&other.runs, &other.kickers))

        // this way is stupid but i'm gonna do it for now, because reversing them internally is too hard
        (
            &self.runs.iter().rev().collect::<Vec<_>>(),
            &self.kickers.iter().rev().collect::<Vec<_>>(),
        )
            .cmp(&(
                &other.runs.iter().rev().collect::<Vec<_>>(),
                &other.kickers.iter().rev().collect::<Vec<_>>(),
            ))
    }

    pub fn _display_hand_with_info(&self) -> String {
        format!(
            "{} {}-HIGH {:?} {:?} {:?}  {} points",
            self,
            match self._high_rank() {
                Some(rank) => format!("{}", rank),
                None => "xx".into(),
            },
            self.ranking,
            self.runs,
            self.kickers,
            super::score::score_hand(self),
        )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.compare(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        match self.compare(other) {
            std::cmp::Ordering::Equal => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for card in &self.hand {
            write!(f, "{}", card)?;
        }
        write!(f, "]")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive)]
#[repr(usize)]
pub enum HandRanking {
    HighCard = 0,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
    FiveOfAKind,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn royal_flush() {
        use Rank::*;
        use Suit::*;

        let hand = Hand::new(
            &[
                (Ten, Diamond),
                (Jack, Diamond),
                (Queen, Diamond),
                (King, Diamond),
                (Ace, Diamond),
            ]
            .into_iter()
            .map(|(rank, suit)| Card { rank, suit })
            .collect::<Vec<_>>()[..],
        );

        assert!(hand.is_straight());
        assert!(hand.is_flush());
        assert!(hand.is_royal());
    }

    #[test]
    fn ace_high_straight() {
        use Rank::*;
        use Suit::*;

        let hand = Hand::new(
            &[
                (Ten, Heart),
                (Jack, Spade),
                (Queen, Spade),
                (King, Spade),
                (Ace, Spade),
            ]
            .into_iter()
            .map(|(rank, suit)| Card { rank, suit })
            .collect::<Vec<_>>()[..],
        );

        use HandRanking::*;

        let is_straight = match &hand.ranking {
            Straight | StraightFlush | RoyalFlush => true,
            _ => false,
        };

        assert!(is_straight);
    }

    #[test]
    fn five_high_straight() {
        use Rank::*;
        use Suit::*;

        let hand = Hand::new(
            &[
                (Ace, Heart),
                (Two, Spade),
                (Three, Spade),
                (Four, Spade),
                (Five, Spade),
            ]
            .into_iter()
            .map(|(rank, suit)| Card { rank, suit })
            .collect::<Vec<_>>()[..],
        );

        let mut copy = hand.hand.clone();

        copy.sort();

        let sorted = copy;

        let is_five_high_straight =
            sorted.iter().map(|card| &card.rank).collect::<Vec<_>>() == FIVE_HIGH_STRAIGHT;

        use HandRanking::*;

        let is_straight = match &hand.ranking {
            Straight | StraightFlush | RoyalFlush => true,
            _ => false,
        };

        assert!(is_straight);
        assert!(is_five_high_straight);
    }

    #[test]
    fn sorting() {
        let mut a = [4, 3, 2];
        a.sort();
        let mut b = vec![vec![8, 3], vec![6, 2, 4], vec![2]];
        b.sort();
        assert_eq!(&b[..], vec![vec![2], vec![6, 2, 4], vec![8, 3]]);

        let winner = (vec![vec![11, 11]], vec![5, 6, 7]);
        let loser = (vec![vec![10, 10]], vec![5, 6, 9]);
        assert!(winner > loser);
    }

    #[test]
    fn five_high_straight_compare() {
        use Rank::*;
        use Suit::*;

        let hand = Hand::new(
            &[
                (Ace, Heart),
                (Two, Spade),
                (Three, Spade),
                (Four, Spade),
                (Five, Spade),
            ]
            .into_iter()
            .map(|(rank, suit)| Card { rank, suit })
            .collect::<Vec<_>>()[..],
        );

        let other_hand = Hand::new(
            &[
                (Two, Spade),
                (Three, Spade),
                (Four, Spade),
                (Five, Spade),
                (Six, Heart),
            ]
            .into_iter()
            .map(|(rank, suit)| Card { rank, suit })
            .collect::<Vec<_>>()[..],
        );

        assert!(hand < other_hand);
        assert_eq!(hand.compare(&other_hand), std::cmp::Ordering::Less);
    }
}
