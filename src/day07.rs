#![allow(dead_code)]

use std::borrow::BorrowMut;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use super::utils;

// If you decide to solve this using recursive composition, check:
// https://stackoverflow.com/questions/36167160/how-do-i-express-mutually-recursive-data-structures-in-safe-rust

// struct Rule {
//     color: String,
//     holds: HashMap<Rule, usize>,
//     fits: Vec<Rule>,
// }

// After learning about lifetimes:
struct Rule<'a> {
    color: String,
    holds: HashMap<Rule<'a>, usize>,
    fits: Vec<Rule<'a>>,
}


fn part_one() {

}

fn part_two() {

}

fn read_rules(path: &str) -> HashMap<&String, Rule, RandomState> {
    let mut rules: HashMap<&String, Rule> = HashMap::new();
    utils::read_file_lines(path).iter()
        .for_each(|line| parse_rule(line, &mut rules));
    rules
}

fn parse_rule(line: &String, rules: &mut HashMap<&String, Rule>) {
    let mut parts = line.split("bags contain");
    let color = parts.next().unwrap().trim().to_string();
    let mut rule = rules.entry(&color)
        .or_insert(Rule { color, holds: HashMap::new(), fits: vec![] });

    parts.next().unwrap().trim().split(", ")
        .for_each(|contained| parse_contained(contained, &mut rule));
}

fn parse_contained(contained: &str, rule: &mut Rule, rules: &mut HashMap<&String, Rule>) {
    let mut parts = contained.split_whitespace();

    // If the first token is 'no', this bag holds no other bags.
    let count: usize = match parts.next().unwrap().parse() {
        Ok(number) => number,
        Err(e) => return
    };

    let adjective = parts.next().unwrap().to_string();
    let color = adjective + " " + parts.next().unwrap();
    let contained_rule = rules.entry(&color)
        .or_insert(Rule { color, holds: HashMap::new(), fits: vec![] });
    contained_rule.fits.push(rule);
    // No way dat dit goed gaat
    rule.holds.borrow_mut().insert(contained_rule, count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_are_parsed() {
        let rules = read_rules("./resources/test/day07.txt");
        println!("{:?}", rules.values().collect());
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_run_part_one() {

    }

    #[test]
    fn part_one_solution() {

    }

    #[test]
    fn test_run_part_two() {

    }

    #[test]
    fn part_two_solution() {

    }
}