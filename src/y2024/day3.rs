use regex::Regex;

#[derive(PartialEq, Clone, Copy)]
enum Op {
    Do,
    Dont,
    Mul,
}

#[derive(Debug)]
struct State {
    enabled: bool,
    count: i32,
}

pub fn part_2(input: &str) -> i32 {
    let regex = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)|(do)\(\)()()|(don't)\(\)()()").unwrap();
    let all: Vec<(Op, i32, i32)> = regex
        .captures_iter(input)
        .map(|caps| {
            let (_, [op, a, b]) = caps.extract();
            match op {
                "mul" => (
                    Op::Mul,
                    a.parse::<i32>().unwrap(),
                    b.parse::<i32>().unwrap(),
                ),
                "do" => (Op::Do, 0, 0),
                _ => (Op::Dont, 0, 0),
            }
        })
        .collect();

    all.into_iter()
        .fold(
            State {
                enabled: true,
                count: 0,
            },
            |acc, (op, a, b)| match op {
                Op::Do => State {
                    enabled: true,
                    count: acc.count,
                },
                Op::Dont => State {
                    enabled: false,
                    count: acc.count,
                },
                Op::Mul => {
                    if acc.enabled {
                        State {
                            enabled: true,
                            count: acc.count + a * b,
                        }
                    } else {
                        acc
                    }
                }
            },
        )
        .count
}

pub fn part_1(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let all: Vec<(i32, i32)> = regex
        .captures_iter(input)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        })
        .collect();
    all.into_iter().fold(0, |acc, (a, b)| acc + a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(part_1(input), 161);
    }

    #[test]
    fn part_2_test() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(part_2(input), 48);
    }
}
