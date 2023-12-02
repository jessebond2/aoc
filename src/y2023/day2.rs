use once_cell::sync::Lazy;
use regex::Regex;
// Game (\d+):(?:(?: (\d+) (blue|green|red),?)+;?)+
struct Game {
    id: usize,
    blue: i32,
    red: i32,
    green: i32,
}

impl Game {
    pub fn build(line: &str) {
        let mut pieces = line.split(":");
        let game_split = pieces.next().expect("No game found");
        let rounds_split = pieces.next().expect("No rounds found");

        static re: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (?P<id>\d+)").unwrap());
        let caps = re.captures(game_split).unwrap();
        println!("Game id {}", &caps["id"]);
    }
}

pub fn part_1(input: &Vec<String>) -> i32 {
    let _ = input.iter().map(|line| Game::build(line));
    0
}

pub fn part_2(input: &Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_1(&lines), 142)
    }

    #[test]
    fn part_2_test() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_2(&lines), 281)
    }
}
