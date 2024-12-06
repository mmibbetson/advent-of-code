fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input_file = &args[1];

    println!("Finding solution for: {input_file}");
}

#[cfg(test)]
mod test {
    #[test]
    fn example_input() {
        // Arrange
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;
        let expected = 41;

        // Act
        let result = foo(input);

        // Assert
        assert_eq!(result, expected);
    }
}
