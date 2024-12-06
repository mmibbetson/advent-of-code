use std::{cmp::Ordering, env, fs, ops::Sub};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Increasing,
    Neutral,
    Decreasing,
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input_file = &args[1];

    println!("Finding solution for: {input_file}");

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");
    let reports = parse_reports(&input);

    let safe_report_count = get_safe_report_count(&reports);
    let safe_report_count_dampened = get_safe_report_count_dampened(&reports);

    println!("The number of safe reports is:\n\n    {safe_report_count}\n");
    println!(
        "The number of safe reports after dampening is:\n\n    {safe_report_count_dampened}\n"
    );
}

fn get_safe_report_count(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|r| is_safe(r)).count()
}

fn get_safe_report_count_dampened(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .map(|r| get_dampened_variants(&r))
        .filter(|v| v.iter().any(|r| is_safe(&r)))
        .count()
}

fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|r| {
            r.split_whitespace()
                .map(|n| n.parse::<i32>().expect("Failed to parse level value"))
                .collect()
        })
        .collect()
}

fn is_safe(levels: &[i32]) -> bool {
    let direction = get_direction(levels[0], levels[1]);

    let (diffs, dirs): (Vec<bool>, Vec<Direction>) = levels
        .iter()
        .skip(1)
        .scan(levels[0], |acc, cur| {
            let diffs = is_valid_diff(*acc, *cur);
            let dirs = get_direction(*acc, *cur);

            *acc = *cur;

            Some((diffs, dirs))
        })
        .unzip();

    diffs.iter().all(|d| *d == true) && dirs.iter().all(|d| *d == direction)
}

fn get_direction(fst: i32, snd: i32) -> Direction {
    match fst.cmp(&snd) {
        Ordering::Greater => Direction::Decreasing,
        Ordering::Equal => Direction::Neutral,
        Ordering::Less => Direction::Increasing,
    }
}

fn is_valid_diff(fst: i32, snd: i32) -> bool {
    let diff = fst.sub(snd).abs();

    diff >= 1 && diff <= 3
}

// TODO: Refactor to be more declarative.
fn get_dampened_variants(levels: &[i32]) -> Vec<Vec<i32>> {
    let mut variants = Vec::new();

    for _ in levels {
        variants.push(Vec::new());
    }

    for (idx, val) in levels.iter().enumerate() {
        for (res_idx, res_val) in variants.iter_mut().enumerate() {
            if res_idx != idx {
                res_val.push(*val);
            }
        }
    }

    variants.push(levels.into());
    variants
}

#[cfg(test)]
mod test {
    use crate::{get_safe_report_count, get_safe_report_count_dampened, parse_reports};

    #[test]
    fn example_input() {
        // Arrange
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;
        let reports = parse_reports(input);
        let expected_sim_score = 2;
        let expected_list_diff = 4;

        // Act
        let result_1 = get_safe_report_count(&reports);
        let result_2 = get_safe_report_count_dampened(&reports);

        // Assert
        assert_eq!(result_1, expected_sim_score);
        assert_eq!(result_2, expected_list_diff);
    }
}
