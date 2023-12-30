use std::cmp;
use std::collections::HashSet;
use super::direction::Direction;

#[derive(Debug)]
struct LightBeam {
    row: usize,
    col: usize,
    direction: Direction,
}

impl LightBeam {
    fn try_new(
        col: isize,
        row: isize,
        direction: Direction,
        height: usize,
        width: usize,
    ) -> Option<LightBeam> {
        if col < 0 || col as usize >= width || row < 0 || row as usize >= height {
            return None;
        }

        Some(LightBeam {
            row: row as usize,
            col: col as usize,
            direction,
        })
    }
}

struct Contraption {
    layout: Vec<Vec<char>>,
    width: usize,
    height: usize,
    energized: HashSet<(usize, usize)>,
    visited: HashSet<(usize, usize, Direction)>,
}

impl Contraption {
    fn energize(&mut self, lb: LightBeam) -> (Option<LightBeam>, Option<LightBeam>) {
        if let Some(_) = self.visited.get(&(lb.col, lb.row, lb.direction)) {
            return (None, None);
        }
        self.energized.insert((lb.col, lb.row));
        self.visited.insert((lb.col, lb.row, lb.direction));

        match self.layout[lb.row][lb.col] {
            '.' => match lb.direction {
                Direction::Up => (
                    LightBeam::try_new(
                        lb.col as isize,
                        lb.row as isize - 1,
                        lb.direction,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
                Direction::Down => (
                    LightBeam::try_new(
                        lb.col as isize,
                        lb.row as isize + 1,
                        lb.direction,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
                Direction::Left => (
                    LightBeam::try_new(
                        lb.col as isize - 1,
                        lb.row as isize,
                        lb.direction,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
                Direction::Right => (
                    LightBeam::try_new(
                        lb.col as isize + 1,
                        lb.row as isize,
                        lb.direction,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
            },
            '-' => match lb.direction {
                Direction::Up => (
                    LightBeam::try_new(
                        lb.col as isize - 1,
                        lb.row as isize,
                        Direction::Left,
                        self.height,
                        self.width,
                    ),
                    LightBeam::try_new(
                        lb.col as isize + 1,
                        lb.row as isize,
                        Direction::Right,
                        self.height,
                        self.width,
                    ),
                ),
                Direction::Down => (
                    LightBeam::try_new(
                        lb.col as isize - 1,
                        lb.row as isize,
                        Direction::Left,
                        self.height,
                        self.width,
                    ),
                    LightBeam::try_new(
                        lb.col as isize + 1,
                        lb.row as isize,
                        Direction::Right,
                        self.height,
                        self.width,
                    ),
                ),
                Direction::Left => (
                    LightBeam::try_new(
                        lb.col as isize - 1,
                        lb.row as isize,
                        lb.direction,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
                Direction::Right => (
                    LightBeam::try_new(
                        lb.col as isize + 1,
                        lb.row as isize,
                        lb.direction,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
            },
            '|' => match lb.direction {
                Direction::Up => (
                    LightBeam::try_new(
                        lb.col as isize,
                        lb.row as isize - 1,
                        lb.direction,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
                Direction::Down => (
                    LightBeam::try_new(
                        lb.col as isize,
                        lb.row as isize + 1,
                        lb.direction,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
                Direction::Left => (
                    LightBeam::try_new(
                        lb.col as isize,
                        lb.row as isize - 1,
                        Direction::Up,
                        self.height,
                        self.width,
                    ),
                    LightBeam::try_new(
                        lb.col as isize,
                        lb.row as isize + 1,
                        Direction::Down,
                        self.height,
                        self.width,
                    ),
                ),
                Direction::Right => (
                    LightBeam::try_new(
                        lb.col as isize,
                        lb.row as isize - 1,
                        Direction::Up,
                        self.height,
                        self.width,
                    ),
                    LightBeam::try_new(
                        lb.col as isize,
                        lb.row as isize + 1,
                        Direction::Down,
                        self.height,
                        self.width,
                    ),
                ),
            },
            '/' => match lb.direction {
                Direction::Up => (
                    LightBeam::try_new(
                        lb.col as isize + 1,
                        lb.row as isize,
                        Direction::Right,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
                Direction::Down => (
                    LightBeam::try_new(
                        lb.col as isize - 1,
                        lb.row as isize,
                        Direction::Left,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
                Direction::Left => (
                    LightBeam::try_new(
                        lb.col as isize,
                        lb.row as isize + 1,
                        Direction::Down,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
                Direction::Right => (
                    LightBeam::try_new(
                        lb.col as isize,
                        lb.row as isize - 1,
                        Direction::Up,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
            },
            '\\' => match lb.direction {
                Direction::Up => (
                    LightBeam::try_new(
                        lb.col as isize - 1,
                        lb.row as isize,
                        Direction::Left,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
                Direction::Down => (
                    LightBeam::try_new(
                        lb.col as isize + 1,
                        lb.row as isize,
                        Direction::Right,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
                Direction::Left => (
                    LightBeam::try_new(
                        lb.col as isize,
                        lb.row as isize - 1,
                        Direction::Up,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
                Direction::Right => (
                    LightBeam::try_new(
                        lb.col as isize,
                        lb.row as isize + 1,
                        Direction::Down,
                        self.height,
                        self.width,
                    ),
                    None,
                ),
            },
            _ => {
                panic!("Invalid character");
            }
        }
    }

    fn score(&mut self, lb: LightBeam) -> usize {
        let mut stack = vec![lb];

        while let Some(lb) = stack.pop() {
            // println!("Start {:?}", lb);
            let result = self.energize(lb);
            // println!("\tEnd {:?},{:?}", result.0, result.1);
            if let Some(new_lb) = result.0 {
                stack.push(new_lb);
            }
            if let Some(new_lb) = result.1 {
                stack.push(new_lb);
            }
        }

        let score = self.energized.len();
        self.energized = HashSet::new();
        self.visited = HashSet::new();

        score
    }
}

impl From<&str> for Contraption {
    fn from(s: &str) -> Contraption {
        let mut width = 0;
        let mut height = 0;
        let mut layout = vec![];

        for line in s.lines() {
            let line = line.trim();
            width = line.len();
            height += 1;
            layout.push(line.chars().collect());
        }

        Contraption {
            layout,
            width,
            height,
            energized: HashSet::new(),
            visited: HashSet::new(),
        }
    }
}

pub fn part_1(input: &str) -> usize {
    let mut contraption = Contraption::from(input);

    contraption.score(LightBeam {
        col: 0,
        row: 0,
        direction: Direction::Right,
    })
}

pub fn part_2(input: &str) -> usize {
    let mut contraption = Contraption::from(input);
    let mut max_score = 0;

    for row in 0..contraption.height {
        let score = contraption.score(LightBeam {
            row,
            col: 0,
            direction: Direction::Right,
        });
        max_score = cmp::max(score, max_score);
        let score = contraption.score(LightBeam {
            row,
            col: contraption.width - 1,
            direction: Direction::Left,
        });
        max_score = cmp::max(score, max_score);
    }
    for col in 0..contraption.width {
        let score = contraption.score(LightBeam {
            row: 0,
            col,
            direction: Direction::Down,
        });
        max_score = cmp::max(score, max_score);
        let score = contraption.score(LightBeam {
            row: contraption.height - 1,
            col,
            direction: Direction::Up,
        });
        max_score = cmp::max(score, max_score);
    }

    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = r".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....";

        assert_eq!(part_1(input), 46);
    }

    #[test]
    fn part_2_test() {
        let input = r".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....";

        assert_eq!(part_2(input), 51);
    }
}
