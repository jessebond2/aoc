use crate::utils::read_lines_to_vec;

pub mod utils;
pub mod y2021;
pub mod y2023;

fn main() {
    let input =
        read_lines_to_vec("./input/2021/day3.txt").expect("Unable to parse file into integers");

    let result_1 = crate::y2021::day3::part_1(&input);
    println!("Result from day3 part 1: {}", result_1);

    let result_2 = crate::y2021::day3::part_2(&input);
    println!("Result from day3 part 2: {}", result_2);

    let input =
        read_lines_to_vec("./input/2021/day4.txt").expect("Unable to parse file into integers");

    let result_1 = crate::y2021::day4::part_1(&input);
    println!("Result from day4 part 1: {}", result_1);

    let result_2 = crate::y2021::day4::part_2(&input);
    println!("Result from day4 part 2: {}", result_2);

    println!();
    println!();
    println!("================================================");
    println!();
    println!("2023");
    println!();
    println!("================================================");
    println!();

    let input =
        read_lines_to_vec("./input/2023/day1.txt").expect("Unable to parse file into integers");

    let result_1 = crate::y2023::day1::part_1(&input);
    println!("Result from day1 part 1: {}", result_1);

    let result_2 = crate::y2023::day1::part_2(&input);
    println!("Result from day1 part 2: {}", result_2);

    let input =
        read_lines_to_vec("./input/2023/day2.txt").expect("Unable to parse file into integers");
    let result_1 = crate::y2023::day2::part_1(&input);
    println!("Result from day2 part 1: {}", result_1);

    let result_2 = crate::y2023::day2::part_2(&input);
    println!("Result from day2 part 2: {}", result_2);

    let input =
        read_lines_to_vec("./input/2023/day3.txt").expect("Unable to parse file into integers");
    let result_1 = crate::y2023::day3::part_1(&input);
    println!("Result from day3 part 1: {}", result_1);

    let result_2 = crate::y2023::day3::part_2(&input);
    println!("Result from day3 part 2: {}", result_2);
}
