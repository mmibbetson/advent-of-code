use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input_file = &args[1];

    println!("Finding solution for: {input_file}");

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");
    let ocurrences = xmas_occurrences(&input);
}

fn xmas_occurrences(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::xmas_occurrences;

    #[test]
    fn example_input() {
        // Arrange
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let expected = 18;

        // Act
        let result = xmas_occurrences(input);

        // Assert
        assert_eq!(result, expected);
    }
}
