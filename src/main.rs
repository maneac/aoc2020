#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

macro_rules! day {
    ($module:ident, $p1:tt,$p2:tt) => {
        DayRunner {
            num: day_num(stringify!($module)),
            container: Box::new(<$module::Container>::new()),
            part_1_expected: $p1,
            part_2_expected: $p2,
        };
    };
}

use std::{fs::read_to_string, path::Path, time::Instant};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

fn main() {
    let mut days = vec![
        day!(day_1, "542619", "32858450"),
        day!(day_2, "424", "747"),
        day!(day_3, "270", "2122848000"),
        day!(day_4, "206", "123"),
        day!(day_5, "838", "714"),
        day!(day_6, "6504", "3351"),
        day!(day_7, "261", "3765"),
        day!(day_8, "1727", "552"),
    ];

    for day in days.iter_mut() {
        println!("Day {}", day.num);
        let input_string = match read_to_string(Path::new(&format!("./data/day_{}.txt", day.num))) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Failed to read data for day {}: {}", day.num, e);
                continue;
            }
        };

        print!("\tParsing...");
        let start_parse = Instant::now();
        if let Err(e) = day.container.parse_input(&input_string) {
            eprintln!("Failed to parse input for day {}: {}", day.num, e);
            continue;
        }
        let parse_runtime = start_parse.elapsed();
        println!("\r\tParsed - {}ns", parse_runtime.as_nanos());

        print!("\tPart 1...");
        let start_part_1 = Instant::now();
        let part_1 = day.container.part_1();
        let part_1_runtime = start_part_1.elapsed();
        println!("\r\tPart 1 - {}ns", part_1_runtime.as_nanos());
        validate_part(day.num, 1, day.part_1_expected, part_1);

        print!("\tPart 2...");
        let start_part_2 = Instant::now();
        let part_2 = day.container.part_2();
        let part_2_runtime = start_part_2.elapsed();
        println!("\r\tPart 2 - {}ns", part_2_runtime.as_nanos());
        validate_part(day.num, 2, day.part_2_expected, part_2);

        println!(
            "Total: {}ns\n",
            parse_runtime.as_nanos() + part_1_runtime.as_nanos() + part_2_runtime.as_nanos()
        );
    }
}

trait Day {
    fn parse_input(&mut self, input: &str) -> Result<(), String>;
    fn part_1(&self) -> Result<String, String>;
    fn part_2(&self) -> Result<String, String>;
}

struct DayRunner<'day> {
    num: u8,
    container: Box<dyn Day>,
    part_1_expected: &'day str,
    part_2_expected: &'day str,
}

fn validate_part(day: u8, part: u8, expected: &str, result: Result<String, String>) {
    match result {
        Ok(output) => {
            if output != expected {
                eprintln!(
                        "\tIncorrect result for day {}, part {} returned:\n\t\tExpected: {}\n\t\tReturned: {}\n",
                        day, part, expected, output,
                    );
            }
        }
        Err(e) => {
            eprintln!("\tFailed to run day {}, part {}: {}\n", day, part, e);
        }
    };
}

fn day_num(module_name: &str) -> u8 {
    module_name
        .split('_')
        .next_back()
        .unwrap()
        .parse::<u8>()
        .unwrap()
}
