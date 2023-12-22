use core::fmt;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::time::Instant;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use workerpool::thunk::{Thunk, ThunkWorker};
use workerpool::Pool;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
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

#[derive(Clone, Copy)]
struct SpringValidation {
    valid: bool,
    segment_index: usize,
    completed_segments: u32,
    current_len: u32,
    building: bool,
    part_index: usize,
    done: bool,
}

impl SpringValidation {
    fn new() -> Self {
        Self {
            valid: false,
            segment_index: 0,
            completed_segments: 0,
            current_len: 0,
            building: false,
            part_index: 0,
            done: false,
        }
    }
}

struct PartCounts {
    good: usize,
    damaged: usize,
    unknown: usize,
}

#[derive(Debug, PartialEq)]
struct SpringGroup {
    springs: Vec<Part>,
    segments: Vec<u32>,
}

impl SpringGroup {
    fn from_str2(s: &str) -> SpringGroup {
        let base = SpringGroup::from_str(s).expect("Expected parsing");
        let mut springs: Vec<Part> = vec![];
        let mut segments: Vec<u32> = vec![];
        for i in 0..5 {
            springs.append(&mut base.springs.clone());
            if i < 4 {
                springs.push(Part::Unknown);
            }
            segments.append(&mut base.segments.clone());
        }

        SpringGroup { springs, segments }
    }

    fn from_str2_v2(s: &str) -> SpringGroup {
        let spring_group = Self::from_str2(s);
        let mut springs: Vec<Part> = vec![];
        let mut last = Part::Unknown;

        for part in spring_group.springs {
            if last == Part::Good && part == Part::Good {
                continue;
            }
            springs.push(part);
            last = part;
        }

        SpringGroup {
            springs,
            segments: spring_group.segments,
        }
    }

    fn get_possibilities(&self) -> usize {
        let mut possibilities: usize = 0;

        let mut stack: Vec<(SpringValidation, Vec<Part>)> =
            vec![(SpringValidation::new(), self.springs.clone())];

        while let Some((validation, mut possibility)) = stack.pop() {
            for i in validation.part_index..possibility.len() {
                let part = &possibility[i];

                if *part == Part::Unknown {
                    possibility[i] = Part::Good;
                    let new_validation =
                        SpringGroup::is_valid_springs(self, &possibility, validation);
                    if new_validation.valid {
                        stack.push((new_validation, possibility.clone()));
                    }
                    possibility[i] = Part::Damaged;
                    let new_validation =
                        SpringGroup::is_valid_springs(self, &possibility, validation);
                    if new_validation.valid {
                        stack.push((new_validation, possibility));
                    }
                    break;
                }
            }
            if validation.done {
                // println!("Possibilitiy: {:?}", possibility);
                possibilities += 1;
            }
        }

        possibilities
    }

    fn _is_valid(&self) -> bool {
        self.is_valid_springs(&self.springs, SpringValidation::new())
            .valid
    }

    fn is_valid_springs(
        &self,
        springs: &[Part],
        mut validation: SpringValidation,
    ) -> SpringValidation {
        let mut segment_iter = self.segments.iter();

        validation.valid = false;
        let mut segment = segment_iter.nth(validation.segment_index).unwrap_or(&0);

        for part in springs.iter().skip(validation.part_index) {
            match *part {
                Part::Unknown => {
                    validation.valid = true;
                    return validation;
                }
                Part::Good => {
                    if validation.building {
                        validation.building = false;
                        if validation.current_len == *segment {
                            segment = segment_iter.next().unwrap_or(&0);
                            validation.segment_index += 1;
                            validation.current_len = 0;
                            validation.completed_segments += 1;
                        } else {
                            return validation;
                        }
                    }
                }
                _ => {
                    if validation.current_len == *segment {
                        return validation;
                    }
                    validation.building = true;
                    validation.current_len += 1;
                }
            }
            validation.part_index += 1;
        }
        if validation.building {
            if validation.current_len != *segment {
                return validation;
            }
            validation.completed_segments += 1;
        }
        if validation.completed_segments as usize != self.segments.len() {
            return validation;
        }

        validation.valid = true;
        validation.done = true;
        validation
    }

    fn damaged_count(springs: &[Part]) -> usize {
        springs
            .iter()
            .filter(|part| **part == Part::Damaged)
            .collect::<Vec<_>>()
            .len()
    }

    fn dp_base_case(window: usize, springs: &[Part]) -> (u64, usize) {
        let damaged_count: usize = SpringGroup::damaged_count(springs);

        let mut value: u64 = 0;
        let mut counts = PartCounts {
            good: 0,
            damaged: 0,
            unknown: 0,
        };
        for n in 0..springs.len() {
            let part = springs[n];
            match part {
                Part::Damaged => counts.damaged += 1,
                Part::Unknown => counts.unknown += 1,
                Part::Good => counts.good += 1,
            }
            if n + 1 > window {
                let part_removal = springs[n - window];
                match part_removal {
                    Part::Damaged => counts.damaged -= 1,
                    Part::Unknown => counts.unknown -= 1,
                    Part::Good => counts.good -= 1,
                }
            }
            if n + 1 >= window && counts.good == 0 && counts.damaged >= damaged_count {
                value += 1;
            }
        }

        (value, damaged_count)
    }

    fn dp_possibilities(&self) -> u64 {
        let mut memo: HashMap<(&[Part], &[u32]), (u64, usize)> = HashMap::new();
        fn helper<'a>(
            springs: &'a [Part],
            segments: &'a [u32],
            memo: &mut HashMap<(&'a [Part], &'a [u32]), (u64, usize)>,
        ) -> (u64, usize) {
            let sum = match memo.get(&(springs, segments)) {
                Some(value) => value,
                None => {
                    if springs.is_empty() || (springs.len() == 1 && segments[0] > 1) {
                        memo.insert((springs, segments), (0, 0));
                        return (0, 0);
                    }
                    if segments.len() == 1 && (springs.len() < segments[0] as usize) {
                        memo.insert((springs, segments), (0, 0));
                        return (0, 0);
                    }

                    if segments.len() == 1 {
                        let result = SpringGroup::dp_base_case(segments[0] as usize, springs);

                        memo.insert((springs, segments), result);
                        return *memo.get(&(springs, segments)).unwrap();
                    }

                    let target = SpringGroup::damaged_count(springs);

                    let mut value: u64 = 0;
                    let mut damaged_value: usize = 0;
                    let window =
                        segments.iter().map(|s| *s as usize).sum::<usize>() + segments.len() - 1;
                    if window == springs.len() {
                        let start = 0;
                        let end = segments[0] as usize;
                        let start2 = end + 1;
                        let v1 = helper(&springs[start..end], &segments[..1], memo);
                        let mut v2 = (1, 0);
                        if segments.len() > 1 {
                            v2 = helper(&springs[start2..], &segments[1..], memo);
                        }
                        if springs[end] == Part::Damaged {
                            v2 = (0, 0);
                        }

                        if v1.1 + v2.1 == target && v1.0 * v2.0 > 0 {
                            value += v1.0 * v2.0;
                            damaged_value = target;
                        }
                    } else {
                        for n in 0..=springs.len() - window {
                            let start = n;
                            let end = n + segments[0] as usize;
                            if springs[end] == Part::Damaged {
                                continue;
                            }
                            let start2 = end + 1;
                            let v1 = helper(&springs[start..end], &segments[..1], memo);
                            let mut v2 = (1, 0);
                            if segments.len() > 1 {
                                v2 = helper(&springs[start2..], &segments[1..], memo);
                            }
                            if v1.1 + v2.1 == target && v1.0 * v2.0 > 0 {
                                value += v1.0 * v2.0;
                                damaged_value = target;
                            }
                        }
                    }
                    if damaged_value == target {
                        memo.insert((springs, segments), (value, damaged_value));
                    } else {
                        memo.insert((springs, segments), (0, 0));
                    }
                    memo.get(&(springs, segments)).unwrap()
                }
            };

            *sum
        }

        let result = helper(&self.springs, &self.segments, &mut memo).0;
        if result == 0 {
            println!("Probably a problem with {:?}", self);
        }
        result
    }
}

#[derive(Debug, PartialEq)]
struct ParseSpringGroupError;

impl FromStr for SpringGroup {
    type Err = ParseSpringGroupError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split(' ');
        let springs: Vec<Part> = iter
            .next()
            .unwrap()
            .chars()
            .map(|char| match char {
                '.' => Part::Good,
                '#' => Part::Damaged,
                _ => Part::Unknown,
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

pub fn part_2(input: &str) -> u64 {
    let now = Instant::now();
    let lines: Vec<_> = input.lines().collect();
    let n_jobs = lines.len();
    let n_workers = 32;
    let pool = Pool::<ThunkWorker<u64>>::new(n_workers);

    let (tx, rx) = channel();
    for line in input.lines() {
        let string = String::from(line);
        pool.execute_to(
            tx.clone(),
            Thunk::of(move || SpringGroup::from_str2_v2(&string).dp_possibilities()),
        );
    }

    // let mut job_count = 0;
    let mut sum = 0;
    for result in rx.iter().take(n_jobs) {
        // job_count += 1;
        // println!(
        //     "Elapsed time: {:?}, count: {} of {}, sum {}",
        //     now.elapsed(),
        //     job_count,
        //     n_jobs,
        //     sum
        // );
        sum += result;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";

        assert_eq!(part_1(input), 21);
    }

    #[test]
    fn is_valid() {
        assert!(!SpringGroup::from_str(".###.##..... 3,2,1")
            .unwrap()
            ._is_valid());
        assert!(!SpringGroup::from_str("####???????? 3,2,1")
            .unwrap()
            ._is_valid());

        assert!(SpringGroup::from_str("#.#.### 1,1,3").unwrap()._is_valid());
        assert!(SpringGroup::from_str(".###.##.#... 3,2,1")
            .unwrap()
            ._is_valid());
        assert!(!SpringGroup::from_str("#.##.##.#... 3,2,1")
            .unwrap()
            ._is_valid(),);
        assert!(!SpringGroup::from_str("...### 2,1").unwrap()._is_valid(),);
        assert!(SpringGroup::from_str("...### 3").unwrap()._is_valid());
        assert!(SpringGroup::from_str("?..### 3").unwrap()._is_valid());
        assert!(!SpringGroup::from_str("##.##? 3").unwrap()._is_valid());
    }

    #[test]
    fn parse_spring_group() {
        let input = "#.#.### 1,1,3";

        assert_eq!(
            SpringGroup::from_str(input).unwrap(),
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
    fn parse_spring_group2() {
        let input = ".# 1";

        assert_eq!(
            SpringGroup::from_str2(input),
            SpringGroup::from_str(".#?.#?.#?.#?.# 1,1,1,1,1").unwrap()
        );

        assert_eq!(
            SpringGroup::from_str(
                "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"
            )
            .unwrap(),
            SpringGroup::from_str2("???.### 1,1,3")
        );
    }

    #[test]
    fn get_possibilities() {
        assert_eq!(
            SpringGroup::from_str("?###???????? 3,2,1")
                .unwrap()
                .get_possibilities(),
            10
        )
    }

    #[test]
    fn get_possibilities2() {
        assert_eq!(
            SpringGroup::from_str2("???.### 1,1,3").get_possibilities(),
            1
        );
        // assert_eq!(
        //     SpringGroup::from_str2("?.#?????.?????????#. 1,2,1,4,1,2").get_possibilities(),
        //     1
        // );
        assert_eq!(
            SpringGroup::from_str(".??????. 3,1")
                .unwrap()
                .get_possibilities(),
            3
        );
    }

    #[test]
    fn dp_possibilities_test() {
        assert_eq!(
            SpringGroup::from_str2_v2("???.### 1,1,3").dp_possibilities(),
            1
        );
        assert_eq!(
            SpringGroup::from_str(".??????. 3,1")
                .unwrap()
                .dp_possibilities(),
            3
        );
        assert_eq!(
            SpringGroup::from_str(".???#??. 3,1")
                .unwrap()
                .dp_possibilities(),
            1
        );
        assert_eq!(
            SpringGroup::from_str("??????? 2,1")
                .unwrap()
                .dp_possibilities(),
            10
        );
        assert_eq!(
            SpringGroup::from_str("?###???????? 3,2,1")
                .unwrap()
                .dp_possibilities(),
            10
        );

        assert_eq!(
            SpringGroup::from_str(".?##?.?? 3,1")
                .unwrap()
                .dp_possibilities(),
            4
        );
        assert_eq!(
            SpringGroup::from_str(".?##?.??.?? 3,1,1")
                .unwrap()
                .dp_possibilities(),
            8
        );
        assert_eq!(
            SpringGroup::from_str(".??.??.??.??.??.??. 1,1,1,1,1,1")
                .unwrap()
                .dp_possibilities(),
            64
        );
        assert_eq!(
            SpringGroup::from_str(".??.??.?##?.??.?##?.??. 1,1,3,1,3,1")
                .unwrap()
                .dp_possibilities(),
            64
        );

        assert_eq!(
            SpringGroup::from_str2_v2(".??..??...?##. 1,1,3").dp_possibilities(),
            16384
        );
        assert_eq!(
            SpringGroup::from_str2_v2(".??..??...?##. 1,1,3").dp_possibilities(),
            16384
        );
        assert_eq!(
            SpringGroup::from_str2_v2("????.######..#####. 1,6,5").dp_possibilities(),
            2500
        );
        assert_eq!(
            SpringGroup::from_str("???###? 1")
                .unwrap()
                .dp_possibilities(),
            0
        );
        assert_eq!(
            SpringGroup::from_str2_v2("?###???????? 3,2,1").dp_possibilities(),
            506250
        );
    }

    #[test]
    fn part_2_test() {
        let input = String::from(
            "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1",
        );

        assert_eq!(part_2(&input), 525152);
    }

    #[test]
    fn from_str2_v2_test() {
        assert_eq!(
            SpringGroup::from_str2_v2("......# 1"),
            SpringGroup::from_str2(".# 1")
        );
    }

    #[test]
    fn dp_base_case_test() {
        let sg = SpringGroup::from_str(".?##??. 3").unwrap();
        let sg2 = SpringGroup::from_str(".?##.#. 3").unwrap();
        let sg3 = SpringGroup::from_str("### 3").unwrap();
        let sg4 = SpringGroup::from_str(".?.?.#?. 3").unwrap();
        let sg5 = SpringGroup::from_str(".?.?#?. 3").unwrap();

        assert_eq!(SpringGroup::dp_base_case(4, &sg.springs), (2, 2));
        assert_eq!(SpringGroup::dp_base_case(3, &sg.springs), (2, 2));
        assert_eq!(SpringGroup::dp_base_case(2, &sg.springs), (1, 2));
        assert_eq!(SpringGroup::dp_base_case(2, &sg2.springs), (0, 3));
        assert_eq!(SpringGroup::dp_base_case(3, &sg2.springs), (0, 3));
        assert_eq!(SpringGroup::dp_base_case(4, &sg2.springs), (0, 3));
        assert_eq!(SpringGroup::dp_base_case(5, &sg2.springs), (0, 3));
        assert_eq!(SpringGroup::dp_base_case(1, &sg3.springs), (0, 3));
        assert_eq!(SpringGroup::dp_base_case(2, &sg3.springs), (0, 3));
        assert_eq!(SpringGroup::dp_base_case(1, &sg4.springs), (1, 1));
        assert_eq!(SpringGroup::dp_base_case(2, &sg4.springs), (1, 1));
        assert_eq!(SpringGroup::dp_base_case(1, &sg5.springs), (1, 1));
        assert_eq!(SpringGroup::dp_base_case(2, &sg5.springs), (2, 1));
    }
}
