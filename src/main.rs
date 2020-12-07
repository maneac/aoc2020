#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

use std::{error::Error, time::Instant};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

pub type DayResponse = Result<(String, String), Box<dyn Error>>;

struct Day<'day> {
    run_fn: &'day dyn Fn() -> DayResponse,
    part_1_expected: &'day str,
    part_2_expected: &'day str,
}

fn validate_day(day: usize, part: usize, expected: &str, received: String) {
    if expected != received {
        panic!(
            "Incorrect result for day {}, part {} returned:\nExpected: {}\nReturned: {}",
            day, part, expected, received,
        );
    }
}

fn main() {
    let days: Vec<Day> = vec![
        Day {
            run_fn: &day_1::run,
            part_1_expected: "542619",
            part_2_expected: "32858450",
        },
        Day {
            run_fn: &day_2::run,
            part_1_expected: "424",
            part_2_expected: "747",
        },
        Day {
            run_fn: &day_3::run,
            part_1_expected: "270",
            part_2_expected: "2122848000",
        },
        Day {
            run_fn: &day_4::run,
            part_1_expected: "206",
            part_2_expected: "123",
        },
        Day {
            run_fn: &day_5::run,
            part_1_expected: "838",
            part_2_expected: "714",
        },
        Day {
            run_fn: &day_6::run,
            part_1_expected: "6504",
            part_2_expected: "3351",
        },
        Day {
            run_fn: &day_7::run,
            part_1_expected: "",
            part_2_expected: "",
        },
    ];

    days.iter().enumerate().for_each(|(idx, day)| {
        let day_num = idx + 1;
        print!("Day {}", day_num);

        let start = Instant::now();
        let results = (*day.run_fn)();
        let runtime = start.elapsed();

        println!(" [{}\u{b5}s]", runtime.as_micros());

        match results {
            Err(e) => println!("Failed to run day {}: {}", day_num, e),
            Ok((part1, part2)) => {
                validate_day(day_num, 1, day.part_1_expected, part1);
                validate_day(day_num, 2, day.part_2_expected, part2);
            }
        }
    });
}
