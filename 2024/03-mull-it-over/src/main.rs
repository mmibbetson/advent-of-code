use std::{env, fs};

use regex::Regex;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input_file = &args[1];

    println!("Finding solution for: {input_file}");

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");
    let input_trimmed = trim_instructions(&input);

    let valid_sum = valid_mul_sum(&input_trimmed);

    println!("The sum of valid mul() instructions is:\n\n    {valid_sum}\n");
}

fn trim_instructions(input: &str) -> String {
    let pattern_do = Regex::new(r#"do\(\)"#).unwrap();
    // NOTE: There is a failing edge case when matching .* instead of [\s\S]*
    // since . does not match newline characters.
    let pattern_dont = Regex::new(r#"don't\(\)[\s\S]*"#).unwrap();

    pattern_do
        .split(&input)
        .flat_map(|s| pattern_dont.split(s))
        .collect()
}

fn valid_mul_sum(input: &str) -> i32 {
    // Regex to capture l,r from mul(l,r) occurrences.
    Regex::new(r#"mul\(([0-9]{1,3},[0-9]{1,3})\)"#)
        .unwrap()
        .captures_iter(&input)
        .map(|c| {
            c.get(1)
                .unwrap()
                .as_str()
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .product::<i32>()
        })
        .sum::<i32>()
}

#[cfg(test)]
mod test {
    use crate::{trim_instructions, valid_mul_sum};

    #[test]
    fn example_input_1() {
        // Arrange
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        let expected = 161;

        // Act
        let result = valid_mul_sum(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn example_input_2() {
        // Arrange
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        let input_trimmed = trim_instructions(&input);
        let expected = 48;

        // Act
        let result = valid_mul_sum(&input_trimmed);

        // Assert
        assert_eq!(result, expected);
    }
}
