use std::str::FromStr;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Pos,
    Neg,
}

pub fn is_safe(nums: &[i32]) -> bool {
    let mut safe = true;
    let mut direction: Option<Direction> = None;

    for i in 0..nums.len() - 1 {
        let a = nums[i];
        let b = nums[i + 1];
        let delta = a - b;
        let new_direction = if delta < 0 {
            Direction::Neg
        } else {
            Direction::Pos
        };

        match direction {
            None => direction = Some(new_direction),
            Some(dir) => {
                if dir != new_direction {
                    safe = false;
                }
            }
        }

        if delta.abs() > 3 || delta.abs() < 1 {
            safe = false;
        }
    }

    return safe;
}

pub fn part_1(input: &str) -> i32 {
    let mut count = 0;

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(i32::from_str)
            .map(Result::unwrap)
            .collect();

        if is_safe(&nums) {
            count += 1;
        }
    }

    count
}

pub fn part_2(input: &str) -> i32 {
    let mut count = 0;

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(i32::from_str)
            .map(Result::unwrap)
            .collect();

        if is_safe(&nums) {
            count += 1;
        } else {
            for n in 0..nums.len() {
                if n > 0 {
                    let slice = [&nums[..n], &nums[n + 1..]].concat();
                    if is_safe(&slice) {
                        count += 1;
                        break;
                    }
                } else {
                    let slice = &nums[1..];
                    if is_safe(slice) {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(part_1(input), 2);
    }

    #[test]
    fn part_2_test() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(part_2(input), 4);
    }
}
