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
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;
        let expected = 143;

        // Act
        let result = foo(input);

        // Assert
        assert_eq!(result, expected);
    }
}
