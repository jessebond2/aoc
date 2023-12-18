use core::fmt;
use std::sync::mpsc::channel;
use std::time::Instant;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use workerpool::thunk::{Thunk, ThunkWorker};
use workerpool::Pool;

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

    fn get_possibilities(&self) -> usize {
        let mut possibilities: usize = 0;

        let mut stack: Vec<(SpringValidation, Vec<Part>)> =
            vec![(SpringValidation::new(), self.springs.clone())];

        while let Some((validation, mut possibility)) = stack.pop() {
            // let updated_validation = SpringGroup::is_valid_springs(self, &possibility, validation);
            // if !updated_validation.valid {
            //     continue;
            // }

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

pub fn part_2(input: &str) -> usize {
    let now = Instant::now();
    let lines: Vec<_> = input.lines().collect();
    let n_jobs = lines.len();
    let n_workers = 16;
    let pool = Pool::<ThunkWorker<usize>>::new(n_workers);

    let (tx, rx) = channel();
    for line in input.lines() {
        let string = String::from(line);
        pool.execute_to(
            tx.clone(),
            Thunk::of(move || SpringGroup::from_str2(&string).get_possibilities()),
        );
    }

    let mut job_count = 0;
    let mut sum = 0;
    for result in rx.iter().take(n_jobs) {
        job_count += 1;
        println!(
            "Elapsed time: {:?}, count: {} of {}, sum {}",
            now.elapsed(),
            job_count,
            n_jobs,
            sum
        );
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
    }

    #[test]
    fn part_2_test() {
        let input = "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";

        assert_eq!(part_2(input), 525152);
    }
}
