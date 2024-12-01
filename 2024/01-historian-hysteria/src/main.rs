use std::{collections::HashMap, env, fs, iter::repeat};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input_file = &args[1];

    println!("Finding solution for: {input_file}");

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file.");
    let (left, right) = separate_id_lists(&input);
    let diff_sum = id_list_difference(&left, &right);
    let sim_score = similarity_score(&left, &right);

    println!("The similarity score for these lists is:\n\n    {sim_score}\n");
    println!("The total sum of the differences of the location ID's is:\n\n    {diff_sum}\n");
}

fn separate_id_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut left, mut right) = input
        .lines()
        .map(|l| {
            let mut iter = l
                .split_whitespace()
                .map(|c| c.parse::<i32>().expect("Failed to parse location ID!"));

            (
                iter.next().expect("There was no first ID on a line."),
                iter.next().expect("There was no second ID on a line."),
            )
        })
        .fold(
            (Vec::new(), Vec::new()),
            |(mut acc_l, mut acc_r), (cur_l, cur_r)| {
                acc_l.push(cur_l);
                acc_r.push(cur_r);

                (acc_l, acc_r)
            },
        );

    left.sort();
    right.sort();

    (left, right)
}

fn id_list_difference(left: &[i32], right: &[i32]) -> i32 {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum()
}

fn similarity_score(left: &[i32], right: &[i32]) -> i32 {
    let mut freq_map: HashMap<i32, i32> = HashMap::from_iter(
        left.iter()
            .map(|n| n.to_owned())
            .zip(repeat(0))
            .collect::<Vec<(i32, i32)>>(),
    );

    for l in left {
        freq_map.insert(*l, freq_map[l] + 1);
    }

    freq_map
        .iter()
        .map(|(k, v)| k * v * right.iter().filter(|r| **r == *k).count() as i32)
        .sum()
}
