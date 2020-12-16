#![allow(dead_code)]

use std::collections::HashSet;
use std::fs::read_to_string;

const TEST_FILE: &str = "./resources/test/day06.txt";
const PUZZLE_INPUT: &str = "./resources/day06.txt";

fn part_one(path: &str) -> u32 {
    count_answers(path, |group| find_unique_answers(group))
}

fn part_two(path: &str) -> u32 {
    count_answers(path, |group| find_common_answers(group))
}

fn count_answers(path: &str, find: fn(Vec<String>) -> String) -> u32 {
    read_groups_from_file(path).into_iter()
        .map(|group| find(group))
        .fold(0, |a, b| a + b.len() as u32)
}

fn read_groups_from_file(path: &str) -> Vec<Vec<String>> {
    read_to_string(path)
        .expect("Error while reading file!")
        .split("\n\n")
        .map(|group| group.split("\n")
                .map(|line| line.to_string())
                .collect()
        )
        .collect()
}

fn find_common_answers(forms: Vec<String>) -> String {
    let mut common: Vec<char> = forms.get(0).unwrap().chars().collect();
    for i in 1..forms.len() {
        let current = forms.get(i).unwrap();
        common = common.into_iter()
            .filter(|ch| current.contains(*ch))
            .collect();
    }
    common.iter().collect()
}

fn find_unique_answers(forms: Vec<String>) -> String {
    let unique_answers: HashSet<char> = forms.iter()
        .flat_map(|form| form.chars())
        .collect();
    unique_answers.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_are_read_from_file() {
        let groups = read_groups_from_file(TEST_FILE);
        assert_eq!(5, groups.len());
    }

    #[test]
    fn test_run_part_one() {
        assert_eq!(11, part_one(TEST_FILE));
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(6170, part_one(PUZZLE_INPUT));
    }

    #[test]
    fn test_run_part_two() {
        assert_eq!(6, part_two(TEST_FILE));
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(2947, part_two(PUZZLE_INPUT));
    }
}
