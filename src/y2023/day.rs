pub fn part_1(input: &Vec<String>) -> usize {
    0
}

pub fn part_2(input: &Vec<String>) -> usize {
    0
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
    fn part_2_test() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_2(&lines), 71503);
    }
}
