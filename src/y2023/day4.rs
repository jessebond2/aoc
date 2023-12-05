use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct ScratchCard {
    winning_numbers: HashSet<i32>,
    numbers: HashSet<i32>,
    matched: u32,
}

impl ScratchCard {
    fn new(line: &str) -> Self {
        let rest = line.split(":").last().expect("Should be numbers left over");
        let pieces = rest.split("|").map(|str| str.trim()).collect::<Vec<&str>>();
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
        }
    }

    fn score(self) -> u32 {
        if self.matched == 0 {
            return 0;
        }

        u32::pow(2, self.matched - 1)
    }
}

pub fn part_1(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|line| ScratchCard::new(line).score())
        .sum()
}

pub fn part_2(input: &Vec<String>) -> u32 {
    0
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
        assert_eq!(part_1(&lines), 13)
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
    fn part_2_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_2(&lines), 467835)
    }
}
