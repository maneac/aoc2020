use crate::Day;

pub struct Container {
    input: Vec<Group>,
}

impl Container {
    pub fn new() -> Self {
        Self { input: Vec::new() }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Group {
    or: u32,
    and: u32,
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        self.input = input
            .trim()
            .split("\n\n")
            .map(|group| {
                group
                    .trim()
                    .lines()
                    .fold(Group { or: 0, and: !0 }, |mut acc, line| {
                        let entry = line.chars().fold(0u32, |mut acc, chr| {
                            acc |= 1 << (chr as u8 - b'a');
                            acc
                        });
                        acc.or |= entry;
                        acc.and &= entry;
                        acc
                    })
            })
            .collect();
        Ok(())
    }

    fn part_1(&self) -> Result<String, String> {
        Ok(self
            .input
            .iter()
            .fold(0u32, |mut acc, group| {
                acc += group.or.count_ones();
                acc
            })
            .to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        Ok(self
            .input
            .iter()
            .fold(0u32, |mut acc, group| {
                acc += group.and.count_ones();
                acc
            })
            .to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let expected = vec![
            Group {
                or: 1 << 0 | 1 << 1 | 1 << 2,
                and: 1 << 0 | 1 << 1 | 1 << 2,
            },
            Group {
                or: 1 << 0 | 1 << 1 | 1 << 2,
                and: 0,
            },
            Group {
                or: 1 << 0 | 1 << 1 | 1 << 2,
                and: 1 << 0,
            },
            Group {
                or: 1 << 0,
                and: 1 << 0,
            },
            Group {
                or: 1 << 1,
                and: 1 << 1,
            },
        ];

        let mut cont = Container::new();
        assert_eq!(Ok(()), cont.parse_input(input));
        assert_eq!(expected, cont.input);
    }

    #[test]
    fn test_part_1_example() {
        let input = Container {
            input: vec![
                Group {
                    or: 1 << 0 | 1 << 1 | 1 << 2,
                    and: 1 << 0 | 1 << 1 | 1 << 2,
                },
                Group {
                    or: 1 << 0 | 1 << 1 | 1 << 2,
                    and: 0,
                },
                Group {
                    or: 1 << 0 | 1 << 1 | 1 << 2,
                    and: 1 << 0,
                },
                Group {
                    or: 1 << 0,
                    and: 1 << 0,
                },
                Group {
                    or: 1 << 1,
                    and: 1 << 1,
                },
            ],
        };

        let expected = 11.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_2_example() {
        let input = Container {
            input: vec![
                Group {
                    or: 1 << 0 | 1 << 1 | 1 << 2,
                    and: 1 << 0 | 1 << 1 | 1 << 2,
                },
                Group {
                    or: 1 << 0 | 1 << 1 | 1 << 2,
                    and: 0,
                },
                Group {
                    or: 1 << 0 | 1 << 1 | 1 << 2,
                    and: 1 << 0,
                },
                Group {
                    or: 1 << 0,
                    and: 1 << 0,
                },
                Group {
                    or: 1 << 1,
                    and: 1 << 1,
                },
            ],
        };

        let expected = 6.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }
}
