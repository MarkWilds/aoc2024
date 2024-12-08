use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct MultiplyExpression {
    enabled: bool,
    x: i32,
    y: i32,
}

impl MultiplyExpression {
    fn new(x: i32, y: i32, enabled: bool) -> Self {
        Self { enabled, x, y }
    }

    fn perform(&self) -> i32 {
        self.x * self.y
    }
}

pub fn puzzle_three(input: &str) {
    let program = get_program_from_file(input);
    let operations = parse(program.as_str());

    let sumOfMultiplies: i32 = operations.iter()
        .map(|m| m.perform())
        .sum();

    let sumOfMultipliesPart2: i32 = operations.iter()
        .filter(|m| m.enabled)
        .map(|m| m.perform())
        .sum();

    println!("Sum of multiplies part1 {}", sumOfMultiplies);
    println!("Sum of multiplies part2 {}", sumOfMultipliesPart2);
}

fn get_program_from_file(file: &str) -> String {
    read_to_string(file).unwrap()
}

fn parse(program: &str) -> Vec<MultiplyExpression> {
    let re = Regex::new(r"(don't\(\)|do\(\))|mul\((\d+),(\d+)\)").unwrap();

    let mut enabled = true;
    re.captures_iter(program)
        .filter_map(|cap| {
            let mut result = None;
            match &cap[0] {
                "don't()" => {enabled = false},
                "do()" => {enabled = true},
                _ => {
                    result = Some(MultiplyExpression::new(cap[2].parse::<i32>().unwrap(), cap[3].parse::<i32>().unwrap(), enabled))
                }
            }

            result
        }).collect()
}
