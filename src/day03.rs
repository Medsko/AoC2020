use super::utils;

const INPUT_FILE: &str = "./resources/day03.txt";

struct Slope(usize, usize);

pub fn part_one() {
    let pattern = read_pattern_from_file(INPUT_FILE);
    let trees_encountered = traverse_slope(&pattern, &Slope(3, 1));
    println!("Answer day 3 part one: {}", trees_encountered);
}

pub fn part_two() {
    let pattern = read_pattern_from_file(INPUT_FILE);
    let trees_encountered_multiplied = traverse_slope_rubber_banded(pattern, get_slopes());
    println!("Answer day 3 part two: {}", trees_encountered_multiplied);
}

fn traverse_slope_rubber_banded(pattern: Vec<Vec<char>>, slopes: Vec<Slope>) -> u32 {
    let mut trees_encountered_multiplied = 1;

    for slope in slopes.iter() {
        trees_encountered_multiplied *= traverse_slope(&pattern, slope);
    }
    trees_encountered_multiplied
}

fn traverse_slope(pattern: &Vec<Vec<char>>, slope: &Slope) -> u32 {
    let mut x_index = 0;
    let mut trees_encountered: u32 = 0;
    let pattern_width = pattern[0].len();

    for y_index in (slope.1..pattern.len()).step_by(slope.1) {
        x_index += slope.0;
        // Pattern repeats from left two right indefinitely, so x index should wrap around.
        if x_index >= pattern_width {
            x_index -= pattern_width;
        }
        if pattern[y_index][x_index] == '#' {
            trees_encountered += 1;
        }
    }
    trees_encountered
}

fn read_pattern_from_file(path: &str) -> Vec<Vec<char>> {
    utils::read_file_lines(path).iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_slopes() -> Vec<Slope> {
    vec![
        Slope(1, 1),
        Slope(3, 1),
        Slope(5, 1),
        Slope(7, 1),
        Slope(1, 2),
    ]
}

#[test]
fn pattern_is_read_from_file() {
    let pattern = read_pattern_from_file(INPUT_FILE);

    assert_eq!(323, pattern.len());
    assert_eq!(31, pattern[0].len());
    assert_eq!('#', pattern[0][2]);
}

#[test]
fn test_run_part_one() {
    let pattern = read_pattern_from_file("./resources/test/day03.txt");

    let trees_encountered = traverse_slope(&pattern, &Slope(3, 1));

    assert_eq!(7, trees_encountered);
}

#[test]
fn part_one_solution() {
    let pattern = read_pattern_from_file("./resources/day03.txt");

    let trees_encountered = traverse_slope(&pattern, &Slope(3, 1));

    assert_eq!(262, trees_encountered);
}

#[test]
fn test_run_part_two() {
    let pattern = read_pattern_from_file("./resources/test/day03.txt");
    let mut multiplied_encounters = 1;
    for slope in get_slopes().iter() {
        let trees_encountered = traverse_slope(&pattern, slope);
        println!("Trees encountered on slope {},{}: {}", slope.0, slope.1, trees_encountered);
        multiplied_encounters *= trees_encountered;
    }

    assert_eq!(336, multiplied_encounters);
}

#[test]
fn part_two_solution() {
    let pattern = read_pattern_from_file("./resources/day03.txt");

    let multiplied_encounters = traverse_slope_rubber_banded(pattern, get_slopes());

    assert_ne!(2791966320, multiplied_encounters);
    assert_eq!(2698900776, multiplied_encounters);
}