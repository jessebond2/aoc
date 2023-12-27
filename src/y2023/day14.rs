use core::fmt;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum BoulderShape {
    Round,
    Square,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

        for _ in 0..self.height {
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
        let columns = self.sort_by_column();

        // println!("Tilt up");
        self.tilt_high(columns, true)
    }

    fn tilt_down(&self) -> Dish {
        let columns = self.sort_by_column();

        // println!("Tilt down");
        self.tilt_low(columns, true)
    }

    fn tilt_left(&self) -> Dish {
        let rows = self.sort_by_row();

        // println!("Tilt left");
        self.tilt_high(rows, false)
    }

    fn tilt_right(&self) -> Dish {
        let rows = self.sort_by_row();

        // println!("Tilt right");
        self.tilt_low(rows, false)
    }

    fn spin(&self) -> Dish {
        self.tilt_up().tilt_left().tilt_down().tilt_right()
    }

    fn tilt_high(&self, mut vecs: Vec<Vec<Boulder>>, is_vertical: bool) -> Dish {
        for vec in &mut vecs {
            let mut last: Option<&mut Boulder> = None;
            for boulder in vec {
                if let Some(last) = last {
                    if boulder.shape == BoulderShape::Round {
                        if is_vertical {
                            boulder.row = last.row + 1;
                        } else {
                            boulder.col = last.col + 1;
                        }
                    }
                } else if boulder.shape == BoulderShape::Round {
                    if is_vertical {
                        boulder.row = 0;
                    } else {
                        boulder.col = 0;
                    }
                }

                last = Some(boulder)
            }
        }
        let boulders: Vec<Boulder> = vecs.iter().flatten().copied().collect();

        Dish {
            boulders,
            height: self.height,
            width: self.width,
        }
    }

    fn tilt_low(&self, mut vecs: Vec<Vec<Boulder>>, is_vertical: bool) -> Dish {
        for vec in &mut vecs {
            let mut last: Option<&mut Boulder> = None;
            for boulder in vec.iter_mut().rev() {
                if let Some(last) = last {
                    if boulder.shape == BoulderShape::Round {
                        if is_vertical {
                            boulder.row = last.row - 1;
                        } else {
                            boulder.col = last.col - 1;
                        }
                    }
                } else if boulder.shape == BoulderShape::Round {
                    if is_vertical {
                        boulder.row = self.height - 1;
                    } else {
                        boulder.col = self.width - 1;
                    }
                }

                last = Some(boulder)
            }
        }
        let boulders: Vec<Boulder> = vecs.iter().flatten().copied().collect();

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

pub fn part_2(input: &str) -> u32 {
    let mut dish = Dish::from_str(input).unwrap();
    let mut memo: HashMap<Dish, i32> = HashMap::new();

    let mut cycle = 0;
    let mut cycle_size = None;

    loop {
        dish = dish.spin();
        cycle += 1;
        // println!("iter {} score {}", cycle, dish.score());

        if let Some(result) = memo.get(&dish) {
            // println!("Cycle detected, {}", result);
            cycle_size = Some(cycle - result);
            break;
        } else {
            memo.insert(dish.clone(), cycle);
        }
    }

    let target_cycle = 1_000_000_000;
    let target_cycle_remainder = target_cycle % cycle_size.unwrap();

    loop {
        dish = dish.spin();
        cycle += 1;

        // println!(
        //     "Cycle size is {}, cycle position {}, target_cycle_remainder {}",
        //     cycle_size,
        //     cycle % cycle_size,
        //     target_cycle_remainder
        // );

        if cycle % cycle_size.unwrap() == target_cycle_remainder {
            break;
        }
    }

    dish.score()
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
    fn spin_cycle() {
        let mut dish = Dish::from_str(
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
        .unwrap();
        dish = dish.spin();

        assert_eq!(
            dish.score(),
            Dish::from_str(
                ".....#....
        ....#...O#
        ...OO##...
        .OO#......
        .....OOO#.
        .O#...O#.#
        ....O#....
        ......OOOO
        #...O###..
        #..OO#...."
            )
            .unwrap()
            .score()
        )
    }

    #[test]
    fn part_2_test() {
        let input = "";

        assert_eq!(part_2(input), 71503);
    }
}
