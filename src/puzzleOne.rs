use crate::uneven_even_element_iterator::*;
use std::fs::read_to_string;
use std::iter::zip;

pub fn puzzle_one(input: &str) {
    let numbers = get_numbers_from_file(input);
    let total_distance = get_total_distance(&numbers);
    let similarity_score = get_similarity_score(&numbers);

    println!("Distance total is {}", total_distance);
    println!("Similarity score {}", similarity_score)
}

fn get_total_distance(numbers: &Vec<i32>) -> i32 {
    let mut even_sorted: Vec<_> = UnEvenEvenElementIterator::new(numbers, 0)
        .collect();
    even_sorted.sort();

    let mut un_even_sorted: Vec<_> = UnEvenEvenElementIterator::new(numbers, 1)
        .collect();
    un_even_sorted.sort();

    zip(even_sorted.iter(), un_even_sorted.iter())
        .map(|t| i32::abs(t.0 - t.1))
        .sum()
}

fn get_similarity_score(numbers: &Vec<i32>) -> i32 {
    UnEvenEvenElementIterator::new(numbers, 0)
        .map(|e|
            e * UnEvenEvenElementIterator::new(numbers, 1)
                .filter(|ue| ue == &e)
                .count() as i32)
        .sum()
}

fn get_numbers_from_file(file: &str) -> Vec<i32> {
    read_to_string(file)
        .unwrap()
        .lines()
        .flat_map(|l| l.split_whitespace())
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}