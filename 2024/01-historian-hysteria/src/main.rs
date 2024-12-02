use std::{collections::HashMap, env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input_file = &args[1];

    println!("Finding solution for: {input_file}");

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");
    let (left, right) = separate_id_lists(&input);
    let diff_sum = id_list_difference(&left, &right);
    let sim_score = similarity_score(&left, &right);

    println!("The similarity score for these lists is:\n\n    {sim_score}\n");
    println!("The total sum of the differences of the location ID's is:\n\n    {diff_sum}\n");
}

fn separate_id_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let (l, r) = parse_line(line);

        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    (left, right)
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
