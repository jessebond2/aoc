use core::fmt;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum BoulderShape {
    Round,
    Square,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Boulder {
    shape: BoulderShape,
    row: usize,
    col: usize,
}

impl Display for Boulder {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.shape {
            BoulderShape::Round => write!(f, "O"),
            BoulderShape::Square => write!(f, "#"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Dish {
    boulders: Vec<Boulder>,
    height: usize,
    width: usize,
}

impl Dish {
    fn score(&self) -> u32 {
        self.boulders
            .iter()
            .filter(|b| b.shape == BoulderShape::Round)
            .map(|b| (self.height - b.row) as u32)
            .sum::<u32>()
    }

    fn sort_by_column(&self) -> Vec<Vec<Boulder>> {
        let mut columns: Vec<Vec<Boulder>> = vec![];

        for _ in 0..self.width {
            columns.push(vec![]);
        }

        for boulder in &self.boulders {
            columns[boulder.col].push(*boulder);
        }

        for column in &mut columns {
            column.sort_by(|a, b| {
                if a.row < b.row {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
        }
        columns
    }

    fn sort_by_row(&self) -> Vec<Vec<Boulder>> {
        let mut rows: Vec<Vec<Boulder>> = vec![];

        for _ in 0..self.width {
            rows.push(vec![]);
        }

        for boulder in &self.boulders {
            rows[boulder.row].push(*boulder);
        }

        for row in &mut rows {
            row.sort_by(|a, b| {
                if a.row < b.row {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
        }
        rows
    }

    fn tilt_up(&self) -> Dish {
        let mut columns = self.sort_by_column();

        for column in &mut columns {
            let mut last: Option<&mut Boulder> = None;
            for boulder in column {
                if let Some(last) = last {
                    if boulder.shape == BoulderShape::Round {
                        boulder.row = last.row + 1;
                    }
                } else if boulder.shape == BoulderShape::Round {
                    boulder.row = 0
                }

                last = Some(boulder)
            }
        }
        let boulders: Vec<Boulder> = columns.iter().flatten().copied().collect();

        Dish {
            boulders,
            height: self.height,
            width: self.width,
        }
    }

    fn tilt_down(&self) -> Dish {
        let mut columns = self.sort_by_column();

        for column in &mut columns {
            let mut last: Option<&mut Boulder> = None;
            for boulder in column.iter_mut().rev() {
                if let Some(last) = last {
                    if boulder.shape == BoulderShape::Round {
                        boulder.row = last.row - 1;
                    }
                } else if boulder.shape == BoulderShape::Round {
                    boulder.row = self.height
                }

                last = Some(boulder)
            }
        }
        let boulders: Vec<Boulder> = columns.iter().flatten().copied().collect();

        Dish {
            boulders,
            height: self.height,
            width: self.width,
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseDishError;

impl FromStr for Dish {
    type Err = ParseDishError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut boulders: Vec<Boulder> = vec![];
        let mut height = 0;
        let mut width = 0;

        for row in s.lines().enumerate() {
            height += 1;
            width = row.1.trim().len();

            for col in row.1.trim().chars().enumerate() {
                let boulder = match col.1 {
                    'O' => Some(Boulder {
                        shape: BoulderShape::Round,
                        row: row.0,
                        col: col.0,
                    }),
                    '#' => Some(Boulder {
                        shape: BoulderShape::Square,
                        row: row.0,
                        col: col.0,
                    }),
                    _ => None,
                };

                if let Some(boulder) = boulder {
                    boulders.push(boulder);
                }
            }
        }

        Ok(Dish {
            boulders,
            height,
            width,
        })
    }
}

pub fn part_1(input: &str) -> u32 {
    let mut dishes = vec![];
    for section in input.split("\n\n") {
        dishes.push(Dish::from_str(section).unwrap());
    }

    dishes.iter().map(|d| d.tilt_up().score()).sum()
}

pub fn part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(
            part_1(
                "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#...."
            ),
            136
        );
    }

    #[test]
    fn get_score_test() {
        let dish = Dish::from_str(
            "OOOO.#.O..
        OO..#....#
        OO..O##..O
        O..#.OO...
        ........#.
        ..#....#.#
        ..O..#.O.O
        ..O.......
        #....###..
        #....#....",
        )
        .unwrap();

        assert_eq!(dish.score(), 136);
    }

    #[test]
    fn tilt_up_test() {
        let dish1 = Dish::from_str(
            "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....",
        )
        .unwrap()
        .tilt_up();
        let dish2 = Dish::from_str(
            "OOOO.#.O..
        OO..#....#
        OO..O##..O
        O..#.OO...
        ........#.
        ..#....#.#
        ..O..#.O.O
        ..O.......
        #....###..
        #....#....",
        )
        .unwrap();

        assert_eq!(dish1.score(), dish2.score());
    }

    #[test]
    fn part_2_test() {
        let input = "";

        assert_eq!(part_2(input), 71503);
    }
}
