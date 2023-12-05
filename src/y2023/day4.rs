use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Clone)]
pub struct ScratchCard {
    winning_numbers: HashSet<i32>,
    numbers: HashSet<i32>,
    matched: u32,
    id: u32,
}

impl ScratchCard {
    fn new(line: &str) -> Self {
        let mut pieces = line.split(":");
        let id: u32 = pieces
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .expect("Should have id");
        let pieces = pieces
            .next()
            .unwrap()
            .split("|")
            .map(|str| str.trim())
            .collect::<Vec<&str>>();
        let raw_winning_numbers = pieces[0];
        let raw_numbers = pieces[1];

        let mut winning_numbers = HashSet::new();
        let mut numbers = HashSet::new();
        let mut matched = 0;

        for raw_number in raw_winning_numbers.trim().split(" ") {
            let res = match raw_number.trim().parse::<i32>() {
                Ok(number) => number,
                Err(_) => continue,
            };
            winning_numbers.insert(res);
        }
        for raw_number in raw_numbers.trim().split(" ") {
            let res = match raw_number.trim().parse::<i32>() {
                Ok(number) => number,
                Err(_) => continue,
            };
            numbers.insert(res);
            if winning_numbers.contains(&res) {
                matched += 1;
            }
        }

        ScratchCard {
            winning_numbers,
            numbers,
            matched,
            id,
        }
    }

    fn score(self) -> u32 {
        if self.matched == 0 {
            return 0;
        }

        u32::pow(2, self.matched - 1)
    }

    fn winnings(self, max: u32) -> Vec<u32> {
        let start = self.id + 1;
        (start..cmp::min(start + self.matched, max + 1)).collect()
    }
}

pub fn part_1(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|line| ScratchCard::new(line).score())
        .sum()
}

// pub fn get_scores(
//     sc: &ScratchCard,
//     scratch_cards: &Vec<ScratchCard>,
//     mut counts: HashMap<u32, u32>,
// ) -> u32 {
//     if let Some(score) = counts.get(&sc.id) {
//         return *score;
//     }

//     let score: u32 = sc
//         .clone()
//         .winnings(99999)
//         .iter()
//         .map(|w| get_scores(&scratch_cards[*w as usize], scratch_cards, counts))
//         .sum::<u32>();

//     counts.entry(sc.id).or_insert(0);

//     score
// }

pub fn part_2(input: &Vec<String>) -> u32 {
    let scratch_cards: Vec<_> = input.iter().map(|line| ScratchCard::new(line)).collect();
    let scrath_card_ids: Vec<_> = scratch_cards.iter().map(|sc| sc.id).collect();
    let mut scratch_counts: HashMap<u32, u32> = HashMap::new();
    let mut scratch_winnings: HashMap<u32, Vec<u32>> = HashMap::new();
    scratch_cards.iter().for_each(|sc| {
        scratch_winnings.insert(
            sc.id,
            sc.clone()
                .winnings(u32::try_from(input.len()).unwrap() - 1 + 999),
        );
    });

    let mut memo: HashMap<u32, u32> = HashMap::new();
    let max = scratch_cards.last().expect("It exists").id;

    for sc in scratch_cards.iter().rev() {
        let winnings = sc.clone().winnings(max);
        let score: u32 =winnings
            .iter()
            .map(|x| memo.get(x).expect("hmm missing"))
            .sum();
        memo.insert(sc.id, score + 1);
    }

    scrath_card_ids
        .iter()
        .map(|id| memo.get(id).unwrap_or(&0))
        .sum()

    // let mut queue = VecDeque::from(scrath_card_ids);
    // loop {
    //     let id = queue.pop_front();
    //     if let Some(id) = id {
    //         *scratch_counts.entry(id).or_insert(0) += 1;

    //         let _winnings = scratch_winnings
    //             .get(&id)
    //             .unwrap()
    //             .iter()
    //             .for_each(|winning_id| {
    //                 queue.push_back(*winning_id);
    //             });
    //         // println!(
    //         //     "Winnings for card {}: {:?}",
    //         //     id,
    //         //     scratch_winnings.get(&id).unwrap(),
    //         // );
    //         // println!("\tQueue: {:?}", queue);
    //         // println!("\tCounts: {:?}", scratch_counts);
    //     } else {
    //         break;
    //     }
    // }

    // scratch_counts.iter().map(|(_key, value)| value).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_1(&lines), 13);
        let v: Vec<u32> = vec![];
        assert_eq!(v.iter().sum::<u32>(), 0);
    }

    #[test]
    fn build_scratch_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        assert_eq!(
            ScratchCard::new(input),
            ScratchCard {
                winning_numbers: HashSet::from_iter(vec![41, 48, 83, 86, 17]),
                numbers: HashSet::from_iter(vec![83, 86, 6, 31, 17, 9, 48, 53]),
                matched: 4,
                id: 1,
            }
        )
    }

    #[test]
    fn scratch_card_score() {
        assert_eq!(
            ScratchCard::new("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").score(),
            8
        );
        assert_eq!(
            ScratchCard::new("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19").score(),
            2
        );
        assert_eq!(
            ScratchCard::new("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1").score(),
            2
        );
        assert_eq!(
            ScratchCard::new("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83").score(),
            1
        );
        assert_eq!(
            ScratchCard::new("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36").score(),
            0
        );
        assert_eq!(
            ScratchCard::new("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").score(),
            0
        );
    }

    #[test]
    fn winnings() {
        assert_eq!(
            ScratchCard::new("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").winnings(10),
            vec![2, 3, 4, 5]
        );
        assert_eq!(
            ScratchCard::new("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").winnings(3),
            vec![2, 3]
        );
    }

    #[test]
    fn part_2_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_2(&lines), 30)
    }

    #[test]
    fn part_2_test2() {
        let input = "Card   1:  5 37 16  3 56 11 23 72  7  8 |  3 79 35 45 72 69 15 14 48 88 96 37 11 75 83 56 23  7 16 50 21 91 32 97 17
        Card   2:  1 45 93 96 65 88 78 15 27 26 |  5 84 62 63 45 61  1 80 88 77 40 51 73 21 32 98 74 59 97  9 15 71 25 43 23
        Card   3:  9 99 34 44 37 16 67 43 41 83 | 43 41  5 69 90 50 34 94 86 59 98 16 99 28 44 37 47 57  7 14 83 67 76  9 77
        Card   4: 45 99 64 82 57  9 56 17 78  7 | 75 56 30 88 64  1 98 27  9 57  7  6 77 44 17 78 82 99 16 91 76 94 63 87 45
        Card   5: 76 80 42 88 26 56 79 63  6 37 | 16  4 40 34 46 76 67 69  1 54  5 55 59 24 78 29 26  9 51 44 92 41 63 88 65
        Card   6: 59 23 88 38 49 16 24 18 22 89 | 52 25 88 27 23 79 22 84 72 80 39 17 49 96 56 60 44 45 16 63 78 38 19  5 43
        Card   7: 81  1 37  6 20 76  3 31 93 83 | 74 32 25 76 43 87 52 93 47 85 83 31 17 72  6 99  1 36 20 81  3 69 78 44 37
        Card   8: 74 73 65 29 66 47 43 11 24 38 |  5  3  1 88 29 11 49 67 47 33 31 61 63 75 84 35 18 71 66 92 81 97  8  9 85
        Card   9: 67 68  8 74 17 11 28 47 96  2 | 85  7 37 33 15 18 91 96  4 67 16 47 28 26 80 52 17 97 68  8 11 79  2 46 74
        Card  10: 91 22 85 35 47 26 99 39 72 38 |  5  7 12 14 62 93 61 56 82  4  1 51 86 36 43 29 50 75 68 25 98 77 74 64 24
        Card  11:  4 53 44 83 23 84 40 55 69 82 | 24 48 11 37 60 76 41 29 58 39 45 88 95 67 49 28 36 35 86 33 18 63 51 19 93
        Card  12: 40 45 87 58 72 59 89 55 20 91 |  7 62  2 91 59 78  4 44 25 24 57 94 79 75 51 54 55 90 83 30 68 47  3 69 26
        Card  13: 92 58 35 96 84 62 31 65 95  5 | 22 52 84 98 62 31 75  7 12 78 51 91 37 58 46 85 21 61 95 49 36  5 79 92  4
        Card  14: 75 37 41 53 12 77 97  6 54 29 | 52 65 46 94 20  6 76 75 70 83 29 93 64  1 12 58 89 49 26 16 82 85 74 61 41";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_2(&lines), 30)
    }
}
