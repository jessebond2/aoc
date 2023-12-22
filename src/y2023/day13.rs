use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct MirrorField {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

impl MirrorField {
    fn row_reflection(&self) -> usize {
        MirrorField::reflection(&self.rows)
    }

    fn col_reflection(&self) -> usize {
        MirrorField::reflection(&self.cols)
    }

    fn is_reflection(vec: &Vec<u32>, n: usize) -> bool {
        let mut n = n;
        let mut m = n + 1;
        while m < vec.len() {
            if vec[n] != vec[m] {
                return false;
            }
            if n == 0 {
                break;
            }
            n -= 1;
            m += 1;
        }

        true
    }

    fn reflection(vec: &Vec<u32>) -> usize {
        let mut result = None;
        let mut m = vec.len() / 2 + 1;
        for n in (0..=(vec.len() / 2)).rev() {
            if Self::is_reflection(vec, n) {
                result = Some(n);
                break;
            }
            if m + 1 < vec.len() && Self::is_reflection(vec, m) {
                result = Some(m);
                break;
            }
            m += 1;
        }

        if let Some(result) = result {
            result + 1
        } else {
            0
        }
    }

    fn score(&self) -> usize {
        self.row_reflection() * 100 + self.col_reflection()
    }
}

#[derive(Debug, PartialEq)]
struct ParseMirrorFieldError;

impl FromStr for MirrorField {
    type Err = ParseMirrorFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars: Vec<Vec<char>> = vec![];
        let mut rows = vec![];
        let mut cols = vec![];

        for row in s.lines() {
            let mut number: u32 = 0b0;
            let char_row: Vec<_> = row.trim().chars().collect();

            for char in char_row.iter() {
                let bit: u32 = match char {
                    '.' => 0b0,
                    _ => 0b1,
                };
                number = (number << 1) + bit;
            }
            chars.push(char_row);
            rows.push(number);
        }

        for col in 0..chars[0].len() {
            let mut number: u32 = 0b0;

            for row in &chars {
                let char = row[col];
                let bit: u32 = match char {
                    '.' => 0b0,
                    _ => 0b1,
                };
                number = (number << 1) + bit;
            }
            cols.push(number);
        }

        Ok(MirrorField { rows, cols })
    }
}

pub fn part_1(input: &str) -> usize {
    let mut mirror_fields = vec![];
    for section in input.split("\n\n") {
        mirror_fields.push(MirrorField::from_str(section).unwrap());
    }

    mirror_fields.iter().map(|mf| mf.score()).sum()
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

        assert_eq!(part_1(input), 405);
    }

    #[test]
    fn score_test() {
        let input = "###.#....#.
        #..########
        ...##.##.##
        .#..#....#.
        .###..##..#
        .#.#.#..#.#
        .#.#.#..#.#
        .###..##..#
        .#..#....#.
        ...##.##.##
        #..########
        #.#.#....#.
        ##.#......#
        #.....##...
        ##.##....##";

        assert_eq!(MirrorField::from_str(input).unwrap().score(), 7);

        let input = "#...##.##..
        #...##.##..
        .#.#..###..
        ##.#.#.##..
        .#.####..##
        .##.##.#..#
        #.#####.##.
        .#.#.#..#.#
        ..#...#.#..
        .#####..#..
        ##...#.#...
        ##...#.#...
        .#####..#..
        ..##..#.#..
        .#.#.#..#.#
        #.#####.##.
        .##.##.#..#";

        assert_eq!(MirrorField::from_str(input).unwrap().score(), 100);
    }

    #[test]
    fn is_reflection_test() {
        let v: Vec<u32> = vec![1, 2, 3, 4, 4, 3, 2, 1];
        assert!(!MirrorField::is_reflection(&v, 2));
        assert!(MirrorField::is_reflection(&v, 3));
        assert!(!MirrorField::is_reflection(&v, 4));

        let v: Vec<u32> = vec![1, 2, 3, 4, 4, 3, 2, 1, 0];
        assert!(!MirrorField::is_reflection(&v, 2));
        assert!(MirrorField::is_reflection(&v, 3));
        assert!(!MirrorField::is_reflection(&v, 4));

        let v: Vec<u32> = vec![1, 2, 3, 4, 4, 3, 2];
        assert!(!MirrorField::is_reflection(&v, 2));
        assert!(MirrorField::is_reflection(&v, 3));
        assert!(!MirrorField::is_reflection(&v, 4));

        let v: Vec<u32> = vec![0, 0, 0, 0, 1, 1, 0, 0, 1, 1];
        assert!(!MirrorField::is_reflection(&v, 2));
        assert!(!MirrorField::is_reflection(&v, 4));
        assert!(MirrorField::is_reflection(&v, 6));
    }

    #[test]
    fn reflection_test() {
        let v: Vec<u32> = vec![0, 0, 0, 0, 1, 1, 0, 0, 1, 1];
        assert_eq!(MirrorField::reflection(&v), 7);
    }

    #[test]
    fn part_2_test() {
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

        assert_eq!(part_2(input), 0);
    }

    #[test]
    fn bit_count() {
        let mut a: u8 = 0b010110;
        let mut count = 0;
        for b in 0..8 {
            count += a & 1;
            a = a >> 1;
        }

        assert_eq!(count, 3);
    }
}
