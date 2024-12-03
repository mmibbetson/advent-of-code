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
    let reports = input.lines().map(|r| {
        r.split_whitespace()
            .map(|n| n.parse::<i32>().expect("Failed to parse level value"))
            .collect::<Vec<_>>()
    });

    let dampened = reports.clone().map(|r| get_dampened_variants(&r));

    let safe_report_count = reports.clone().filter(|l| is_safe(l)).count();
    let safe_report_count_dampened = reports.filter(|l| is_safe_dampened(l)).count();

    println!("The number of safe reports is:\n\n    {safe_report_count}\n");
    println!(
        "The number of safe reports after dampening is:\n\n    {safe_report_count_dampened}\n"
    );
}

fn is_safe(levels: &[i32]) -> bool {
    let direction = get_direction(levels[0], levels[1]);

    let (diffs, dirs): (Vec<bool>, Vec<Direction>) = levels
        .iter()
        .skip(1)
        .scan(levels[0], |acc, cur| {
            let diffs = is_valid_diff(*acc, *cur);
            let dirs = get_direction(*acc, *cur);

            // I hate this so much.
            *acc = *cur;

            Some((diffs, dirs))
        })
        .unzip();

    diffs.iter().all(|d| *d == true) && dirs.iter().all(|d| *d == direction)
}

fn is_safe_dampened(levels: &[i32]) -> bool {
    let direction = get_direction(levels[0], levels[1]);

    let (diffs, dirs): (Vec<bool>, Vec<Direction>) = levels
        .iter()
        .skip(1)
        .scan(levels[0], |acc, cur| {
            let diffs = is_valid_diff(*acc, *cur);
            let dirs = get_direction(*acc, *cur);

            // I hate this so much.
            *acc = *cur;

            Some((diffs, dirs))
        })
        .unzip();

    diffs.iter().all(|d| *d == true) && dirs.iter().all(|d| *d == direction)
}

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

    variants
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
