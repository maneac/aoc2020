#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

macro_rules! day {
    ($module:ident, $p1:tt,$p2:tt) => {
        DayRunner {
            num: day_num(stringify!($module)),
            container: Box::new(<$module::solution::Container>::new()),
            part_1_expected: $p1,
            part_2_expected: $p2,
        };
    };
}

use std::{
    fs::read_to_string,
    path::Path,
    time::{Duration, Instant},
};

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

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
        day!(day_9, "138879426", "23761694"),
        day!(day_10, "2450", "32396521357312"),
        day!(day_11, "2273", "2064"),
        day!(day_12, "362", "29895"),
    ];

    let mut total_time = Duration::new(0, 0);
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
        println!("\r\tParsed - {}", format_time(&parse_runtime));

        print!("\tPart 1...");
        let start_part_1 = Instant::now();
        let part_1 = day.container.part_1();
        let part_1_runtime = start_part_1.elapsed();
        println!("\r\tPart 1 - {}", format_time(&part_1_runtime));
        validate_part(day.num, 1, day.part_1_expected, part_1);

        print!("\tPart 2...");
        let start_part_2 = Instant::now();
        let part_2 = day.container.part_2();
        let part_2_runtime = start_part_2.elapsed();
        println!("\r\tPart 2 - {}", format_time(&part_2_runtime));
        validate_part(day.num, 2, day.part_2_expected, part_2);

        let sub_time_total = parse_runtime + part_1_runtime + part_2_runtime;
        println!("Day {} time: {}\n", day.num, format_time(&sub_time_total));
        total_time = total_time.checked_add(sub_time_total).unwrap();
    }

    println!(
        "\nTotal time: {}.{:03}s\n",
        total_time.as_secs(),
        total_time.subsec_millis(),
    );
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

fn format_time(time: &Duration) -> String {
    match time.as_nanos() {
        0..=999 => format!("{}ns", time.as_nanos()),
        1_000..=999_999 => format!("{}\u{b5}s", time.as_nanos() as f32 / 1000f32),
        1_000_000..=999_999_999 => format!("{}ms", time.as_micros() as f32 / 1000f32),
        _ => format!("{}.{:03}s", time.as_secs(), time.subsec_millis()),
    }
}
