/* Source: https://adventofcode.com/2020/day/2
--- Day 2: Password Philosophy ---

Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc

Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

--- Part One ---

How many passwords are valid according to their policies?

--- Part Two ---

While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.

The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.

Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second
character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these
positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.

Given the same example list from above:

    1-3 a: abcde is valid: position 1 contains a and position 3 does not.
    1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
    2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.

How many passwords are valid according to the new interpretation of the policies?


*/

use std::{fs::read_to_string, path::Path};

pub fn run() -> crate::DayResponse {
    let input_string = read_to_string(Path::new("./data/day_2.txt"))?;

    let input = parse_input(&input_string);

    let part1 = part_1(&input);

    let part2 = part_2(&input);

    Ok((part1.to_string(), part2.to_string()))
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Entry {
    min: u32,
    max: u32,
    target: char,
    password: String,
}

fn parse_input(input: &str) -> Vec<Entry> {
    let mut out = Vec::<Entry>::new();
    for line in input.lines() {
        let mut stage = 0;
        out.push(line.trim().chars().fold(
            Entry {
                min: 0,
                max: 0,
                target: ' ',
                password: String::new(),
            },
            |mut entry, chr| {
                match stage {
                    0 => {
                        if chr == '-' {
                            stage += 1;
                            return entry;
                        }
                        entry.min = (entry.min * 10)
                            + chr.to_digit(10).expect("Unable to parse minimum as uint")
                    }
                    1 => {
                        if chr == ' ' {
                            stage += 1;
                            return entry;
                        }
                        entry.max = (entry.max * 10)
                            + chr.to_digit(10).expect("Unable to parse maximum as uint")
                    }
                    2 => {
                        if chr == ' ' {
                            stage += 1;
                            return entry;
                        }
                        if entry.target == ' ' {
                            entry.target = chr;
                        }
                    }
                    _ => entry.password.push(chr),
                }
                entry
            },
        ));
    }
    out
}

fn part_1(input: &[Entry]) -> usize {
    return input
        .iter()
        .filter(|entry| {
            let ct = entry
                .password
                .chars()
                .filter(|chr| chr.eq(&entry.target))
                .count() as u32;
            entry.min <= ct && ct <= entry.max
        })
        .count();
}

fn part_2(input: &[Entry]) -> usize {
    return input
        .iter()
        .filter(|entry| {
            return (entry
                .password
                .chars()
                .nth(entry.min as usize - 1)
                .expect("Invalid minimum bound for entry")
                .eq(&entry.target))
                != (entry
                    .password
                    .chars()
                    .nth(entry.max as usize - 1)
                    .expect("Invalid minimum bound for entry")
                    .eq(&entry.target));
        })
        .count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc
            1-10 a: aa
            11-1 b: aa
            10-11 c: ab";
        let expected = vec![
            Entry {
                min: 1,
                max: 3,
                target: 'a',
                password: String::from("abcde"),
            },
            Entry {
                min: 1,
                max: 3,
                target: 'b',
                password: String::from("cdefg"),
            },
            Entry {
                min: 2,
                max: 9,
                target: 'c',
                password: String::from("ccccccccc"),
            },
            Entry {
                min: 1,
                max: 10,
                target: 'a',
                password: String::from("aa"),
            },
            Entry {
                min: 11,
                max: 1,
                target: 'b',
                password: String::from("aa"),
            },
            Entry {
                min: 10,
                max: 11,
                target: 'c',
                password: String::from("ab"),
            },
        ];

        assert_eq!(expected, parse_input(&input));
    }

    #[test]
    fn test_part1_example() {
        let input = vec![
            Entry {
                min: 1,
                max: 3,
                target: 'a',
                password: String::from("abcde"),
            },
            Entry {
                min: 1,
                max: 3,
                target: 'b',
                password: String::from("cdefg"),
            },
            Entry {
                min: 2,
                max: 9,
                target: 'c',
                password: String::from("ccccccccc"),
            },
        ];
        let expected = 2;

        assert_eq!(expected, part_1(&input));
    }

    #[test]
    fn test_part2_example() {
        let input = vec![
            Entry {
                min: 1,
                max: 3,
                target: 'a',
                password: String::from("abcde"),
            },
            Entry {
                min: 1,
                max: 3,
                target: 'b',
                password: String::from("cdefg"),
            },
            Entry {
                min: 2,
                max: 9,
                target: 'c',
                password: String::from("ccccccccc"),
            },
        ];
        let expected = 1;

        assert_eq!(expected, part_2(&input));
    }
}
