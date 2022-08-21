use crate::card::*;

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

    match hand.ranking() {
        HandRanking::RoyalFlush => (),
        _ => panic!(),
    }
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

    match hand.ranking() {
        HandRanking::Straight => (),
        _ => panic!(),
    }
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

    let mut copy = hand.cards().to_vec();

    copy.sort();

    let sorted = copy;

    let is_five_high_straight =
        sorted.iter().map(|card| &card.rank).collect::<Vec<_>>() == FIVE_HIGH_STRAIGHT;

    use HandRanking::*;

    let is_straight = match &hand.ranking() {
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
}

#[test]
fn nonflush_sorting() {
    use Rank::*;

    let hand_ranks = &[
        // high card
        [Two, Three, Four, Five, Seven],
        [Two, Three, Four, Five, Eight],
        [Two, Four, Five, Six, Eight],
        [Two, Five, Six, Seven, Eight],
        // pair
        [Jack, Jack, Two, Three, Four],
        [Queen, Queen, Two, Three, Four],
        [Queen, Queen, Three, Four, Five],
        // two pair
        [Queen, Queen, Seven, Seven, Two],
        [Queen, Queen, Seven, Seven, Three],
        [Queen, Queen, Eight, Eight, Three],
        // three of a kind
        [Queen, Queen, Queen, Seven, Eight],
        // straight
        [Queen, Jack, Ten, Nine, Eight],
        [King, Queen, Jack, Ten, Nine],
        // full house
        [King, King, Eight, Eight, Eight],
        [Ace, Ace, Eight, Eight, Eight],
        [Ace, Ace, Nine, Nine, Nine],
        // four of a kind
        [Nine, Nine, Nine, Nine, Ace],
        [Ace, Ace, Ace, Ace, Nine],
    ];

    let hands = hand_ranks
        .iter()
        .map(|ranks| hand_from_ranks(&ranks[..]))
        .collect::<Vec<_>>();

    let mut hands_copy = hand_ranks
        .iter()
        .map(|ranks| hand_from_ranks(&ranks[..]))
        .collect::<Vec<_>>();

    hands_copy.sort();
    let sorted_hands = hands_copy;

    assert_eq!(&hands, &sorted_hands)
}

fn hand_from_ranks(ranks: &[Rank]) -> Hand {
    Hand::new(
        &ranks
            .into_iter()
            .enumerate()
            .map(|(i, &rank)| Card {
                rank,
                suit: Suit::try_from(i % 4).unwrap(),
            })
            .collect::<Vec<_>>()[..],
    )
}
