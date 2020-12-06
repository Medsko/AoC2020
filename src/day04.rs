use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use super::utils;

const COUNTRY_ID: &str = "cid";
const INPUT_FILE: &str = "./resources/day04.txt";
const TEST_INPUT_FILE: &str = "./resources/test/day04.txt";

pub fn part_one() {
    let valid_passport_count = parse_passports_from_file(INPUT_FILE).iter()
        .filter(|passport| has_required_fields(*passport))
        .count();
    println!("Answer day 4 part one: {}", valid_passport_count);
}

pub fn part_two() {
    let valid_passport_count = parse_passports_from_file(INPUT_FILE).iter()
        .filter(|passport| is_valid(*passport))
        .count();
    println!("Answer day 4 part two: {}", valid_passport_count);
}

/// Returns true if passport has all required fields (country id is optional).
fn has_required_fields(passport: &HashMap<String, String, RandomState>) -> bool {
    (passport.values().len() == 7 && passport.get(COUNTRY_ID).is_none())
        || passport.values().len() == 8
}

fn is_valid(passport: &HashMap<String, String, RandomState>) -> bool {
    has_required_fields(passport)
        && has_valid_year_values(passport)
        && has_valid_height(passport)
        && has_valid_hair_color(passport)
        && has_valid_eye_color(passport)
        && has_valid_passport_id(passport)
}

fn has_valid_year_values(passport: &HashMap<String, String, RandomState>) -> bool {
    is_valid_year(passport.get("byr").unwrap(), 1920, 2002)
        && is_valid_year(passport.get("iyr").unwrap(), 2010, 2020)
        && is_valid_year(passport.get("eyr").unwrap(), 2020, 2030)
}

fn has_valid_height(passport: &HashMap<String, String, RandomState>) -> bool {
    let value = passport.get("hgt").unwrap();
    let height = value[..value.len() - 2].parse::<u16>();
    if height.is_err() {
        return false;
    }
    let height = height.unwrap();
    let measure = &value[value.len() - 2..value.len()];
    return if measure == "in" {
        height >= 59 && height <= 76
    } else {
        height >= 150 && height <= 193
    }
}

fn has_valid_hair_color(passport: &HashMap<String, String, RandomState>) -> bool {
    let value = passport.get("hcl").unwrap();
    let hash = &value[..1];
    let hex = &value[1..];
    hash == "#" && hex.len() == 6 && i64::from_str_radix(hex, 16).is_ok()
}

fn has_valid_eye_color(passport: &HashMap<String, String, RandomState>) -> bool {
    let value = passport.get("ecl").unwrap();
    let valid_eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valid_eye_colors.contains(&value.trim())
}

fn has_valid_passport_id(passport: &HashMap<String, String, RandomState>) -> bool {
    let value = passport.get("pid").unwrap();
    value.len() == 9 && value.parse::<u32>().is_ok()
}

fn is_valid_year(value: &String, min: u16, max: u16) -> bool {
    match value.parse::<u16>() {
        Ok(year) => year >= min && year <= max,
        Err(_) => false
    }
}

fn parse_passports_from_file(path: &str) -> Vec<HashMap<String, String, RandomState>> {
    let lines = utils::read_file_lines(path);
    let mut current_passport = HashMap::new();
    let mut passports = Vec::new();
    for line in lines.iter() {
        if line.len() == 0 {
            passports.push(current_passport.clone());
            current_passport = HashMap::new();
        } else {
            parse_passport_line(line, &mut current_passport);
        }
    }
    passports.push(current_passport);
    passports
}

fn parse_passport_line(line: &String, passport: &mut HashMap<String, String>) {
    line.split_whitespace()
        .map(|pair| pair.split(":").collect())
        .map(|pair: Vec<&str>| passport.insert(pair[0].to_string(), pair[1].to_string()))
        .filter(|previous_value| previous_value.is_some())
        .for_each(|previous_value|
            panic!("Value {} was overwritten for this passport!", previous_value.unwrap()));
}

#[test]
fn all_passports_are_parsed_from_test_file() {
    let passports = parse_passports_from_file(TEST_INPUT_FILE);

    assert_eq!(4, passports.len());
    let second_passport = &passports[1];
    assert_eq!("028048884".to_string(), *second_passport.get("pid").unwrap());
    assert_eq!("#cfa07d".to_string(), *second_passport.get("hcl").unwrap());
    let fourth_passport = &passports[3];
    assert_eq!("59in".to_string(), *fourth_passport.get("hgt").unwrap());
}

#[test]
fn valid_passports_are_judged_valid() {
    let passports = parse_passports_from_file(TEST_INPUT_FILE);
    for i in (0..4).step_by(2) {
        let valid_passport = &passports[i];
        assert!(has_required_fields(valid_passport))
    }
}

#[test]
fn invalid_passports_are_judged_invalid() {
    let passports = parse_passports_from_file(TEST_INPUT_FILE);
    for i in (1..4).step_by(2) {
        let invalid_passport = &passports[i];
        assert!(!has_required_fields(invalid_passport))
    }
}

#[test]
fn invalid_passports_are_judged_invalid_part_two() {
    let passports = parse_passports_from_file("./resources/test/day04invalid.txt");

    let valid_count = passports.iter().filter(|passport| is_valid(passport)).count();

    assert_eq!(0, valid_count);
}

#[test]
fn valid_passports_are_judged_valid_part_two() {
    let passports = parse_passports_from_file("./resources/test/day04valid.txt");

    let valid_count = passports.iter().filter(|passport| is_valid(passport)).count();

    assert_eq!(4, valid_count);
}
