use std::fs::{copy, File};
use std::io::{BufRead, BufReader};

pub fn find_combinations(numbers: Vec<u32>,
                         current: &mut Vec<u32>,
                         combinations: &mut Vec<Vec<u32>>,
                         size: usize) {

    for index in 0..numbers.len() {
        let mut available_numbers = numbers.clone();
        current.push(available_numbers[index]);
        available_numbers.drain(0..index + 1);

        if current.len() == size {
            combinations.push(current.clone());
            current.remove(size - 1);
            continue;
        }
        find_combinations(available_numbers, current, combinations, size);
        current.remove(current.len() - 1);
    }
}

pub fn read_file_lines(path: &str) -> Vec<String> {
    if let Ok(file) = File::open(path) {
        return BufReader::new(file).lines()
            .map(|line| line.unwrap())
            .collect();
    }
    Vec::new()
}

pub fn read_numbers_from_file(path: &str) -> Vec<u32> {
    read_file_lines(path).iter()
        .filter_map(|num| num.trim().parse().ok())
        .collect()
}

pub fn generate_solution_template(day: usize) {
    let rust_file = format!("./src/day{:02}.rs", day);
    let test_input_file = format!("./resources/test/day{:02}.txt", day);
    let input_file = format!("./resources/day{:02}.txt", day);

    copy("./resources/rust_file_template.txt", &rust_file)
        .expect(&format!("Failed to create Rust file at: {}", &rust_file));
    File::create(&test_input_file)
        .expect(&format!("Failed to create test input file at: {}", &test_input_file));
    File::create(&input_file)
        .expect(&format!("Failed to create input file at: {}", &input_file));
}

#[test]
fn all_combinations_found() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    let mut current: Vec<u32> = Vec::new();
    let mut combinations: Vec<Vec<u32>> = Vec::new();
    find_combinations(input, &mut current, &mut combinations, 3);

    assert_eq!(combinations.len(), 20);
}

#[test]
fn combinations_have_correct_size() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    let mut current: Vec<u32> = Vec::new();
    let mut combinations: Vec<Vec<u32>> = Vec::new();
    find_combinations(input, &mut current, &mut combinations, 3);

    for combination in combinations {
        println!("{:?}", combination);
        assert_eq!(combination.len(), 3);
    }
}

#[test]
fn all_lines_are_read_from_file() {
    let lines = read_file_lines("./resources/day01.txt");
    assert_eq!(lines.len(), 200);
}

#[test]
fn all_numbers_in_file_are_parsed() {
    let total: u32 = read_numbers_from_file("./resources/day01.txt").iter().sum();
    assert_eq!(total, 321845)
}

#[test]
fn solution_template_is_generated() {
    generate_solution_template(30);
}