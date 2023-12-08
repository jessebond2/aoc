use std::collections::HashMap;
use std::slice::Iter;

struct Node {
    left: String,
    right: String,
}

fn build_map(iter: Iter<'_, String>) -> HashMap<String, Node> {
    let mut map: HashMap<String, Node> = HashMap::new();
    iter.for_each(|line| {
        let name = &line[..3];
        let left = &line[7..10];
        let right = &line[12..15];

        // println!("name {}, left {}, right {}", name, left, right);
        map.insert(
            name.to_string(),
            Node {
                left: left.to_string(),
                right: right.to_string(),
            },
        );
    });

    map
}

pub fn part_1(input: &Vec<String>) -> u32 {
    let mut iter = input.iter();
    let moves = (iter.next().unwrap()).clone();

    iter.next();

    let map = build_map(iter);
    let mut current = "AAA";
    let mut step: u32 = 0;

    'outer: loop {
        for movement in moves.chars() {
            if current == "ZZZ" {
                break 'outer;
            }

            let node = map.get(current).unwrap();
            current = match movement {
                'L' => &node.left,
                _ => &node.right,
            };
            step += 1;
        }
    }

    step
}

pub fn part_2(input: &Vec<String>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_1(&lines), 2);
    }

    #[test]
    fn part_1_test_2() {
        let input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_1(&lines), 6);
    }

    #[test]
    fn part_2_test() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_2(&lines), 71503);
    }
}
