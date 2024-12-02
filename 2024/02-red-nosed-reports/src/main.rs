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
    let safe_report_count = input.lines().filter(|l| is_safe(l)).count();

    println!("The number of safe reports is:\n\n    {safe_report_count}\n");
}

fn is_safe(report: &str) -> bool {
    let levels = report
        .split_whitespace()
        .map(|n| n.parse::<i32>().expect("Failed to parse level value"))
        .collect::<Vec<_>>();
    let direction = get_direction(levels[0], levels[1]);

    let (diffs, dirs): (Vec<bool>, Vec<Direction>) = levels
        .iter()
        .skip(1)
        // TODO: Somewhere before unzipping, can try applying the Problem Dampener.
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
