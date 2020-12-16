#![allow(dead_code)]

use super::utils;

const INPUT_FILE: &str = "./resources/day02.txt";

struct PasswordEntry {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

pub fn part_one() {
    let valid_passwords_count = read_passwords_from_file().iter()
        .filter(|password| is_valid(*password))
        .count();
    println!("Answer day 2 part one: {}", valid_passwords_count);
}

pub fn part_two() {
    let valid_toboggan_passwords_count = read_passwords_from_file().iter()
        .filter(|password| is_valid_toboggan(*password))
        .count();
    println!("Answer day 2 part two: {}", valid_toboggan_passwords_count);
}

fn read_passwords_from_file() -> Vec<PasswordEntry> {
    utils::read_file_lines(INPUT_FILE).iter()
        .map(|line| parse_password_entry(line))
        .collect()
}

fn parse_password_entry(line: &str) -> PasswordEntry {
    let mut parts = line.split_whitespace();
    let mut occurrences = parts.next().unwrap().split("-");
    let min: usize = occurrences.next().unwrap().parse().unwrap();
    let max: usize = occurrences.next().unwrap().parse().unwrap();

    let letter: char = parts.next().unwrap().replace(":", "").parse().unwrap();
    let password = parts.next().unwrap().to_string();

    PasswordEntry {
        min,
        max,
        letter,
        password
    }
}

fn is_valid(entry: &PasswordEntry) -> bool {
    let occurrences = entry.password.matches(entry.letter).count();
    occurrences >= entry.min && occurrences <= entry.max
}

fn is_valid_toboggan(entry: &PasswordEntry) -> bool {
    let chars: Vec<char> = entry.password.chars().collect();
    // One of the given indices should point to the specified letter. Index is 1-based!
    (chars[entry.min - 1] == entry.letter) ^ (chars[entry.max - 1] == entry.letter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_is_parsed_into_password_entry() {
        let input = "15-16 l: klfbblslvjclmlnqklvg";
        let parsed = parse_password_entry(input);

        assert_eq!(parsed.min, 15);
        assert_eq!(parsed.max, 16);
        assert_eq!(parsed.letter, 'l');
        assert_eq!(parsed.password, "klfbblslvjclmlnqklvg".to_string());
    }

    #[test]
    fn valid_password_is_judged_valid() {
        let valid_password = parse_password_entry("1-3 a: abcde");
        assert!(is_valid(&valid_password));
    }

    #[test]
    fn invalid_passwords_are_judged_invalid() {
        let invalid_passwords = vec![
            "1-3 b: cdefg",
            "1-6 t: rttftttttttttmdttttt"
        ];

        invalid_passwords.iter()
            .map(|password| parse_password_entry(password))
            .for_each(|entry| assert!(!is_valid(&entry)));
    }

    #[test]
    fn test_run_part_one() {
        let valid_password_count = get_test_input().iter()
            .map(|password| parse_password_entry(password))
            .filter(|entry| is_valid(entry))
            .count();

        assert_eq!(2, valid_password_count);
    }

    #[test]
    fn test_run_part_two() {
        let valid_toboggan_password_count = get_test_input().iter()
            .map(|password| parse_password_entry(password))
            .filter(|entry| is_valid_toboggan(entry))
            .count();

        assert_eq!(1, valid_toboggan_password_count);
    }

    fn get_test_input() -> Vec<&'static str> {
        vec![
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc"
        ]
    }
}