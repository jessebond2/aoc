use std::str::FromStr;

fn bit_count(a: u32) -> u32 {
    let mut a: u32 = a;
    let mut count = 0;
    for _ in 0..32 {
        count += a & 1;
        a >>= 1;
    }
    count
}

fn bit_difference(a: u32, b: u32) -> u32 {
    bit_count(a ^ b)
}

fn bit_difference_index(a: u32, b: u32) -> u32 {
    let mut index = 0;
    let mut diff = a ^ b;
    for idx in 0..32 {
        if diff & 1 == 1 {
            index = idx;
            break;
        }
        index += 1;
        diff >>= 1;
    }
    index
}

fn flip_bit(a: u32, index: usize) -> u32 {
    let mask = 0b1 << index;
    a ^ mask
}

#[derive(Debug, PartialEq)]
struct MirrorField {
    rows: Vec<u32>,
    cols: Vec<u32>,
    string: String,
}

impl MirrorField {
    fn row_reflection(&self) -> Option<usize> {
        Self::reflection(&self.rows, None)
    }

    fn col_reflection(&self) -> Option<usize> {
        Self::reflection(&self.cols, None)
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

    fn find_single_bit_difference(vec: &Vec<u32>, n: usize) -> Vec<(usize, usize)> {
        let mut n = n;
        let mut m = n + 1;
        let mut out = vec![];

        while m < vec.len() {
            if vec[n] != vec[m] && bit_difference(vec[n], vec[m]) == 1 {
                out.push((n, m))
            }
            if n == 0 {
                break;
            }
            n -= 1;
            m += 1;
        }

        out
    }

    fn find_all_differences(vec: &Vec<u32>) -> Vec<(usize, usize)> {
        let mut result = vec![];
        for n in 0..vec.len() - 1 {
            result.append(&mut Self::find_single_bit_difference(vec, n))
        }
        result
    }

    fn reflection(vec: &Vec<u32>, skip: Option<usize>) -> Option<usize> {
        let mut result = None;
        let mut m = vec.len() / 2 + 1;
        for n in (0..=(vec.len() / 2)).rev() {
            if Self::is_reflection(vec, n) && skip != Some(n + 1) {
                result = Some(n);
                break;
            }
            if m + 1 < vec.len() && Self::is_reflection(vec, m) && skip != Some(m + 1) {
                result = Some(m);
                break;
            }
            m += 1;
        }

        result.map(|result| result + 1)
    }

    fn score(&self) -> usize {
        self.row_reflection().unwrap_or(0) * 100 + self.col_reflection().unwrap_or(0)
    }

    fn score_2(&self, rows: &Vec<u32>, cols: &Vec<u32>) -> usize {
        let old_row = Self::row_reflection(self);
        let new_row = Self::reflection(rows, old_row);
        if old_row != new_row {
            if let Some(new_row) = new_row {
                return new_row * 100;
            }
        }

        let old_col = Self::col_reflection(self);
        Self::reflection(cols, old_col).unwrap_or(0)
    }

    fn smudge_score(&self) -> usize {
        let original_score = Self::score(self);
        // println!("Self {:?}", self);
        let row_possibilities = Self::find_all_differences(&self.rows);
        for pos in row_possibilities {
            let mut new_row = self.rows.clone();
            let mut new_col = self.cols.clone();
            let column_bit_index = bit_difference_index(new_row[pos.0], new_row[pos.1]);
            let column = new_col.len() - column_bit_index as usize - 1;

            // println!("Posibilities in row {} and {}", pos.0, pos.1);
            // println!(
            //     "Should update column {} from bit index {}",
            //     column, column_bit_index
            // );
            new_row[pos.0] = flip_bit(new_row[pos.0], column_bit_index as usize);
            // println!(
            //     "Row before {:b}, after {:b}",
            //     self.rows[pos.0], new_row[pos.0]
            // );
            new_col[column] = flip_bit(new_col[column], new_row.len() - 1 - column);
            // println!(
            //     "Col before {:b}, after {:b}",
            //     self.cols[column], new_col[column]
            // );
            let new_score = Self::score_2(self, &new_row, &new_col);
            if new_score > 0 && new_score != original_score {
                return new_score;
            }

            let mut new_row = self.rows.clone();
            let mut new_col = self.cols.clone();
            new_row[pos.1] = flip_bit(new_row[pos.1], column_bit_index as usize);
            new_col[column] = flip_bit(new_col[column], new_row.len() - 1 - pos.1);
            let new_score = Self::score_2(self, &new_row, &new_col);
            if new_score > 0 && new_score != original_score {
                return new_score;
            }
        }
        let col_possibilities = Self::find_all_differences(&self.cols);
        for pos in col_possibilities {
            let mut new_row = self.rows.clone();
            let mut new_col = self.cols.clone();
            let row_bit_index = bit_difference_index(new_col[pos.0], new_col[pos.1]);
            let row = new_row.len() - row_bit_index as usize - 1;

            new_col[pos.0] = flip_bit(new_col[pos.0], row_bit_index as usize);
            new_row[row] = flip_bit(new_row[row], new_row.len() - 1 - row);

            let new_score = Self::score_2(self, &new_row, &new_col);
            if new_score > 0 && new_score != original_score {
                return new_score;
            }

            let mut new_row = self.rows.clone();
            let mut new_col = self.cols.clone();
            new_col[pos.1] = flip_bit(new_col[pos.1], row_bit_index as usize);
            if new_row.len() < 1 + pos.1 {
                continue;
            }
            new_row[row] = flip_bit(new_row[row], new_row.len() - 1 - pos.1);
            let new_score = Self::score_2(self, &new_row, &new_col);
            if new_score > 0 && new_score != original_score {
                return new_score;
            }
        }

        println!("Problem with:\n{}", self.string);
        0
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

        Ok(MirrorField {
            rows,
            cols,
            string: String::from(s),
        })
    }
}

pub fn part_1(input: &str) -> usize {
    let mut mirror_fields = vec![];
    for section in input.split("\n\n") {
        mirror_fields.push(MirrorField::from_str(section).unwrap());
    }

    mirror_fields.iter().map(|mf| mf.score()).sum()
}

pub fn part_2(input: &str) -> usize {
    let mut mirror_fields = vec![];
    for section in input.split("\n\n") {
        mirror_fields.push(MirrorField::from_str(section).unwrap());
    }

    mirror_fields.iter().map(|mf| mf.smudge_score()).sum()
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
        assert_eq!(MirrorField::reflection(&v, None).unwrap(), 7);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            part_2(
                "#.##..##.
                ..#.##.#.
                ##......#
                ##......#
                ..#.##.#.
                ..##..##.
                #.#.##.#."
            ),
            300
        );
        assert_eq!(
            part_2(
                "#...##..#
                #....#..#
                ..##..###
                #####.##.
                #####.##.
                ..##..###
                #....#..#"
            ),
            100
        );
    }

    #[test]
    fn part_2_input_test() {
        assert_ne!(
            part_2(
                "##.#.#####.....
        ##...##.###.###
        ##...##.###.###
        ##.#.#####.....
        .##...#.#...#..
        ##..#...#..##..
        ###.######....#
        .##.######..###
        #...#...###....
        ..#.....#.##...
        ....#......#.##"
            ),
            0
        );
        assert_ne!(
            part_2(
                "#.###..
        #.###.#
        ##..##.
        ......#
        ..##...
        #....#.
        #....#."
            ),
            0
        );
                assert_ne!(
                    part_2(
                        "#.####.
        ####..#
        #####..
        ....##.
        #..###.
        ####..#
        ####..#"
                    ),
                    0
                );
                assert_ne!(
                    part_2(
                        ".........##..##..
        ........#..##..#.
        .######..######..
        ..#..#..##.##.##.
        .###.##..#.##.#..
        ###..####.####.##
        ##....##.#....#.#"
                    ),
                    0
                );
        assert_ne!(
            part_2(
                "####.#.......
#.#.###.####.
.#..##.####..
.....#.#....#
.#.#...##...#
.#.#...##...#
.....#.#....#
.#..##.####..
#.#.###.####.
####.#.......
..#.....#.#..
#.#..#.##...#
#...#......#.
##.#..####.##
.#.#..####.##"
            ),
            0
        );
    }

    #[test]
    fn bit_count_test() {
        assert_eq!(bit_count(0b010110), 3);
        assert_eq!(bit_count(0b0111010110), 6);
    }

    #[test]
    fn bit_difference_test() {
        assert_eq!(bit_difference(0b0000, 0b1000), 1);
        assert_eq!(bit_difference(0b0000, 0b1010), 2);
        assert_eq!(bit_difference(0b1000, 0b1000), 0);
    }

    #[test]
    fn bit_difference_index_test() {
        assert_eq!(bit_difference_index(0b0000, 0b1000), 3);
        assert_eq!(bit_difference_index(0b1100, 0b1000), 2);
    }

    #[test]
    fn flip_bit_test() {
        assert_eq!(flip_bit(0b0000, 3), 0b1000);
        assert_eq!(flip_bit(0b0111, 1), 0b0101);
    }
}
