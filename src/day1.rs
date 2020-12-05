use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn part_one() -> u32 {
    find_sum_2020(read_from_file())
}

pub fn part_two() -> u32 {
    find_sum_2020_recursive(read_from_file(), 3)
}

fn find_sum_2020_recursive(numbers: Vec<u32>, size: usize) -> u32 {
    let mut current: Vec<u32> = Vec::new();
    let mut combinations: Vec<Vec<u32>> = Vec::new();
    combinations_of_size(numbers, &mut current, &mut combinations, size);

    for combination in combinations {
        let sum: u32 = combination.iter().sum();
        if sum == 2020 {
            return combination.iter().fold(1, |a, b| a * b)
        }
    }
    0
}

fn combinations_of_size(numbers: Vec<u32>,
                        current: &mut Vec<u32>,
                        combinations: &mut Vec<Vec<u32>>,
                        size: usize) {
    for index in 0..numbers.len() {
        current.push(numbers[index]);
        let mut available_numbers = numbers.clone();
        available_numbers.remove(index);

        if current.len() == size {
            combinations.push(current.clone());
            current.remove(size - 1);
            continue;
        }
        combinations_of_size(available_numbers, current, combinations, size);
        current.remove(current.len() - 1);
    }
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

fn read_from_file() -> Vec<u32> {
    let file = match File::open("./resources/dayOne.txt") {
        Ok(f) => f,
        Err(e) => panic!(e)
    };

    let reader = BufReader::new(&file);
    let mut numbers: Vec<u32> = Vec::new();
    for line in reader.lines() {
        let nr_string = line.unwrap();
        println!("{}", nr_string);
        let nr = match nr_string.trim().parse() {
            Ok(num) => num,
            Err(e) => panic!(e)
        };
        numbers.push(nr)
    }
    numbers
}

fn get_test_input() -> Vec<u32> {
    vec![1721, 979, 366, 299, 675, 1456]
}

#[test]
fn test_run_part_2() {
    let answer = find_sum_2020_recursive(get_test_input(), 3);
    assert_eq!(answer, 241861950);
}

#[test]
fn combinations_have_correct_size() {
    let input = get_test_input();
    let mut current: Vec<u32> = Vec::new();
    let mut combinations: Vec<Vec<u32>> = Vec::new();
    combinations_of_size(input, &mut current, &mut combinations, 3);

    for combination in combinations {
        println!("{:?}", combination);
        assert_eq!(combination.len(), 3);
    }
}

#[test]
fn all_combinations_are_found() {
    // Currently broken: result will contain duplicates (also explains shitty performance)
    let input = get_test_input();
    let mut current: Vec<u32> = Vec::new();
    let mut combinations: Vec<Vec<u32>> = Vec::new();
    combinations_of_size(input, &mut current, &mut combinations, 3);

    assert_eq!(combinations.len(), 20);
}
