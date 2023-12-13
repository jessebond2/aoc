use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, PartialEq, Clone)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    X,
    S,
}

#[derive(Debug, Clone)]
struct PipeNode {
    x: i32,
    y: i32,
    pipe: Pipe,
}

impl PipeNode {
    fn new(x: i32, y: i32, c: char) -> PipeNode {
        let pipe = match c {
            '|' => Pipe::NS,
            '-' => Pipe::EW,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            '.' => Pipe::X,
            _ => Pipe::S,
        };

        PipeNode { x, y, pipe }
    }
}

#[derive(Debug, Clone)]
struct PipeGraph {
    start: (i32, i32),
    nodes: HashMap<(i32, i32), PipeNode>,
    empty_count: i32,
}

impl PipeGraph {
    fn next_to_start(&self) -> (i32, i32) {
        // left
        if let Some(node) = self.nodes.get(&(self.start.0 - 1, self.start.1)) {
            if node.pipe == Pipe::EW || node.pipe == Pipe::NE || node.pipe == Pipe::SE {
                return (self.start.0 - 1, self.start.1);
            }
        }
        if let Some(node) = self.nodes.get(&(self.start.0 + 1, self.start.1)) {
            if node.pipe == Pipe::EW || node.pipe == Pipe::NW || node.pipe == Pipe::SW {
                return (self.start.0 + 1, self.start.1);
            }
        }
        if let Some(node) = self.nodes.get(&(self.start.0, self.start.1 - 1)) {
            if node.pipe == Pipe::NS || node.pipe == Pipe::NE || node.pipe == Pipe::NW {
                return (self.start.0, self.start.1 - 1);
            }
        }

        (self.start.0, self.start.1 + 1)
    }

    fn next_node(&self, node: &PipeNode, visited: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
        let nodes = match node.pipe {
            Pipe::NS => ((node.x, node.y - 1), (node.x, node.y + 1)),
            Pipe::EW => ((node.x - 1, node.y), (node.x + 1, node.y)),
            Pipe::NE => ((node.x, node.y - 1), (node.x + 1, node.y)),
            Pipe::NW => ((node.x, node.y - 1), (node.x - 1, node.y)),
            Pipe::SW => ((node.x, node.y + 1), (node.x - 1, node.y)),
            Pipe::SE => ((node.x, node.y + 1), (node.x + 1, node.y)),
            Pipe::X => return None,
            Pipe::S => return Some(self.next_to_start()),
        };

        if visited.contains(&nodes.0) {
            if visited.contains(&nodes.1) {
                return Some(self.start);
            }
            Some(nodes.1)
        } else {
            Some(nodes.0)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePipeGraphError;

impl FromStr for PipeGraph {
    type Err = ParsePipeGraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: HashMap<(i32, i32), PipeNode> = HashMap::new();
        let mut start: (i32, i32) = (0, 0);
        let mut empty_count = 0;

        for (y, line) in s.lines().enumerate() {
            for (x, char) in line.trim().chars().enumerate() {
                let node = PipeNode::new(x as i32, y as i32, char);
                if node.pipe == Pipe::S {
                    start = (x as i32, y as i32);
                }
                if node.pipe == Pipe::X {
                    empty_count += 1;
                }
                nodes.insert((node.x, node.y), node);
            }
        }

        Ok(PipeGraph {
            start,
            nodes,
            empty_count,
        })
    }
}

pub fn part_1(input: &str) -> i32 {
    let graph = PipeGraph::from_str(input).expect("Error parsing Graph");
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut current = graph
        .nodes
        .get(&graph.start)
        .expect("Missing starting node")
        .clone();

    // println!("Graph {:?}", graph);

    loop {
        visited.insert((current.x, current.y));
        let next = graph
            .next_node(&current, &visited)
            .expect("Should have a next node");

        // println!("Visited {:?}, next {:?}", visited, next);
        current = graph.nodes.get(&next).expect("Missing next node").clone();
        if current.pipe == Pipe::S {
            break;
        }
    }

    println!(
        "Visited {}, empty_count {}, total nodes {}, visited + empty_count {}",
        visited.len(),
        graph.empty_count,
        graph.nodes.len(),
        visited.len() as i32 + graph.empty_count,
    );

    visited.len() as i32 / 2
}

pub fn part_2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = ".....
        .S-7.
        .|.|.
        .L-J.
        .....";

        assert_eq!(part_1(input), 4);
    }

    #[test]
    fn part_1_test_2() {
        let input = "..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...";

        assert_eq!(part_1(input), 8);
    }
}
