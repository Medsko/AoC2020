#![allow(dead_code)]

use super::utils;

const INPUT_FILE: &str = "./resources/day05.txt";

struct Seat {
    row: u32,
    column: u32
}

impl Seat {
    fn get_seat_id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

pub fn part_one() {
    let all_ids = determine_all_seat_ids();
    let highest_seat_id = all_ids.iter()
        .max()
        .unwrap_or_else(|| panic!("No valid seat could be determined!"));
    println!("Day 5, part one - highest seat id: {}", highest_seat_id);
}

pub fn part_two() {
    let my_seat_id = find_my_seat();
    println!("Day 5, part two - id of my seat: {}", my_seat_id);
}

fn find_my_seat() -> u32 {
    let mut seat_ids = determine_all_seat_ids();
    seat_ids.sort();
    for i in 0..seat_ids.len() - 1 {
        if (seat_ids.get(i + 1).unwrap() - seat_ids.get(i).unwrap()) > 1 {
            return seat_ids.get(i).unwrap() + 1
        }
    }
    panic!("No missing id found!")
}

fn determine_all_seat_ids() -> Vec<u32> {
    utils::read_file_lines(INPUT_FILE).iter()
        .map(|line| determine_seat(line))
        .map(|seat| seat.get_seat_id())
        .collect()
}

fn determine_seat(specification: &String) -> Seat {
    let front_back = &specification[..7];
    let left_right = &specification[7..];
    Seat {
        row: determine_position(front_back),
        column: determine_position(left_right)
    }
}

fn determine_position(specification: &str) -> u32 {
    let base: u32 = 2;
    let seats_in_dimension = base.pow(specification.len() as u32);
    let mut cursor_modifier = seats_in_dimension;
    let mut position: u32 = 0;
    for letter in specification.chars() {
        cursor_modifier /= 2;
        if letter == 'B' || letter == 'R' {
            position += cursor_modifier;
        }
    }
    position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_seat_correctly() {
        let seat_specification = "BFFFBBFRRR".to_string();
        let seat = determine_seat(&seat_specification);

        assert_eq!(70, seat.row);
        assert_eq!(7, seat.column);
        assert_eq!(567, seat.get_seat_id());
    }

    #[test]
    fn determines_position_correctly() {
        let front_back = "FBFBBFF";
        let left_right = "RLR";

        assert_eq!(44, determine_position(front_back));
        assert_eq!(5, determine_position(left_right));
    }

    #[test]
    fn part_one_solution() {
        let all_ids = determine_all_seat_ids();
        assert_eq!(871, *all_ids.iter().max().unwrap());
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(640, find_my_seat());
    }
}