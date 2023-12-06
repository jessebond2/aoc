use std::iter::zip;

#[derive(Debug, Clone, PartialEq)]
struct Race {
    time: u64,
    distance: u64,
    winning_numbers: Vec<u64>,
}

impl Race {
    fn new(time: u64, distance: u64) -> Race {
        let winning_numbers: Vec<u64> = (0..time).filter(|t| t * (time - t) > distance).collect();
        Race {
            time,
            distance,
            winning_numbers,
        }
    }
}

pub fn part_1(input: &Vec<String>) -> usize {
    let mut iter = input.iter();
    let times: Vec<_> = iter
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap_or(0))
        .collect();
    let distances: Vec<_> = iter
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap_or(0))
        .collect();

    let races = zip(times, distances).map(|(time, distance)| Race::new(time, distance));

    races.fold(1, |acc, race| acc * race.winning_numbers.len())
}

pub fn part_2(input: &Vec<String>) -> usize {
    let mut iter = input.iter();
    let time = iter
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap_or(0);
    let distance = iter
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap_or(0);

    let race = Race::new(time, distance);

    race.winning_numbers.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_1(&lines), 288);
    }

    #[test]
    fn new_race() {
        assert_eq!(
            Race::new(7, 9),
            Race {
                time: 7,
                distance: 9,
                winning_numbers: vec![2, 3, 4, 5],
            },
        );
        assert_eq!(Race::new(15, 40).winning_numbers.len(), 8,);
        assert_eq!(Race::new(30, 200).winning_numbers.len(), 9,);
    }

    #[test]
    fn part_2_test() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_2(&lines), 71503);
    }
}
