pub fn part_1(_input: &str) -> u32 {
    0
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

        assert_eq!(part_2(input), 71503);
    }
}
