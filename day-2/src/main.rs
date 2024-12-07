use std::collections::btree_map::Range;

fn build(input: &str) -> Vec<Vec<u32>> {
 input
 .lines()
 .map(|line|
        line.split_ascii_whitespace()
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
    )
    .collect::<Vec<Vec<u32>>>()
}

fn dampen_report(report: &[u32]) -> bool {
    // remove first level, check report, remove second, check report, etc until it fails or returns safe
    for i in 0.. report.len() {
        let left = &report[0..i];
        let right = &report[i+1..report.len()];
        let dampened_report = &[left, right].concat()[..];
        let r = check_report(dampened_report);
        if r == true {
            return true;
        }
    }
    return false;
} 

fn check_report(report: &[u32]) -> bool {
    report.windows(2)
    .map(|window| {
        let prev = window[0];
        let next = window[1];
        let is_increasing = prev > next;
        let diff: u32 = prev.abs_diff(next);
        let is_within = diff > 0 && 4 > diff;
        (is_increasing, is_within) 
    })
    .reduce(|acc, e| 
         (e.0, acc.0 == e.0 && acc.1 && e.1)
    ).unwrap().1
}

fn check_reports(reports: &Vec<Vec<u32>>) -> u32 {
    reports
    .iter()
    .filter(|report| check_report(report))
    .count() as u32
}

fn check_dampened_reports(reports: &Vec<Vec<u32>>) -> u32 {
    reports
    .iter()
    .filter(|report| dampen_report(report))
    .count() as u32
}

fn main() {
    println!("Starting!");
    let input: &mut String = &mut String::new();
    reader::read("./input.txt", input);

    let reports = build(&input);

    println!("{}", check_reports(&reports));
    println!("{}", check_dampened_reports(&reports));

    println!("Finished!");

}
