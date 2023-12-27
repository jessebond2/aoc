fn hash_value(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, char| ((acc + (char as u32)) * 17) % 256)
}

pub fn part_1(input: &str) -> u32 {
    input.split(',').map(hash_value).sum()
}

pub fn part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(part_1(input), 1320);
    }

    #[test]
    fn hash_value_test() {
        assert_eq!(hash_value("HASH"), 52);
        assert_eq!(hash_value("rn=1"), 30);
    }

    #[test]
    fn part_2_test() {
        let input = "";

        assert_eq!(part_2(input), 71503);
    }
}
