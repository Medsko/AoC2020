use std::io;

mod utils;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() {
    println!("All things must end - even 2020");

    println!("For which day do you want to generate files?");

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line!");

        let day: usize = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a number!", user_input);
                continue;
            },
        };

        utils::generate_solution_template(day);
        println!("Files for day {} were generated successfully - now get to coding!", day);
    }
}
