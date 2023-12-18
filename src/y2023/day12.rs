use core::fmt;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Part {
    Good,
    Damaged,
    Unknown,
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Part::Good => write!(f, "."),
            Part::Damaged => write!(f, "#"),
            Part::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct SpringGroup {
    springs: Vec<Part>,
    segments: Vec<u32>,
}

impl SpringGroup {
    fn get_possibilities(&self) -> usize {
        let mut possibilities: Vec<Vec<Part>> = vec![];

        let mut stack: Vec<Vec<Part>> = vec![self.springs.clone()];

        loop {
            match stack.pop() {
                Some(possibility) => {
                    if !SpringGroup::is_valid_springs(&self, &possibility) {
                        continue;
                    }

                    for i in 0..possibility.len() {
                        let part = &possibility[i];

                        match *part {
                            Part::Unknown => {
                                let mut good = possibility.clone();
                                good[i] = Part::Good;
                                stack.push(good);
                                let mut damaged = possibility.clone();
                                damaged[i] = Part::Damaged;
                                stack.push(damaged);
                                break;
                            }
                            _ => {
                                if i == possibility.len() - 1
                                    && SpringGroup::is_valid_springs(&self, &possibility)
                                {
                                    // println!("Possibilitiy: {:?}", possibility);
                                    possibilities.push(possibility.clone());
                                }
                            }
                        }
                    }
                }
                None => break,
            }
        }

        possibilities.len()
    }

    fn is_valid(&self) -> bool {
        self.is_valid_springs(&self.springs)
    }

    fn is_valid_springs(&self, springs: &Vec<Part>) -> bool {
        let mut segment_iter = self.segments.iter();
        let mut segment = segment_iter.next().unwrap();
        let mut completed_segments = 0;
        let mut current_len = 0;
        let mut building = false;

        for i in 0..springs.len() {
            let part = &springs[i];
            match *part {
                Part::Unknown => return true,
                Part::Good => {
                    if building {
                        building = false;
                        if current_len == *segment {
                            segment = segment_iter.next().unwrap_or(&0);
                            current_len = 0;
                            completed_segments += 1;
                        } else {
                            return false;
                        }
                    }
                }
                _ => {
                    if current_len == *segment {
                        return false;
                    }
                    building = true;
                    current_len += 1;
                }
            }
        }
        if building {
            if current_len != *segment {
                return false;
            }
            completed_segments += 1;
        }
        if completed_segments != self.segments.len() {
            return false;
        }

        true
    }
}

#[derive(Debug, PartialEq)]
struct ParseSpringGroupError;

impl FromStr for SpringGroup {
    type Err = ParseSpringGroupError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let springs: Vec<Part> = iter
            .next()
            .unwrap()
            .chars()
            .map(|char| {
                let part = match char {
                    '.' => Part::Good,
                    '#' => Part::Damaged,
                    _ => Part::Unknown,
                };
                part
            })
            .collect();
        let segments: Vec<u32> = iter
            .next()
            .unwrap()
            .split(',')
            .map(|piece| piece.parse::<u32>().expect("NaN"))
            .collect();

        Ok(SpringGroup { springs, segments })
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            SpringGroup::from_str(line)
                .expect("Issue parsing SpringGroup")
                .get_possibilities()
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "#.#.### 1,1,3
        .#...#....###. 1,1,3
        .#.###.#.###### 1,3,1,6
        ####.#...#... 4,1,1
        #....######..#####. 1,6,5
        .###.##....# 3,2,1";

        assert_eq!(part_1(input), 21);
    }

    #[test]
    fn is_valid() {
        assert_eq!(
            SpringGroup::from_str(&".###.##..... 3,2,1")
                .unwrap()
                .is_valid(),
            false
        );
        assert_eq!(
            SpringGroup::from_str(&"####???????? 3,2,1")
                .unwrap()
                .is_valid(),
            false
        );

        assert_eq!(
            SpringGroup::from_str(&"#.#.### 1,1,3").unwrap().is_valid(),
            true
        );
        assert_eq!(
            SpringGroup::from_str(&".###.##.#... 3,2,1")
                .unwrap()
                .is_valid(),
            true
        );
        assert_eq!(
            SpringGroup::from_str(&"#.##.##.#... 3,2,1")
                .unwrap()
                .is_valid(),
            false
        );
        assert_eq!(
            SpringGroup::from_str(&"...### 2,1").unwrap().is_valid(),
            false
        );
        assert_eq!(SpringGroup::from_str(&"...### 3").unwrap().is_valid(), true);
        assert_eq!(SpringGroup::from_str(&"?..### 3").unwrap().is_valid(), true);
        assert_eq!(
            SpringGroup::from_str(&"##.##? 3").unwrap().is_valid(),
            false
        );
    }

    #[test]
    fn parse_spring_group() {
        let input = "#.#.### 1,1,3";

        assert_eq!(
            SpringGroup::from_str(&input).unwrap(),
            SpringGroup {
                springs: vec![
                    Part::Damaged,
                    Part::Good,
                    Part::Damaged,
                    Part::Good,
                    Part::Damaged,
                    Part::Damaged,
                    Part::Damaged
                ],
                segments: vec![1, 1, 3]
            }
        )
    }

    #[test]
    fn get_possibilities() {
        println!("abc {}", Part::Damaged);
        assert_eq!(
            SpringGroup::from_str(&"?###???????? 3,2,1")
                .unwrap()
                .get_possibilities(),
            10
        )
    }

    // #[test]
    // fn part_2_test() {
    //     let input = "Time:      7  15   30
    //     Distance:  9  40  200";

    //     let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
    //     assert_eq!(part_2(&lines), 71503);
    // }
}
