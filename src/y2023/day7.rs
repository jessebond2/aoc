use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Card {
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Card2 {
    J,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    hand: Vec<Card>,
    hand_type: HandType,
    bet: u32,
}

impl Hand {
    fn new(line: &str) -> Hand {
        let mut parts = line.split(" ");
        let hand_str = parts.next().unwrap();
        let bet = parts.next().unwrap().parse().unwrap();

        let mut hand: Vec<Card> = vec![];
        let mut char_count = HashMap::new();
        for char in hand_str.chars() {
            let card = match char {
                '1' => Card::C1,
                '2' => Card::C2,
                '3' => Card::C3,
                '4' => Card::C4,
                '5' => Card::C5,
                '6' => Card::C6,
                '7' => Card::C7,
                '8' => Card::C8,
                '9' => Card::C9,
                'T' => Card::T,
                'J' => Card::J,
                'Q' => Card::Q,
                'K' => Card::K,
                _ => Card::A,
            };
            hand.push(card);
            let count: &mut u32 = char_count.entry(char).or_insert(0);
            *count += 1;
        }
        let max = char_count.values().max_by(|a, b| a.cmp(b)).unwrap();

        let hand_type = match max {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if char_count.len() == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfKind
                }
            }
            2 => {
                if char_count.len() == 3 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        };

        Hand {
            hand,
            hand_type,
            bet,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type.eq(&other.hand_type) {
            for (card_index, card) in self.hand.iter().enumerate() {
                if card.eq(&other.hand[card_index]) {
                    continue;
                }
                return card.cmp(&other.hand[card_index]);
            }
        }
        self.hand_type.cmp(&other.hand_type)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand2 {
    hand: Vec<Card2>,
    hand_type: HandType,
    bet: u32,
}

impl Hand2 {
    fn new(line: &str) -> Hand2 {
        let mut parts = line.split(" ");
        let hand_str = parts.next().unwrap();
        let bet = parts.next().unwrap().parse().unwrap();

        let mut hand: Vec<Card2> = vec![];
        let mut char_count = HashMap::new();
        for char in hand_str.chars() {
            let card = match char {
                '1' => Card2::C1,
                '2' => Card2::C2,
                '3' => Card2::C3,
                '4' => Card2::C4,
                '5' => Card2::C5,
                '6' => Card2::C6,
                '7' => Card2::C7,
                '8' => Card2::C8,
                '9' => Card2::C9,
                'T' => Card2::T,
                'J' => Card2::J,
                'Q' => Card2::Q,
                'K' => Card2::K,
                _ => Card2::A,
            };
            hand.push(card);
            let count: &mut u32 = char_count.entry(char).or_insert(0);
            *count += 1;
        }

        let joker_count = match char_count.get(&'J') {
            Some(count) => count.clone(),
            None => 0,
        };
        char_count.remove(&'J');
        let max = char_count.values().max_by(|a, b| a.cmp(b)).unwrap_or(&0);
        let adjusted_max = max + joker_count;

        let hand_type = match adjusted_max {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if char_count.len() == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfKind
                }
            }
            2 => {
                if char_count.len() == 3 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        };

        Hand2 {
            hand,
            hand_type,
            bet,
        }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type.eq(&other.hand_type) {
            for (card_index, card) in self.hand.iter().enumerate() {
                if card.eq(&other.hand[card_index]) {
                    continue;
                }
                return card.cmp(&other.hand[card_index]);
            }
        }
        self.hand_type.cmp(&other.hand_type)
    }
}

pub fn part_1(input: &Vec<String>) -> u32 {
    let mut hands: Vec<_> = input.iter().map(|line| Hand::new(line)).collect();
    hands.sort();
    //println!("Hands in order {:?}", hands);

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, hand)| acc + (1 + idx as u32) * hand.bet)
}

pub fn part_2(input: &Vec<String>) -> u32 {
    let mut hands: Vec<_> = input.iter().map(|line| Hand2::new(line)).collect();
    hands.sort();
    //println!("Hands in order {:?}", hands);

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, hand)| acc + (1 + idx as u32) * hand.bet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_1(&lines), 6440);
    }

    #[test]
    fn test_hands() {
        assert_eq!(
            Hand::new("AAAAA 1"),
            Hand {
                hand: vec![Card::A, Card::A, Card::A, Card::A, Card::A],
                hand_type: HandType::FiveOfAKind,
                bet: 1
            }
        );
        assert_eq!(Hand::new("AAAA1 1").hand_type, HandType::FourOfAKind,);
        assert_eq!(Hand::new("AAA11 1").hand_type, HandType::FullHouse);
        assert_eq!(Hand::new("AAA12 1").hand_type, HandType::ThreeOfKind);
        assert_eq!(Hand::new("AA112 1").hand_type, HandType::TwoPair);
        assert_eq!(Hand::new("AA123 1").hand_type, HandType::OnePair);
        assert_eq!(Hand::new("12345 1").hand_type, HandType::HighCard);
    }

    #[test]
    fn compare_hands() {
        assert!(Hand::new("AAAA1 1") < Hand::new("AAAAA 1"));
        assert!(Hand::new("1AAAA 1") < Hand::new("AAAA1 1"));
    }

    #[test]
    fn test_hands_2() {
        assert_eq!(Hand2::new("AAAAA 1").hand_type, HandType::FiveOfAKind);
        assert_eq!(Hand2::new("AAAAJ 1").hand_type, HandType::FiveOfAKind);
        assert_eq!(Hand2::new("AAAJJ 1").hand_type, HandType::FiveOfAKind);
        assert_eq!(Hand2::new("AAJJJ 1").hand_type, HandType::FiveOfAKind);
        assert_eq!(Hand2::new("AJJJJ 1").hand_type, HandType::FiveOfAKind);
        assert_eq!(Hand2::new("JJJJJ 1").hand_type, HandType::FiveOfAKind);

        assert_eq!(Hand2::new("AAAJ1 1").hand_type, HandType::FourOfAKind);
        assert_eq!(Hand2::new("AAJJ1 1").hand_type, HandType::FourOfAKind);
        assert_eq!(Hand2::new("AJJJ1 1").hand_type, HandType::FourOfAKind);

        assert_eq!(Hand2::new("AAA11 1").hand_type, HandType::FullHouse);
        assert_eq!(Hand2::new("AAJ11 1").hand_type, HandType::FullHouse);

        assert_eq!(Hand2::new("AAA12 1").hand_type, HandType::ThreeOfKind);
        assert_eq!(Hand2::new("AJJ12 1").hand_type, HandType::ThreeOfKind);
        assert_eq!(Hand2::new("AAJ12 1").hand_type, HandType::ThreeOfKind);

        assert_eq!(Hand2::new("AA112 1").hand_type, HandType::TwoPair);
        assert_eq!(Hand2::new("AJ123 1").hand_type, HandType::OnePair);

        assert_eq!(Hand2::new("12345 1").hand_type, HandType::HighCard);
    }

    #[test]
    fn compare_hands_2() {
        assert!(Hand2::new("JAAAA 1") < Hand2::new("11111 1"));
        assert!(Hand2::new("JJJAA 1") < Hand2::new("11111 1"));
    }

    #[test]
    fn part_2_test() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_2(&lines), 5905);
    }
}
