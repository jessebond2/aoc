use crate::utils::{read_file_to_string, read_lines_to_vec};

pub mod utils;
pub mod y2021;
pub mod y2023;
pub mod y2024;

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

    // let input =
    //     read_lines_to_vec("./input/2023/day1.txt").expect("Unable to parse file into integers");

    // let result_1 = crate::y2023::day1::part_1(&input);
    // println!("Result from day1 part 1: {}", result_1);

    // let result_2 = crate::y2023::day1::part_2(&input);
    // println!("Result from day1 part 2: {}", result_2);

    // let input =
    //     read_lines_to_vec("./input/2023/day2.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day2::part_1(&input);
    // println!("Result from day2 part 1: {}", result_1);

    // let result_2 = crate::y2023::day2::part_2(&input);
    // println!("Result from day2 part 2: {}", result_2);

    // let input =
    //     read_lines_to_vec("./input/2023/day3.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day3::part_1(&input);
    // println!("Result from day3 part 1: {}", result_1);

    // let result_2 = crate::y2023::day3::part_2(&input);
    // println!("Result from day3 part 2: {}", result_2);

    // let input =
    //     read_lines_to_vec("./input/2023/day4.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day4::part_1(&input);
    // println!("Result from day4 part 1: {}", result_1);

    // let result_2 = crate::y2023::day4::part_2(&input);
    // println!("Result from day4 part 2: {}", result_2);

    // let input =
    //     read_lines_to_vec("./input/2023/day5.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day5::part_1(&input);
    // println!("Result from day5 part 1: {}", result_1);

    // // let result_2 = crate::y2023::day5::part_2(&input);
    // let result_2 = 0; // too slow
    // println!("Result from day5 part 2: {}", result_2);

    // let input =
    //     read_lines_to_vec("./input/2023/day6.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day6::part_1(&input);
    // println!("Result from day6 part 1: {}", result_1);

    // let result_2 = crate::y2023::day6::part_2(&input);
    // println!("Result from day6 part 2: {}", result_2);

    // let input =
    //     read_lines_to_vec("./input/2023/day7.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day7::part_1(&input);
    // println!("Result from day7 part 1: {}", result_1);

    // let result_2 = crate::y2023::day7::part_2(&input);
    // println!("Result from day7 part 2: {}", result_2);

    // let input =
    //     read_lines_to_vec("./input/2023/day8.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day8::part_1(&input);
    // println!("Result from day8 part 1: {}", result_1);

    // let result_2 = crate::y2023::day8::part_2(&input);
    // println!("Result from day8 part 2: {}", result_2);

    // let input =
    //     read_lines_to_vec("./input/2023/day9.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day9::part_1(&input);
    // println!("Result from day9 part 1: {}", result_1);

    // let result_2 = crate::y2023::day9::part_2(&input);
    // println!("Result from day9 part 2: {}", result_2);

    // let input =
    //     read_file_to_string("./input/2023/day10.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day10::part_1(&input);
    // println!("Result from day10 part 1: {}", result_1);

    // let result_2 = crate::y2023::day10::part_2(&input);
    // println!("Result from day10 part 2: {}", result_2);

    // let input =
    //     read_file_to_string("./input/2023/day11.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day11::part_1(&input);
    // println!("Result from day11 part 1: {}", result_1);

    // let result_2 = crate::y2023::day11::part_2(&input);
    // println!("Result from day11 part 2: {}", result_2);

    // let input =
    //     read_file_to_string("./input/2023/day12.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day12::part_1(&input);
    // println!("Result from day12 part 1: {}", result_1);

    // let result_2 = crate::y2023::day12::part_2(&input);
    // println!("Result from day12 part 2: {}", result_2);

    // let input =
    //     read_file_to_string("./input/2023/day13.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day13::part_1(&input);
    // println!("Result from day13 part 1: {}", result_1);

    // let result_2 = crate::y2023::day13::part_2(&input);
    // println!("Result from day13 part 2: {}", result_2);

    // let input =
    //     read_file_to_string("./input/2023/day14.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day14::part_1(&input);
    // println!("Result from day14 part 1: {}", result_1);

    // let result_2 = crate::y2023::day14::part_2(&input);
    // println!("Result from day14 part 2: {}", result_2);

    // let input =
    //     read_file_to_string("./input/2023/day15.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day15::part_1(&input);
    // println!("Result from day15 part 1: {}", result_1);

    // let result_2 = crate::y2023::day15::part_2(&input);
    // println!("Result from day15 part 2: {}", result_2);

    // let input =
    //     read_file_to_string("./input/2023/day16.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day16::part_1(&input);
    // println!("Result from day16 part 1: {}", result_1);

    // let result_2 = crate::y2023::day16::part_2(&input);
    // println!("Result from day16 part 2: {}", result_2);

    // let input =
    //     read_file_to_string("./input/2023/day17.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day17::part_1(&input);
    // println!("Result from day17 part 1: {}", result_1);

    // let result_2 = crate::y2023::day17::part_2(&input);
    // println!("Result from day17 part 2: {}", result_2);

    // let input =
    //     read_file_to_string("./input/2023/day18.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day18::part_1(&input);
    // println!("Result from day18 part 1: {}", result_1);

    // let result_2 = crate::y2023::day18::part_2(&input);
    // println!("Result from day18 part 2: {}", result_2);

    // let input =
    //     read_file_to_string("./input/2023/day19.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day19::part_1(&input);
    // println!("Result from day19 part 1: {}", result_1);

    // let result_2 = crate::y2023::day19::part_2(&input);
    // println!("Result from day19 part 2: {}", result_2);

    // let input =
    //     read_file_to_string("./input/2023/day20.txt").expect("Unable to parse file into integers");
    // let result_1 = crate::y2023::day20::part_1(&input);
    // println!("Result from day20 part 1: {}", result_1);

    // let result_2 = crate::y2023::day20::part_2(&input);
    // println!("Result from day20 part 2: {}", result_2);

    println!();
    println!();
    println!("================================================");
    println!();
    println!("2024");
    println!();
    println!("================================================");
    println!();
    let input =
        read_file_to_string("./input/2024/day1.txt").expect("Unable to parse file into integers");
    let result_1 = crate::y2024::day1::part_1(&input);
    println!("Result from day1 part 1: {}", result_1);

    let result_2 = crate::y2024::day1::part_2(&input);
    println!("Result from day1 part 2: {}", result_2);
}
