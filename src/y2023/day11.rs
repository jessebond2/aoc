use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Star {
    x: usize,
    y: usize,
}

impl Star {
    fn expand(&self, x_factors: &[u32], y_factors: &[u32], factor: u32) -> Star {
        Star {
            x: self.x + ((factor - 1) * x_factors[self.x]) as usize,
            y: self.y + ((factor - 1) * y_factors[self.y]) as usize,
        }
    }

    fn distance(&self, star: &Star) -> usize {
        let mut distance: usize = if self.x > star.x {
            self.x - star.x
        } else {
            star.x - self.x
        };
        distance += if self.y > star.y {
            self.y - star.y
        } else {
            star.y - self.y
        };

        distance
    }
}

pub fn part_1(input: &str) -> usize {
    parse_and_expand(input, 2)
}

pub fn parse_and_expand(input: &str, expansion_factor: u32) -> usize {
    let mut stars: Vec<Star> = vec![];
    let mut x_stars: HashSet<usize> = HashSet::new();
    let mut y_stars: HashSet<usize> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.trim().chars().enumerate() {
            if char == '#' {
                stars.push(Star { x, y });
                x_stars.insert(x);
                y_stars.insert(y);
            }
        }
    }

    let mut x_expansion: Vec<u32> = vec![];
    let mut y_expansion: Vec<u32> = vec![];
    let mut factor = 0;
    for (y, _) in input.lines().enumerate() {
        if !y_stars.contains(&y) {
            factor += 1
        }
        y_expansion.push(factor);
    }
    // println!("y factors {:?}", y_expansion);

    factor = 0;
    for (x, _) in input.lines().next().unwrap().chars().enumerate() {
        if !x_stars.contains(&x) {
            factor += 1
        }
        x_expansion.push(factor);
    }
    // println!("x factors {:?}", x_expansion);

    let expanded_stars: Vec<_> = stars
        .iter()
        .map(|star| star.expand(&x_expansion, &y_expansion, expansion_factor))
        .collect();

    let mut count = 0;
    for n in 0..expanded_stars.len() - 1 {
        let star = expanded_stars[n];
        for star2 in expanded_stars.iter().skip(n + 1) {
            count += star.distance(star2);
        }
    }

    count
}

pub fn part_2(input: &str) -> usize {
    parse_and_expand(input, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

        assert_eq!(part_1(input), 374);
    }

    #[test]
    fn star_distance() {
        assert_eq!(Star { x: 0, y: 0 }.distance(&Star { x: 10, y: 20 }), 30);
        assert_eq!(Star { x: 5, y: 5 }.distance(&Star { x: 0, y: 0 }), 10);
        assert_eq!(Star { x: 103, y: 0 }.distance(&Star { x: 207, y: 1 }), 105);
        assert_eq!(
            Star { x: 103, y: 0 }.distance(&Star { x: 206, y: 104 }),
            207
        );
        assert_eq!(Star { x: 207, y: 1 }.distance(&Star { x: 0, y: 209 }), 415);
    }

    #[test]
    fn star_expansion() {
        let x_expansion: Vec<u32> = vec![0, 1, 2, 3, 4];
        let y_expansion: Vec<u32> = vec![0, 1, 2, 3, 4];

        assert_eq!(
            Star { x: 0, y: 0 }.expand(&x_expansion, &y_expansion, 1),
            Star { x: 0, y: 0 }
        );
        assert_eq!(
            Star { x: 1, y: 1 }.expand(&x_expansion, &y_expansion, 1),
            Star { x: 2, y: 2 }
        );
        assert_eq!(
            Star { x: 4, y: 4 }.expand(&x_expansion, &y_expansion, 1),
            Star { x: 8, y: 8 }
        );
        assert_eq!(
            Star { x: 4, y: 4 }.expand(&x_expansion, &y_expansion, 2),
            Star { x: 12, y: 12 }
        );
        assert_eq!(
            Star { x: 4, y: 4 }.expand(&x_expansion, &y_expansion, 10),
            Star { x: 44, y: 44 }
        );
    }

    #[test]
    fn part_2_test() {
        let input = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

        assert_eq!(parse_and_expand(input, 10), 1030);
        assert_eq!(parse_and_expand(input, 100), 8410);
    }
}
