use std::fs;

mod day1;
mod day2;
pub mod utility;

fn main() {
    let input = get_input("day1");
    println!("day 1.1: {}", day1::problem_one(&input));
    println!("day 1.2: {}", day1::problem_two(&input));

    let input = get_input("day2");
    println!("day 2.1: {}", day2::problem_one(&input));
}

fn get_input(day: &str) -> String {
    fs::read_to_string(day).unwrap()
}
