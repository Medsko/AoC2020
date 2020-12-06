use super::utils;

const INPUT_FILE: &str = "./resources/dayOne.txt";

pub fn part_one() {
    let numbers = utils::read_numbers_from_file(INPUT_FILE);
    let mut current: Vec<u32> = Vec::new();
    let sum_2020_multiplied = find_sum_2020(numbers, &mut current, 2);
    println!("Answer day 1 part one: {}", sum_2020_multiplied);
}

pub fn part_two() {
    let numbers = utils::read_numbers_from_file(INPUT_FILE);
    let mut current: Vec<u32> = Vec::new();
    let sum_2020_multiplied = find_sum_2020(numbers, &mut current, 3);
    println!("Answer day 1 part two: {}", sum_2020_multiplied);
}

fn find_sum_2020_recursive(numbers: Vec<u32>, size: usize) -> u32 {
    let mut current: Vec<u32> = Vec::new();
    let mut combinations: Vec<Vec<u32>> = Vec::new();
    utils::find_combinations(numbers, &mut current, &mut combinations, size);

    for combination in combinations {
        let sum: u32 = combination.iter().sum();
        if sum == 2020 {
            return combination.iter().fold(1, |a, b| a * b)
        }
    }
    0
}

fn find_sum_2020(numbers: Vec<u32>, current: &mut Vec<u32>, size: usize) -> u32 {

    for index in 0..numbers.len() {
        let mut available_numbers = numbers.clone();
        current.push(available_numbers[index]);
        available_numbers.drain(0..index + 1);

        if current.len() == size {
            // Check whether we have found the winning combination
            if 2020 == current.iter().sum::<u32>() {
                return current.iter()
                    .fold(1, |a, b| a* b);
            }
            current.remove(size - 1);
            continue;
        }
        find_sum_2020(available_numbers, current, size);
        current.remove(current.len() - 1);
    }
    0
}

fn get_test_input() -> Vec<u32> {
    vec![1721, 979, 366, 299, 675, 1456]
}

#[test]
fn test_run_part_1() {
    let numbers = get_test_input();
    let mut current: Vec<u32> = Vec::new();
    let answer = find_sum_2020(numbers, &mut current, 2);
    assert_eq!(answer, 514579);
}

#[test]
fn test_run_part_2() {
    let numbers = utils::read_numbers_from_file(INPUT_FILE);
    let answer = find_sum_2020_recursive(numbers, 3);
    assert_eq!(answer, 241861950);
}

#[test]
fn test_run_part_2_final() {
    let mut current: Vec<u32> = Vec::new();
    let numbers = utils::read_numbers_from_file(INPUT_FILE);

    let answer = find_sum_2020(numbers, &mut current, 3);
    assert_eq!(answer, 274879808);
}
