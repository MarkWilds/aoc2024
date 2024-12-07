use std::fs::read_to_string;

type Report = Vec<i32>;

pub fn puzzle_two(input: &str) {
    puzzle_two_part_one(input);
    puzzle_two_part_two(input);
}

fn puzzle_two_part_one(input: &str) {
    let reports = get_reports_from_file(input);
    let safe_reports = get_safe_reports(&reports, false);

    println!("Safe reports {}", safe_reports)
}

fn puzzle_two_part_two(input: &str) {
    let reports = get_reports_from_file(input);
    let safe_reports = get_safe_reports(&reports, true);

    println!("Safe reports with a single bad level {}", safe_reports)
}

fn get_reports_from_file(file: &str) -> Vec<Report> {
    read_to_string(file)
        .unwrap()
        .lines()
        .map(|l| l.split_whitespace()
            .filter_map(|r| r.parse::<i32>().ok())
            .collect())
        .collect()
}

fn get_safe_reports(reports: &Vec<Vec<i32>>, tolerate_one_bad_level: bool) -> usize {
    let safe_reports = reports.iter()
        .filter(|report| is_safe(report, tolerate_one_bad_level))
        .count();

    safe_reports
}

fn is_safe(report: &Vec<i32>, mut tolerate_one_bad_level: bool) -> bool {
    let count = report.len();
    let mut prev_diff = 0;

    for mut i in 0..count - 1 {
        let curr_number = report[i];
        let prev_number = report[i + 1];
        let diff = prev_number - curr_number;

        let is_between_range = i32::abs(diff) > 0 && i32::abs(diff) <= 3;
        let has_same_sign = prev_diff * diff >= 0;
        let is_safe = diff != 0 && has_same_sign && is_between_range;

        if !is_safe {
            if tolerate_one_bad_level {
                tolerate_one_bad_level = false;

            } else {
                return false;
            }
        } else {
            prev_diff = diff;
        }
    }

    true
}