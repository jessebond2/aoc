#[derive(Debug, PartialEq, Clone)]
struct PartNumber {
    x1: i32,
    x2: i32,
    y: usize,
    value: u32,
    valid: bool,
}

impl PartNumber {
    fn validate(&mut self) {
        self.valid = true;
    }
}

#[derive(Debug, PartialEq)]
struct Symbol {
    x: i32,
    y: usize,
}

fn parse_numbers(line: &String, y: usize) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = vec![];
    let mut value: u32 = 0;
    let mut start_index: i32 = 0;
    let mut is_building: bool = false;

    for (idx, char) in line.chars().enumerate() {
        match char {
            '0'..='9' => {
                if !is_building {
                    start_index = idx as i32;
                }
                is_building = true;
                value = value * 10 + char.to_digit(10).expect("This should've been a digit");
            }
            _ => {
                if is_building {
                    is_building = false;
                    part_numbers.push(PartNumber {
                        x1: start_index,
                        x2: (idx as i32) - 1,
                        y,
                        value,
                        valid: false,
                    });
                    value = 0;
                }
            }
        }
    }
    if is_building {
        is_building = false;
        part_numbers.push(PartNumber {
            x1: start_index,
            x2: (line.len() as i32) - 1,
            y,
            value,
            valid: false,
        });
        value = 0;
    }

    part_numbers
}

fn parse_symbols(line: &String, y: usize) -> Vec<Symbol> {
    line.chars()
        .enumerate()
        .filter_map(|(idx, char)| match char {
            '0'..='9' => None,
            '.' => None,
            _ => Some(Symbol { x: idx as i32, y }),
        })
        .collect()
}

fn validate_row(row: &Vec<PartNumber>, symbol: &Symbol, y: usize) -> Vec<PartNumber> {
    let valid: Vec<PartNumber> = row
        .iter()
        .map(|part_number| {
            if validate_part_number(part_number, symbol) {
                let mut pn = part_number.clone();
                pn.validate();
                // println!("updated pn {:?}", pn);
                return pn;
            }
            part_number.clone()
        })
        .collect();

    // println!("Valid part numbers {:?} for line {}", valid, y);

    valid
}

fn validate_part_number(part_number: &PartNumber, symbol: &Symbol) -> bool {
    let is_valid = part_number.x1 - 1 <= symbol.x && symbol.x <= part_number.x2 + 1;
    // println!("is_valid {} {:?} {:?}", is_valid, part_number, symbol);
    is_valid
}

fn parse_line2(_line: &String) -> u32 {
    0
}

pub fn part_1(input: &Vec<String>) -> i32 {
    let mut part_numbers: Vec<Vec<PartNumber>> = input
        .iter()
        .enumerate()
        .map(|(y, line)| parse_numbers(line, y))
        .collect();
    let symbols: Vec<Vec<Symbol>> = input
        .iter()
        .enumerate()
        .map(|(y, line)| parse_symbols(line, y))
        .collect();

    for (y, line) in symbols.iter().enumerate() {
        for symbol in line {
            if y > 0 {
                // println!("Testing {:?} symbol above {}", symbol, y - 1);
                part_numbers[y - 1] = validate_row(&part_numbers[y - 1], symbol, y);
            }
            // println!("Testing {:?} symbol at {}", symbol, y);
            part_numbers[y] = validate_row(&part_numbers[y], symbol, y);
            if y + 1 < input.len() {
                // println!("Testing {:?} symbol below {}", symbol, y + 1);
                part_numbers[y + 1] = validate_row(&part_numbers[y + 1], symbol, y);
            }
        }
    }

    let valid_numbers: Vec<Vec<i32>> = part_numbers
        .iter()
        .map(|part_numbers| {
            part_numbers
                .iter()
                .filter(|pn| pn.valid)
                .map(|pn| pn.value as i32)
                .collect()
        })
        .enumerate()
        .map(|(idx, pns)| {
            // println!("Valid at line {}: {:?}", idx + 1, pns);
            pns
        })
        .collect();

    valid_numbers.iter().fold(0, |acc, part_numbers| {
        acc + part_numbers
            .iter()
            // .filter(|pn| pn.valid)
            // .map(|pn| pn.value as i32)
            .sum::<i32>()
    })
}

pub fn part_2(_input: &Vec<String>) -> i32 {
    0
    // let sum: u32 = input.iter().map(|line| parse_line2(line)).sum();

    // sum as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_1(&lines), 4361)
    }

    #[test]
    fn part_1_test_above_and_below() {
        let input = "..#...
        ..1...
        ..#...";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_1(&lines), 1)
    }

    #[test]
    fn part_1_test_edge() {
        let input = "#...
        1...";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_1(&lines), 1)
    }

    #[test]
    fn part_1_test_edge2() {
        let input = "1...
        #...";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_1(&lines), 1)
    }

    #[test]
    fn part_1_test_edge3() {
        let input = "...1
        ...#";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_1(&lines), 1)
    }

    #[test]
    fn parse_numbers_test() {
        let input = String::from("123...456..90$9");

        assert_eq!(
            parse_numbers(&input, 1),
            vec![
                PartNumber {
                    x1: 0,
                    x2: 2,
                    y: 1,
                    value: 123,
                    valid: false,
                },
                PartNumber {
                    x1: 6,
                    x2: 8,
                    y: 1,
                    value: 456,
                    valid: false,
                },
                PartNumber {
                    x1: 11,
                    x2: 12,
                    y: 1,
                    value: 90,
                    valid: false,
                },
                PartNumber {
                    x1: 14,
                    x2: 14,
                    y: 1,
                    value: 9,
                    valid: false,
                }
            ]
        )
    }

    #[test]
    fn parse_symbols_test() {
        let input = String::from("123...456..90$");

        assert_eq!(parse_symbols(&input, 1), vec![Symbol { x: 13, y: 1 }])
    }

    #[test]
    fn validate_row_test() {
        let input = vec![
            PartNumber {
                x1: 0,
                x2: 2,
                y: 0,
                value: 100,
                valid: false,
            },
            PartNumber {
                x1: 2,
                x2: 3,
                y: 0,
                value: 200,
                valid: false,
            },
            PartNumber {
                x1: 5,
                x2: 6,
                y: 0,
                value: 10,
                valid: false,
            },
        ];
        let symbol = Symbol { x: 1, y: 0 };

        assert_eq!(
            validate_row(&input, &symbol, 1),
            vec![
                PartNumber {
                    x1: 0,
                    x2: 2,
                    y: 0,
                    value: 100,
                    valid: true,
                },
                PartNumber {
                    x1: 2,
                    x2: 3,
                    y: 0,
                    value: 200,
                    valid: true,
                },
                PartNumber {
                    x1: 5,
                    x2: 6,
                    y: 0,
                    value: 10,
                    valid: false,
                },
            ]
        )
    }

    //     #[test]
    //     fn part_2_test() {
    //         let input = r#"two1nine
    // eightwothree
    // abcone2threexyz
    // xtwone3four
    // 4nineeightseven2
    // zoneight234
    // 7pqrstsixteen"#;

    //         let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
    //         assert_eq!(part_2(&lines), 281)
    //     }
}
