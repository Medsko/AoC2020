use super::utils;

const INPUT_FILE: &str = "./resources/dayOne.txt";

pub fn part_one() -> u32 {
    let numbers = utils::read_numbers_from_file(INPUT_FILE);
    find_sum_2020(numbers)
}

pub fn part_two() -> u32 {
    let numbers = utils::read_numbers_from_file(INPUT_FILE);
    find_sum_2020_recursive(numbers, 3)
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


fn find_sum_2020(mut numbers: Vec<u32>) -> u32 {
    numbers.sort();
    'outer: for (index, number) in numbers.iter().enumerate() {
        for i in index..numbers.len() {
            if number + numbers[i] == 2020 {
                return number * numbers[i]
            } else if number + numbers[i] > 2020 {
                continue 'outer
            }
        }
    }
    panic!("Could not find the two numbers with sum 2020!")
}

fn get_test_input() -> Vec<u32> {
    vec![1721, 979, 366, 299, 675, 1456]
}

#[test]
fn test_run_part_2() {
    let answer = find_sum_2020_recursive(get_test_input(), 3);
    assert_eq!(answer, 241861950);
}
