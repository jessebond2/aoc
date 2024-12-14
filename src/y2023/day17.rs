use super::direction::DirectionHeading;
use std::collections::HashMap;
use std::cmp;

type CityMemo = HashMap<Point, (DirectionHeading, u32)>;
type Point = (isize, isize);

struct City {
    blocks: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl City {
    fn minimize_heatloss(&self) -> u32 {
        let mut memo: CityMemo = HashMap::new();
        let start: Point = (0, 0);
        let _end: Point = (self.width as isize - 1, self.height as isize - 1);

        memo.insert(start, (DirectionHeading::Down(0), 0));
        self.minimize_heatloss_helper((0, 1), DirectionHeading::Down(1), &mut memo);
        self.minimize_heatloss_helper((1, 0), DirectionHeading::Right(1), &mut memo);
        // self.minimize_heatloss_helper(end, DirectionHeading::Left(2), &mut memo);

        cmp::min(memo.get(&(0, 1)).unwrap().1, memo.get(&(1, 0)).unwrap().1)
    }

    fn update_if_lower(
        &self,
        position: Point,
        heading: DirectionHeading,
        value: Option<(DirectionHeading, u32)>,
        memo: &mut CityMemo,
    ) {
        if let Some(value) = value {
            println!(
                "Current {:?}, heading: {:?}, value: {:?}",
                position, heading, value
            );
            let current = self.blocks[position.1 as usize][position.0 as usize];

            if let Some(prior_value) = memo.get(&position) {
                if prior_value.1 > value.1 + current {
                    println!(
                        "\tOverwriting {:?} with {:?}",
                        position,
                        (heading, value.1 + current)
                    );
                    memo.insert(position, (heading, value.1 + current));
                }
            } else {
                println!(
                    "\tInserting {:?} with {:?}",
                    position,
                    (heading, value.1 + current)
                );
                memo.insert(position, (heading, value.1 + current));
            }
        }
    }

    fn minimize_heatloss_helper(
        &self,
        position: Point,
        heading: DirectionHeading,
        memo: &mut CityMemo,
    ) -> Option<(DirectionHeading, u32)> {
        if let Some(value) = memo.get(&position) {
            return Some(*value);
        }
        if position.0 == self.width as isize - 1 && position.1 == self.height as isize - 1 {
            let current = self.blocks[position.1 as usize][position.0 as usize];
            println!("Base case {:?}, {:?}", position, current);
            memo.insert(position, (heading, current));
            return Some(*memo.get(&position).unwrap());
        }
        if position.0 < 0
            || position.0 > self.width as isize - 1
            || position.1 < 0
            || position.1 > self.height as isize - 1
        {
            return None;
        }

        println!("Traversing {:?}, heading {:?}", position, heading);
        match heading {
            DirectionHeading::Up(distance) => {
                if distance < 3 {
                    let value = self.minimize_heatloss_helper(
                        (position.0, position.1 - 1),
                        DirectionHeading::Up(distance + 1),
                        memo,
                    );
                    self.update_if_lower(position, heading, value, memo);
                }
                let value = self.minimize_heatloss_helper(
                    (position.0 - 1, position.1),
                    DirectionHeading::Left(1),
                    memo,
                );
                self.update_if_lower(position, heading, value, memo);

                let value = self.minimize_heatloss_helper(
                    (position.0 + 1, position.1),
                    DirectionHeading::Right(1),
                    memo,
                );
                self.update_if_lower(position, heading, value, memo);
            }
            DirectionHeading::Down(distance) => {
                if distance < 3 {
                    let value = self.minimize_heatloss_helper(
                        (position.0, position.1 + 1),
                        DirectionHeading::Down(distance + 1),
                        memo,
                    );
                    self.update_if_lower(position, heading, value, memo);
                }
                let value = self.minimize_heatloss_helper(
                    (position.0 - 1, position.1),
                    DirectionHeading::Left(1),
                    memo,
                );
                self.update_if_lower(position, heading, value, memo);

                let value = self.minimize_heatloss_helper(
                    (position.0 + 1, position.1),
                    DirectionHeading::Right(1),
                    memo,
                );
                self.update_if_lower(position, heading, value, memo);
            }
            DirectionHeading::Left(distance) => {
                if distance < 3 {
                    let value = self.minimize_heatloss_helper(
                        (position.0 - 1, position.1),
                        DirectionHeading::Left(distance + 1),
                        memo,
                    );
                    self.update_if_lower(position, heading, value, memo);
                }
                let value = self.minimize_heatloss_helper(
                    (position.0, position.1 - 1),
                    DirectionHeading::Up(1),
                    memo,
                );
                self.update_if_lower(position, heading, value, memo);

                let value = self.minimize_heatloss_helper(
                    (position.0, position.1 + 1),
                    DirectionHeading::Down(1),
                    memo,
                );
                self.update_if_lower(position, heading, value, memo);
            }
            DirectionHeading::Right(distance) => {
                if distance < 3 {
                    let value = self.minimize_heatloss_helper(
                        (position.0 + 1, position.1),
                        DirectionHeading::Right(distance + 1),
                        memo,
                    );
                    self.update_if_lower(position, heading, value, memo);
                }
                let value = self.minimize_heatloss_helper(
                    (position.0, position.1 - 1),
                    DirectionHeading::Up(1),
                    memo,
                );
                self.update_if_lower(position, heading, value, memo);

                let value = self.minimize_heatloss_helper(
                    (position.0, position.1 + 1),
                    DirectionHeading::Down(1),
                    memo,
                );
                self.update_if_lower(position, heading, value, memo);
            }
        }

        Some(*memo.get(&position).unwrap())
    }
}

impl From<&str> for City {
    fn from(s: &str) -> City {
        let mut width = 0;
        let mut height = 0;
        let mut blocks = vec![];

        for line in s.lines() {
            let line = line.trim();
            width = line.len();
            height += 1;
            blocks.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
        }

        City {
            blocks,
            width,
            height,
        }
    }
}

pub fn part_1(input: &str) -> u32 {
    let city = City::from(input);
    city.minimize_heatloss()
}

pub fn part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_small_test() {
        let input = "111
        155
        111";

        assert_eq!(part_1(input), 4);
    }

    #[test]
    fn part_1_small2_test() {
        let input = "1111
        1551
        9911";

        assert_eq!(part_1(input), 5);
    }

    #[test]
    fn part_1_small3_test() {
        let input = "11111
        15551
        99911";

        assert_eq!(part_1(input), 10);
    }

    #[test]
    fn part_1_test() {
        let input = "2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533";

        assert_eq!(part_1(input), 102);
    }

    #[test]
    fn part_2_test() {
        let input = "";

        assert_eq!(part_2(input), 71503);
    }
}
