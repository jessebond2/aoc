use core::slice::Iter;

#[derive(Debug, PartialEq)]
struct BingoPosition {
    x: usize,
    y: usize,
    value: i32,
    marked: bool,
}

impl BingoPosition {
    pub fn build(value_str: &str, x: usize, y: usize) -> BingoPosition {
        let value = value_str.parse::<i32>().unwrap();
        BingoPosition {
            x,
            y,
            value,
            marked: false,
        }
    }

    // pub fn mark(&mut self) {
    //     self.marked = true;
    // }
}

#[derive(Debug, PartialEq)]
struct BingoBoard {
    rows: Vec<Vec<Box<BingoPosition>>>,
    bingo: Vec<Box<BingoPosition>>,
}

impl BingoBoard {
    fn build() -> BingoBoard {
        BingoBoard {
            rows: vec![],
            bingo: vec![],
        }
    }

    fn add_row(&mut self, row_str: &str) {
        let positions: Vec<Box<BingoPosition>> = row_str
            .split_whitespace()
            .enumerate()
            .map(|(index, value)| {
                Box::new(BingoPosition::build(
                    value,
                    index.try_into().unwrap(),
                    self.rows.len(),
                ))
            })
            .collect();
        self.rows.push(positions);
    }

    // fn mark(&mut self, value: i32) {
    //     'outer: for i in self.rows.iter_mut() {
    //         for bp in i.iter_mut() {
    //             if value == bp.value {
    //                 bp.mark();
    //                 break 'outer;
    //             }
    //         }
    //     }
    // }

    // fn check_row(&mut self, i: usize) {
    //     let row = &self.rows[i];
    //     let has_bingo = row.iter().filter(|bp| bp.marked).collect::<Vec<_>>().len() == 5;
    //     if (has_bingo) {
    //         self.bingo = (&row).to_vec();
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bingo_board_add_row() {
        let input = " 1  2  3  4 50";
        let mut board = BingoBoard::build();
        board.add_row(input);

        assert_eq!(board.rows.len(), 1);
        assert_eq!(
            board.rows[0],
            vec![
                BingoPosition {
                    value: 1,
                    x: 0,
                    y: 0,
                    marked: false
                },
                BingoPosition {
                    value: 2,
                    x: 1,
                    y: 0,
                    marked: false
                },
                BingoPosition {
                    value: 3,
                    x: 2,
                    y: 0,
                    marked: false
                },
                BingoPosition {
                    value: 4,
                    x: 3,
                    y: 0,
                    marked: false
                },
                BingoPosition {
                    value: 50,
                    x: 4,
                    y: 0,
                    marked: false
                }
            ]
        );
    }

    #[test]
    fn bingo_board_mark() {
        let input = " 1  2  3  4 50";
        let mut board = BingoBoard::build();
        board.add_row(input);
        board.mark(3);

        assert_eq!(
            board.rows[0],
            vec![
                BingoPosition {
                    value: 1,
                    x: 0,
                    y: 0,
                    marked: false
                },
                BingoPosition {
                    value: 2,
                    x: 1,
                    y: 0,
                    marked: false
                },
                BingoPosition {
                    value: 3,
                    x: 2,
                    y: 0,
                    marked: true
                },
                BingoPosition {
                    value: 4,
                    x: 3,
                    y: 0,
                    marked: false
                },
                BingoPosition {
                    value: 50,
                    x: 4,
                    y: 0,
                    marked: false
                }
            ]
        );
    }

    #[test]
    fn build_boards_test() {
        let input = r#"1,2,3

        83 11 47 61 45
        30 74 73 14 66
        53 52 10 57 15
        64 50 54 28 87
        26 85 63 25 86"#;
        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        let boards = build_boards(lines.iter());

        assert_eq!(boards.len(), 1);
        assert_eq!(boards[0].rows.len(), 5);
    }
}

fn build_boards(mut input_iter: Iter<'_, String>) -> Vec<BingoBoard> {
    let mut boards: Vec<BingoBoard> = vec![];

    if let Some(_i) = input_iter.next() {
        let mut board = BingoBoard::build();

        board.add_row(input_iter.next().expect("Row not working"));
        board.add_row(input_iter.next().expect("Row not working"));
        board.add_row(input_iter.next().expect("Row not working"));
        board.add_row(input_iter.next().expect("Row not working"));
        board.add_row(input_iter.next().expect("Row not working"));

        boards.push(board);
    }
    boards
}

pub fn part_1(input: &Vec<String>) -> i32 {
    let mut input_iter = input.iter();
    let _moves: Vec<i32> = input_iter
        .next()
        .expect("No moves")
        .split(",")
        .map(|s| s.parse::<i32>().expect("Can't parse move value"))
        .collect();

    let mut _boards = build_boards(input_iter);
    0
}

pub fn part_2(_input: &Vec<String>) -> i32 {
    0
}
