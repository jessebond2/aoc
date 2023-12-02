fn parse_line(line: &String) -> u32 {
    let mut digits = line.chars().filter_map(|s| s.to_digit(10));
    let first = digits.next().expect("Missing first digit");
    let last = digits.last().unwrap_or(first);

    first * 10 + last
}

fn parse_line2(line: &String) -> u32 {
    let mut digit_vec: Vec<u32> = vec![];
    let chars: Vec<_> = line.chars().collect();

    let mut i = 0;
    while i < line.len() {
        let char = chars[i];
        match char {
            '0'..='9' => {
                digit_vec.push(char.to_digit(10).unwrap());
                i += 1;
                continue;
            }
            _ => {}
        }
        if i + 3 <= line.len() {
            let slice = &line[i..i + 3];

            match slice {
                "one" => {
                    digit_vec.push(1);
                }
                "two" => {
                    digit_vec.push(2);
                }
                "six" => {
                    digit_vec.push(6);
                }
                _ => {}
            }
        }
        if i + 4 <= line.len() {
            let slice = &line[i..i + 4];

            match slice {
                "four" => {
                    digit_vec.push(4);
                }
                "five" => {
                    digit_vec.push(5);
                }
                "nine" => {
                    digit_vec.push(9);
                }
                _ => {}
            }
        }
        if i + 5 <= line.len() {
            let slice = &line[i..i + 5];

            match slice {
                "three" => {
                    digit_vec.push(3);
                }
                "seven" => {
                    digit_vec.push(7);
                }
                "eight" => {
                    digit_vec.push(8);
                }
                _ => {}
            }
        }

        i += 1
    }

    let first = digit_vec[0];
    let last = digit_vec[digit_vec.len() - 1];

    let sum = first * 10 + last;
    println!("Line {} becomes {:?} = sum {}", line, digit_vec, sum);

    sum
}

pub fn part_1(input: &Vec<String>) -> i32 {
    let sum: u32 = input.iter().map(|line| parse_line(line)).sum();

    sum as i32
}

pub fn part_2(input: &Vec<String>) -> i32 {
    let sum: u32 = input.iter().map(|line| parse_line2(line)).sum();

    sum as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

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

    #[test]
    fn parse_line2_test() {
        let input = String::from("two1nine");
        assert_eq!(parse_line2(&input), 29)
    }
}
