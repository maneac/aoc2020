use crate::Day;

pub struct Container {
    input: Vec<Entry>,
}

impl Container {
    pub fn new() -> Self {
        Self { input: Vec::new() }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Entry {
    min: u32,
    max: u32,
    target: char,
    password: String,
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
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
        self.input = out;
        Ok(())
    }

    fn part_1(&self) -> Result<String, String> {
        Ok(self
            .input
            .iter()
            .filter(|entry| {
                let ct = entry
                    .password
                    .chars()
                    .filter(|chr| chr.eq(&entry.target))
                    .count() as u32;
                entry.min <= ct && ct <= entry.max
            })
            .count()
            .to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        Ok(self
            .input
            .iter()
            .filter(|entry| {
                (entry
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
                        .eq(&entry.target))
            })
            .count()
            .to_string())
    }
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

        let mut cont = Container::new();
        assert_eq!(Ok(()), cont.parse_input(&input));
        assert_eq!(expected, cont.input);
    }

    #[test]
    fn test_part_1_example() {
        let input = Container {
            input: vec![
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
            ],
        };

        let expected = 2.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_2_example() {
        let input = Container {
            input: vec![
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
            ],
        };

        let expected = 1.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }
}
