fn convert_binary_string_to_i32(string: &String) -> i32 {
    let sum: u32 = string
        .chars()
        .rev()
        .enumerate()
        .map(|(index, value)| value.to_digit(10).unwrap() << index)
        .sum();
    sum as i32
}

fn convert_binary_vec_to_i32(vec: &Vec<i32>) -> i32 {
    vec.iter()
        .rev()
        .enumerate()
        .map(|(index, value)| value << index)
        .sum()
}

fn gamma_and_epsilon_vecs(input: &Vec<String>, favor_gamma: bool) -> (Vec<i32>, Vec<i32>) {
    let len = input[0].len();
    let mut zero_count_vec = vec![0; len];
    let mut one_count_vec = vec![0; len];

    let mut gamma_vec: Vec<i32> = vec![0; len];
    let mut epsilon_vec: Vec<i32> = vec![0; len];

    for i in input.iter() {
        for (i, c) in i.chars().enumerate() {
            match c {
                '0' => {
                    zero_count_vec[i] = zero_count_vec[i] + 1;
                }
                '1' => {
                    one_count_vec[i] = one_count_vec[i] + 1;
                }
                _ => {
                    panic!("Something broke");
                }
            }
        }
    }

    for i in 0..len {
        // println!(
        //     "idx {}, z count {}, o count {}",
        //     i, zero_count_vec[i], one_count_vec[i]
        // );
        if zero_count_vec[i] == one_count_vec[i] {
            if favor_gamma {
                gamma_vec[i] = 1;
            } else {
                epsilon_vec[i] = 1;
            }
        } else if zero_count_vec[i] > one_count_vec[i] {
            gamma_vec[i] = 1;
        } else {
            epsilon_vec[i] = 1;
        }
    }
    // println!("gamma_vec {:?}, epsilon_vec {:?}", gamma_vec, epsilon_vec);
    (gamma_vec, epsilon_vec)
}

fn oxygen_rating(input: &Vec<String>) -> i32 {
    let mut ratings = input.clone();
    let mut index = 0;

    while ratings.len() > 1 {
        let zeros_or_ones: i32 = ratings
            .iter()
            .map(|s| match s.chars().nth(index).unwrap() {
                '0' => -1,
                _ => 1,
            })
            .sum();
        let mut char = '0';
        if zeros_or_ones >= 0 {
            char = '1';
        };

        ratings = ratings
            .into_iter()
            .filter(|r| r.chars().nth(index).unwrap() == char)
            .collect();

        index += 1;
    }

    let rating = ratings.iter().next().unwrap();
    convert_binary_string_to_i32(rating)
}

fn co2_rating(input: &Vec<String>) -> i32 {
    let mut ratings = input.clone();
    let mut index = 0;

    while ratings.len() > 1 {
        let zeros_or_ones: i32 = ratings
            .iter()
            .map(|s| match s.chars().nth(index).unwrap() {
                '0' => -1,
                _ => 1,
            })
            .sum();
        let mut char = '1';
        if zeros_or_ones >= 0 {
            char = '0';
        };

        ratings = ratings
            .into_iter()
            .filter(|r| r.chars().nth(index).unwrap() == char)
            .collect();

        index += 1;
    }

    let rating = ratings.iter().next().unwrap();
    convert_binary_string_to_i32(rating)
}

pub fn part_1(input: &Vec<String>) -> i32 {
    let (gamma_vec, epsilon_vec) = gamma_and_epsilon_vecs(input, true);
    let epsilon = convert_binary_vec_to_i32(&epsilon_vec);
    let gamma = convert_binary_vec_to_i32(&gamma_vec);

    epsilon * gamma
}

pub fn part_2(input: &Vec<String>) -> i32 {
    let oxygen_rating = oxygen_rating(input);
    let co2_scrubber_ratting = co2_rating(input);

    oxygen_rating * co2_scrubber_ratting
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = r#"00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010"#;
        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        let result = part_1(&lines);

        assert_eq!(result, 198);
    }

    #[test]
    fn part_two_test() {
        let input = r#"00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010"#;
        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();

        assert_eq!(part_2(&lines), 230)
    }

    #[test]
    fn oxygen_generator_rating_test() {
        let input = r#"00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010"#;
        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();

        assert_eq!(oxygen_rating(&lines), 23);
    }

    #[test]
    fn co2_rating_test() {
        let input = r#"00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010"#;
        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();

        assert_eq!(co2_rating(&lines), 10);
    }
}
