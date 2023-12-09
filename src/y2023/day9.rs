fn next_in_sequence_helper(numbers: Vec<i32>) -> i32 {
    let mut deltas: Vec<i32> = vec![];

    if numbers.iter().sum::<i32>() == 0 {
        return 0;
    }

    for i in 0..(numbers.len() - 1) {
        deltas.push(numbers[i + 1] - numbers[i]);
    }

    numbers[numbers.len() - 1] + next_in_sequence_helper(deltas)
}

fn next_in_sequence(line: &str) -> i32 {
    let numbers: Vec<i32> = line
        .split(" ")
        .map(|raw| raw.parse::<i32>().unwrap_or(0))
        .collect();

    next_in_sequence_helper(numbers)
}

fn previous_in_sequence_helper(numbers: Vec<i32>) -> i32 {
    let mut deltas: Vec<i32> = vec![];

    if numbers.iter().sum::<i32>() == 0 {
        return 0;
    }

    for i in 0..(numbers.len() - 1) {
        deltas.push(numbers[i + 1] - numbers[i]);
    }

    numbers[0] - previous_in_sequence_helper(deltas)
}

fn previous_in_sequence(line: &str) -> i32 {
    let numbers: Vec<i32> = line
        .split(" ")
        .map(|raw| raw.parse::<i32>().unwrap_or(0))
        .collect();

    previous_in_sequence_helper(numbers)
}

pub fn part_1(input: &Vec<String>) -> i32 {
    input.iter().map(|line| next_in_sequence(line)).sum()
}

pub fn part_2(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(|line: &String| previous_in_sequence(line))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_1(&lines), 114);
    }

    #[test]
    fn next_in_sequence_helper_test() {
        assert_eq!(next_in_sequence_helper(vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(next_in_sequence_helper(vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(next_in_sequence_helper(vec![10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn part_2_test() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_2(&lines), 2);
    }

    #[test]
    fn previous_in_sequence_helper_test() {
        assert_eq!(previous_in_sequence_helper(vec![0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(previous_in_sequence_helper(vec![1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(previous_in_sequence_helper(vec![10, 13, 16, 21, 30, 45]), 5);
    }
}
