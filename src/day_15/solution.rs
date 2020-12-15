use std::collections::HashMap;

use crate::Day;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Container {
    input: Vec<usize>,
}

impl Container {
    pub fn new() -> Self {
        Self { input: Vec::new() }
    }

    fn speak(&self, limit: usize) -> Result<String, String> {
        let mut working_set = HashMap::new();
        let mut last_spoken = 0;

        self.input.iter().enumerate().for_each(|(idx, value)| {
            working_set.insert(*value, (idx, None));
            last_spoken = *value;
        });

        for i in self.input.len()..limit {
            let last = working_set.get(&last_spoken).unwrap();

            match last.1 {
                Some(last_idx) => {
                    last_spoken = last_idx - last.0;
                }
                None => {
                    last_spoken = 0;
                }
            };

            working_set
                .entry(last_spoken)
                .and_modify(|e| {
                    if let Some(v) = e.1 {
                        e.0 = v;
                    }
                    e.1 = Some(i)
                })
                .or_insert_with(|| (i, None));
        }

        Ok(last_spoken.to_string())
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        input.trim().split(',').try_for_each(|num| {
            self.input.push(
                num.parse::<usize>()
                    .map_err(|e| format!("failed to parse input digit: {}", e))?,
            );
            Ok(())
        })
    }

    fn part_1(&self) -> Result<String, String> {
        self.speak(2020)
    }

    fn part_2(&self) -> Result<String, String> {
        self.speak(30_000_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "0,3,6";

        let expected = Container {
            input: vec![0, 3, 6],
        };

        let mut cont = Container::new();

        assert_eq!(Ok(()), cont.parse_input(input));
        assert_eq!(expected, cont);
    }

    #[test]
    fn test_part_1_examples() {
        let tests = vec![
            (
                "example input",
                Container {
                    input: vec![0, 3, 6],
                },
                436,
            ),
            (
                "bullet 1",
                Container {
                    input: vec![1, 3, 2],
                },
                1,
            ),
            (
                "bullet 2",
                Container {
                    input: vec![2, 1, 3],
                },
                10,
            ),
            (
                "bullet 3",
                Container {
                    input: vec![1, 2, 3],
                },
                27,
            ),
            (
                "bullet 4",
                Container {
                    input: vec![2, 3, 1],
                },
                78,
            ),
            (
                "bullet 5",
                Container {
                    input: vec![3, 2, 1],
                },
                438,
            ),
            (
                "bullet 6",
                Container {
                    input: vec![3, 1, 2],
                },
                1836,
            ),
        ];

        for test in tests.iter() {
            assert_eq!(
                Ok(test.2.to_string()),
                test.1.part_1(),
                "  test: {}",
                test.0
            );
        }
    }

    #[test]
    #[ignore = "takes a long time to run"]
    fn test_part_2_example() {
        let tests = vec![
            (
                "example input",
                Container {
                    input: vec![0, 3, 6],
                },
                175594,
            ),
            (
                "bullet 1",
                Container {
                    input: vec![1, 3, 2],
                },
                2578,
            ),
            (
                "bullet 2",
                Container {
                    input: vec![2, 1, 3],
                },
                3544142,
            ),
            (
                "bullet 3",
                Container {
                    input: vec![1, 2, 3],
                },
                261214,
            ),
            (
                "bullet 4",
                Container {
                    input: vec![2, 3, 1],
                },
                6895259,
            ),
            (
                "bullet 5",
                Container {
                    input: vec![3, 2, 1],
                },
                18,
            ),
            (
                "bullet 6",
                Container {
                    input: vec![3, 1, 2],
                },
                362,
            ),
        ];

        for test in tests.iter() {
            assert_eq!(
                Ok(test.2.to_string()),
                test.1.part_2(),
                "  test: {}",
                test.0
            );
        }
    }
}
