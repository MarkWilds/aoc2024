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

fn is_safe(mut report: Vec<i32>, tolerate_one_bad_level: bool) -> bool {
    let skip_index = try_get_unsafe_index(&report);
    if let Some(index) = skip_index {
        if tolerate_one_bad_level {
            report.remove(index);
            return try_get_unsafe_index(&report)
                .is_none();
        } else {
            return false;
        }
    }

    true
}

fn try_get_unsafe_index(report: &Vec<i32>) -> Option<usize> {
    let count = report.len();
    let mut is_ascending = report[1] > report[0];

    for i in 1..count {
        let delta = report[i] - report[i - 1];
        let is_between_range = i32::abs(delta) > 0 && i32::abs(delta) <= 3;
        let has_continues_slope =  is_ascending == (report[i] > report[i - 1]);

        if !(has_continues_slope && is_between_range) {
            return Some(i - 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::puzzle_two::{is_safe, Report};

    #[rstest]
    #[case(vec![2,2,3], true, true)]
    #[case(vec![2,2,3], false, false)]
    #[case(vec![2,2], false, false)]
    #[case(vec![2,2,4,6,8], false, false)]
    #[case(vec![2,2,4,6,8], true, true)]
    #[case(vec![2,2,4,6,8,8], true, false)]
    #[case(vec![8, 6, 4, 4, 1], true, true)]
    #[case(vec![8, 6, 4, 4, 1], false, false)]
    #[case(vec![1, 3, 2, 4, 5], false, false)]
    #[case(vec![1, 3, 2, 4, 5], true, true)]
    #[case(vec![9, 7, 6, 2, 1], false, false)]
    #[case(vec![9, 7, 6, 2, 1], true, false)]
    #[case(vec![7, 6, 4, 2, 1], false, true)]
    #[case(vec![7, 6, 4, 2, 1], true, true)]
    #[case(vec![7, 6, 4, 3, 2, 2], true, true)]
    #[case(vec![7, 6, 4, 3, 0, -4], true, false)]
    #[case(vec![2, 2, 5], false, false)]
    #[case(vec![2, 2, 5], true, true)]
    fn test_is_safe(#[case] report: Report, #[case] bad_level_toleration: bool, #[case] expected: bool) {
        assert_eq!(is_safe(report, bad_level_toleration), expected);
    }
}