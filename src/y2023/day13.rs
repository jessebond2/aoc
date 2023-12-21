pub fn part_1(input: &str) -> usize {
    for section in input.split("\n\n") {
        println!("section {}", section)
    }
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
        let input = "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#";

        assert_eq!(part_1(input), 288);
    }

    #[test]
    fn part_2_test() {
        let input = "";

        assert_eq!(part_2(input), 0);
    }
}
