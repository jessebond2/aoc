use std::collections::HashMap;
use std::slice::Iter;
use std::time::Instant;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
struct Node<'n, 'l, 'r> {
    left: &'l str,
    right: &'r str,
    name: &'n str,
    ends_in_a: bool,
    ends_in_z: bool,
    visited: bool,
    visited_step: Option<u32>,
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
                left,
                right,
                name,
                ends_in_a: ends_in(name, 'A'),
                ends_in_z: ends_in(name, 'Z'),
                visited: false,
                visited_step: None,
            },
        );
    });

    map
}

fn find_ending_with_a<'a>(map: &'a HashMap<String, Node<'a, 'a, 'a>>) -> Vec<&'a Node<'a, 'a, 'a>> {
    map.iter()
        .filter(|(_, node)| node.ends_in_a)
        .map(|(_, node)| node)
        .collect()
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

fn ends_in(str: &str, ch: char) -> bool {
    str.chars().nth(2) == Some(ch)
}

fn all_ends_in_z(vec: &Vec<&Node>) -> bool {
    vec.iter().fold(true, |acc, val| acc && val.ends_in_z)
}

fn loop_size(node: &Node, moves: &str, map: &HashMap<String, Node>) -> (u32, u32) {
    let mut map = map.clone();
    let mut current = map.get(node.name).unwrap().clone();
    let mut step = 0;

    'outer: loop {
        for movement in moves.chars() {
            if current.visited {
                break 'outer;
            }

            current.visited = true;
            current.visited_step = Some(step);
            map.insert(current.name.to_string(), current.clone());

            current = match movement {
                'L' => map.get(&current.left.to_string()).unwrap().clone(),
                _ => map.get(&current.right.to_string()).unwrap().clone(),
            };
            step += 1;
        }
    }

    (
        current.visited_step.unwrap(),
        step - current.visited_step.unwrap(),
    )
}

pub fn part_2(input: &Vec<String>) -> u32 {
    let mut iter = input.iter();
    let moves = (iter.next().unwrap()).clone();

    iter.next();

    let map = build_map(iter);
    let mut current = find_ending_with_a(&map);

    let loop_sizes: Vec<_> = current
        .iter()
        .map(|node| loop_size(node, &moves, &map))
        .collect();
    println!("starting positions: {:?}", current);
    println!("loop_sizes positions: {:?}", loop_sizes);
    return 0;
    let mut step: u32 = 0;
    let now = Instant::now();

    'outer: loop {
        for movement in moves.chars() {
            if all_ends_in_z(&current) {
                break 'outer;
            }

            for (idx, node) in current.clone().into_iter().enumerate() {
                current[idx] = match movement {
                    'L' => &map.get(&node.left.to_string()).unwrap(),
                    _ => &map.get(&node.right.to_string()).unwrap(),
                };
            }
            step += 1;
            if step % 100000 == 0 {
                println!("step {}, elapsed time {:?}", step, now.elapsed());
            }
        }
    }

    step
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
    fn find_ending_with_a_test() {
        let input: Vec<_> = "AAA = (BBB, CCC)
        BBA = (DDD, EEE)
        CCA = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)"
            .lines()
            .map(|line| line.trim().to_string())
            .collect();
        let iter = input.iter();

        let map = build_map(iter);

        assert_eq!(
            find_ending_with_a(&map).sort(),
            vec!["AAA", "BBA", "CCA"].sort()
        );
    }

    #[test]
    fn loop_size_test() {
        let input = "11A = (11B, XXX)
        12A = (12B, XXX)
        12B = (XXX, 11A)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        let iter = lines.iter();
        let map = build_map(iter);
        let node1 = map.get("11A").unwrap();
        let node2 = map.get("22A").unwrap();
        let node3 = map.get("12A").unwrap();
        let moves = "LR";

        assert_eq!(loop_size(node3, moves, &map), (3 as u32, 2 as u32));
        assert_eq!(loop_size(node1, moves, &map), (1 as u32, 2 as u32));
        assert_eq!(loop_size(node2, moves, &map), (1 as u32, 3 as u32));
    }

    #[test]
    fn part_2_test() {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_2(&lines), 6);
    }
}
