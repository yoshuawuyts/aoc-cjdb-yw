use std::{convert::Infallible, str::FromStr};

use crate::utility::split_lines;

#[derive(Debug)]
struct Game {
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    fn is_valid(&self) -> bool {
        let red = 12;
        let green = 13;
        let blue = 14;

        red >= self.red && green >= self.green && blue >= self.blue
    }
}

impl FromStr for Game {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let index = line.find(':').unwrap() + 2;
        let (_, line) = line.split_at(index);
        let mut ret = Self {
            red: 0,
            green: 0,
            blue: 0,
        };

        for round in line.split(';') {
            for colour in round.split(',') {
                let (n, colour) = colour.trim().split_once(' ').unwrap();
                let n: i32 = n.parse().unwrap();
                match colour {
                    "red" => ret.red = ret.red.max(n),
                    "green" => ret.green = ret.green.max(n),
                    "blue" => ret.blue = ret.blue.max(n),
                    _ => unreachable!(),
                }
            }
        }

        Ok(ret)
    }
}

pub fn problem_one(input: &str) -> i32 {
    let input = split_lines(input);

    let mut result = 0;
    for (index, line) in input.iter().enumerate() {
        let game: Game = line.parse().unwrap();
        if game.is_valid() {
            result += index + 1;
        }
    }

    result as i32
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
