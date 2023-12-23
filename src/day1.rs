use crate::utility::split_lines;

pub fn problem_one(input: &str) -> i32 {
    let input = split_lines(input);
    input
        .iter()
        .map(|s| {
            let first = s.find(|c: char| c.is_ascii_digit()).unwrap();
            let last = s.rfind(|c: char| c.is_ascii_digit()).unwrap();

            let first = s.get(first..first + 1).unwrap().parse::<i32>().unwrap();
            let last = s.get(last..last + 1).unwrap().parse::<i32>().unwrap();
            first * 10 + last
        })
        .sum()
}

fn translate(word: &str) -> i32 {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => unreachable!(),
    }
}

pub fn problem_two(input: &str) -> i32 {
    const WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let input = split_lines(input);
    let mut output = 0;
    for line in input {
        let mut numbers = vec![];
        let mut line = line.as_str();
        loop {
            if line.is_empty() {
                break;
            }

            for word in WORDS {
                if line.starts_with(word) {
                    numbers.push(translate(word));
                    break;
                }
            }

            if line.starts_with(|c: char| c.is_ascii_digit()) {
                numbers.push(line.get(0..1).unwrap().parse().unwrap());
            }

            line = &line[1..];
        }

        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        output += first * 10 + last;
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_problem_one() {
        let input = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(problem_one(&input), 142);
    }

    #[test]
    fn test_problem_two() {
        let input = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(problem_two(&input), 281);
    }
}
