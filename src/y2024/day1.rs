use std::str::FromStr;

pub fn part_1(input: &str) -> i32 {
    let mut l: Vec<i32> = vec![];
    let mut r: Vec<i32> = vec![];

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(i32::from_str)
            .map(Result::unwrap)
            .collect();
        l.push(nums[0]);
        r.push(nums[1]);
    }

    l.sort();
    r.sort();

    let sum: i32 = l
        .iter()
        .zip(r.iter())
        .map(|(lval, rval)| (lval - rval).abs())
        .sum();
    sum
}

pub fn part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(part_1(input), 11);
    }

    #[test]
    fn part_2_test() {
        let input = "";

        assert_eq!(part_2(input), 0);
    }
}
