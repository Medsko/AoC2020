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
    let seat_with_highest_id = utils::read_file_lines(INPUT_FILE).iter()
        .map(|line| determine_seat(line))
        .max_by(|seat, other| seat.get_seat_id().cmp(&other.get_seat_id()))
        .unwrap_or_else(|| panic!("No valid seat could be determined!"));
    println!("Highest seat id: {}", seat_with_highest_id.get_seat_id());
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