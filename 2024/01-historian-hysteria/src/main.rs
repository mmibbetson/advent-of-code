use std::{collections::HashMap, env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input_file = &args[1];

    println!("Finding solution for: {input_file}");

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");
    let (left, right) = separate_id_lists(&input);
    let sim_score = similarity_score(&left, &right);
    let diff_sum = id_list_difference(&left, &right);

    println!("The similarity score for these lists is:\n\n    {sim_score}\n");
    println!("The total sum of the differences of the location ID's is:\n\n    {diff_sum}\n");
}

fn separate_id_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .fold((Vec::new(), Vec::new()), |(mut acc_l, mut acc_r), line| {
            let (l, r) = parse_line(line);

            acc_l.push(l);
            acc_r.push(r);

            (acc_l, acc_r)
        })
        .inspect(|v| v.sort())
}

fn parse_line(line: &str) -> (i32, i32) {
    let mut parts = line.split_whitespace();

    let left = parts
        .next()
        .unwrap()
        .parse()
        .expect("There was no first location ID on a line.");

    let right = parts
        .next()
        .unwrap()
        .parse()
        .expect("There was no second location ID on a line.");

    (left, right)
}

fn id_list_difference(left: &[i32], right: &[i32]) -> i32 {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum()
}

fn similarity_score(left: &[i32], right: &[i32]) -> i32 {
    let freq_left = left.iter().fold(HashMap::new(), |mut acc, &cur| {
        *acc.entry(cur).or_insert(0) += 1;

        acc
    });

    let freq_right = right.iter().fold(HashMap::new(), |mut acc, &cur| {
        *acc.entry(cur).or_insert(0) += 1;

        acc
    });

    freq_left
        .iter()
        .map(|(&k, &v)| k * v * freq_right.get(&k).cloned().unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{id_list_difference, separate_id_lists, similarity_score};

    #[test]
    fn example_input() {
        // Arrange
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
        let expected_sim_score = 31;
        let expected_list_diff = 11;

        // Act
        let (left, right) = separate_id_lists(&input);
        let result_1 = similarity_score(&left, &right);
        let result_2 = id_list_difference(&left, &right);

        // Assert
        assert_eq!(result_1, expected_sim_score);
        assert_eq!(result_2, expected_list_diff);
    }
}
