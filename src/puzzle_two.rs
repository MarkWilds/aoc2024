use std::fs::read_to_string;

type Report = Vec<i32>;

pub fn puzzle_two(input: &str) {
    let reports = get_reports_from_file(input);
    let safe_reports = get_safe_reports(&reports, false);
    let safe_reports_one_bad_level = get_safe_reports(&reports, true);

    println!("Safe reports {}", safe_reports);
    println!("Safe reports with a single bad level {}", safe_reports_one_bad_level);
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
        .filter(|&report| is_safe(report.clone(), tolerate_one_bad_level))
        .count();

    safe_reports
}

fn is_safe(report: Vec<i32>, tolerate_one_bad_level: bool) -> bool {
    let skip_index = try_get_unsafe_index(&report);
    if let Some(index) = skip_index {
        return if tolerate_one_bad_level {
            let count = report.len() as i32;
            let i = index as i32;

            let mut valid_indices = (i-1..=i+1)
                .filter(|&i| i >= 0 && i < count);

            valid_indices
                .any(|index| {
                    let mut sub_report = report.clone();
                    sub_report.remove(index as usize);

                    try_get_unsafe_index(&sub_report)
                        .is_none()
                })
        } else {
            false
        }
    }

    true
}

fn try_get_unsafe_index(report: &Vec<i32>) -> Option<usize> {
    let count = report.len();
    let prev_slope = report[1] - report[0];

    for i in 1..count {
        let slope = report[i] - report[i - 1];
        let is_between_range = i32::abs(slope) <= 3;
        let has_continues_slope =  prev_slope * slope > 0;

        if !(has_continues_slope && is_between_range) {
            return Some(i - 1);
        }
    }

    None
}