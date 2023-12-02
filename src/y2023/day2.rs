use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Game {
    id: i32,
    blue: i32,
    red: i32,
    green: i32,
}

impl Game {
    pub fn build(line: &str) -> Game {
        let mut pieces = line.split(":");
        let game_split = pieces.next().expect("No game found");
        let rounds_split = pieces.next().expect("No rounds found");

        static ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (?P<id>\d+)").unwrap());
        let caps = ID_REGEX.captures(game_split).unwrap();
        let id = &caps["id"]
            .parse::<i32>()
            .expect("Game id should be numeric");

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        static ROUNDS_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?:(?P<count>\d+) (?P<color>blue|green|red),?)").unwrap());
        // let caps = rounds_regex.captures(rounds_split).unwrap();
        for c in ROUNDS_REGEX.captures_iter(rounds_split) {
            let count = &c["count"].parse::<i32>().expect("Missing count");
            match &c["color"] {
                "red" => red = cmp::max(red, *count),
                "blue" => blue = cmp::max(blue, *count),
                _ => green = cmp::max(green, *count),
            }
        }

        Game {
            id: *id,
            blue,
            green,
            red,
        }
    }

    pub fn is_valid(self, red: i32, green: i32, blue: i32) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}

pub fn part_1(input: &Vec<String>) -> i32 {
    let red = 12;
    let green = 13;
    let blue = 14;

    input
        .iter()
        .map(|line| Game::build(line))
        .filter(|game| game.is_valid(red, green, blue))
        .map(|game| game.id)
        .sum()
}

pub fn part_2(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(|line| Game::build(line))
        .map(|game| game.red * game.blue * game.green)
        .sum()
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
        assert_eq!(part_1(&lines), 8)
    }

    #[test]
    fn parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green:";

        assert_eq!(
            Game::build(input),
            Game {
                id: 1,
                blue: 6,
                red: 4,
                green: 2
            }
        )
    }

    #[test]
    fn part_2_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_2(&lines), 2286)
    }
}
