use std::cmp;
use std::collections::{HashMap, HashSet};

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

pub fn part_2(input: &Vec<String>) -> u32 {
    let scratch_cards: Vec<_> = input.iter().map(|line| ScratchCard::new(line)).collect();
    let scrath_card_ids: Vec<_> = scratch_cards.iter().map(|sc| sc.id).collect();
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
        let score: u32 = winnings
            .iter()
            .map(|x| memo.get(x).expect("hmm missing"))
            .sum();
        memo.insert(sc.id, score + 1);
    }

    scrath_card_ids
        .iter()
        .map(|id| memo.get(id).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_1(&lines), 13);
        let v: Vec<u32> = vec![];
        assert_eq!(v.iter().sum::<u32>(), 0);
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
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";
        
        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_2(&lines), 30)
    }
}
