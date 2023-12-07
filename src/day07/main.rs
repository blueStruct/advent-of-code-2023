use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/day07/input")?.parse()?;
    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum HandType {
        FiveOfAKind,
        FourOfAKind,
        FullHouse,
        ThreeOfAKind,
        TwoPair,
        OnePair,
        HighCard,
    }

    #[derive(PartialEq, Eq)]
    struct Hand {
        cards: String,
        bid: u32,
        hand_type: HandType,
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if self.hand_type != other.hand_type {
                return self.hand_type.partial_cmp(&other.hand_type);
            } else {
                let order = "AKQJT98765432";
                for (card, other_card) in self.cards.chars().zip(other.cards.chars()) {
                    let index_card = order.find(card).unwrap();
                    let index_other_card = order.find(other_card).unwrap();

                    if index_card != index_other_card {
                        return index_card.partial_cmp(&index_other_card);
                    }
                }
                None
            }
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
        }
    }

    let mut hands: Vec<Hand> = vec![];

    for line in input.lines() {
        let (cards, bid_str) = line.split_once(" ").unwrap();
        let bid = bid_str.parse().unwrap();

        let mut card_count_map: HashMap<char, u8> = HashMap::new();

        for card in cards.chars() {
            card_count_map
                .entry(card)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        let mut counts: Vec<u8> = card_count_map.into_values().collect();
        counts.sort_unstable();
        counts.reverse();

        let hand_type = {
            if counts.len() == 1 {
                HandType::FiveOfAKind
            } else {
                match (counts[0], counts[1]) {
                    (4, 1) => HandType::FourOfAKind,
                    (3, 2) => HandType::FullHouse,
                    (3, _) => HandType::ThreeOfAKind,
                    (2, 2) => HandType::TwoPair,
                    (2, _) => HandType::OnePair,
                    _ => HandType::HighCard,
                }
            }
        };

        let hand = Hand {
            cards: cards.to_string(),
            bid,
            hand_type,
        };

        hands.push(hand);
    }

    hands.sort();

    let winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (hands.len() - i) as u32 * hand.bid)
        .sum();

    println!("The answer to part 1 is: {}", winnings);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum HandType {
        FiveOfAKind,
        FourOfAKind,
        FullHouse,
        ThreeOfAKind,
        TwoPair,
        OnePair,
        HighCard,
    }

    #[derive(PartialEq, Eq)]
    struct Hand {
        cards: String,
        bid: u32,
        hand_type: HandType,
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if self.hand_type != other.hand_type {
                return self.hand_type.partial_cmp(&other.hand_type);
            } else {
                let order = "AKQT98765432J";
                for (card, other_card) in self.cards.chars().zip(other.cards.chars()) {
                    let index_card = order.find(card).unwrap();
                    let index_other_card = order.find(other_card).unwrap();

                    if index_card != index_other_card {
                        return index_card.partial_cmp(&index_other_card);
                    }
                }
                None
            }
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
        }
    }

    let mut hands: Vec<Hand> = vec![];

    for line in input.lines() {
        let (cards, bid_str) = line.split_once(" ").unwrap();
        let bid = bid_str.parse().unwrap();

        let mut card_count_map: HashMap<char, u8> = HashMap::new();

        for card in cards.chars() {
            card_count_map
                .entry(card)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        let count_jokers = card_count_map.remove(&'J').unwrap_or_default();
        let mut counts: Vec<u8> = card_count_map.into_values().collect();
        counts.sort_unstable();
        counts.reverse();

        let hand_type = {
            if counts.len() == 0 || counts.len() == 1 {
                HandType::FiveOfAKind
            } else {
                match (counts[0] + count_jokers, counts[1]) {
                    (4, 1) => HandType::FourOfAKind,
                    (3, 2) => HandType::FullHouse,
                    (3, _) => HandType::ThreeOfAKind,
                    (2, 2) => HandType::TwoPair,
                    (2, _) => HandType::OnePair,
                    _ => HandType::HighCard,
                }
            }
        };

        let hand = Hand {
            cards: cards.to_string(),
            bid,
            hand_type,
        };

        hands.push(hand);
    }

    hands.sort();

    let winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (hands.len() - i) as u32 * hand.bid)
        .sum();

    println!("The answer to part 2 is: {}", winnings);

    Ok(())
}
