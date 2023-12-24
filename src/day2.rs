use crate::utility::split_lines;
use regex::{Captures, Regex};

fn is_possible(game: &str, colour: &str, max_values: i32) -> bool {
    let re = Regex::new(&format!(r"(\d+) {}", colour)).unwrap();
    max_values
        >= re
            .captures_iter(game)
            .map(|c: Captures| -> i32 { c.get(1).unwrap().as_str().parse().unwrap() })
            .max()
            .unwrap_or(0)
}

pub fn problem_one(input: &str) -> i32 {
    let input = split_lines(input);
    let red = 12;
    let green = 13;
    let blue = 14;

    let input: Vec<String> = input
        .into_iter()
        .map(|s| -> String {
            s.chars()
                .skip_while(|c| c != &':')
                .collect::<String>()
                .trim()
                .into()
        })
        .collect();

    let mut result = 0;
    let mut i = 1;
    for game in input {
        let red = is_possible(&game, "red", red);
        let green = is_possible(&game, "green", green);
        let blue = is_possible(&game, "blue", blue);
        if red && green && blue {
            result += i;
        }
        i += 1;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_problem_one() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(problem_one(input), 8);
    }
}
