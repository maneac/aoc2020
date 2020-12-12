use crate::Day;
use std::collections::HashMap;

pub struct Container {
    input: Vec<usize>,
}

impl Container {
    pub fn new() -> Self {
        Self { input: Vec::new() }
    }
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        if let Err(e) = input.trim().lines().try_for_each(|line| {
            let num = line.parse::<usize>();
            match num {
                Ok(i) => {
                    self.input.push(i);
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            }
        }) {
            Err(e)
        } else {
            self.input.sort_unstable();
            Ok(())
        }
    }

    fn part_1(&self) -> Result<String, String> {
        match self
            .input
            .iter()
            // final hop to computer will always be 3
            .try_fold((0usize, 0usize, 1usize), |mut acc, entry| {
                match entry - acc.0 {
                    1 => {
                        acc.1 += 1;
                    }
                    2 => {}
                    3 => {
                        acc.2 += 1;
                    }
                    _ => return Err("invalid spacing between adapters".to_string()),
                }
                acc.0 = *entry;
                Ok(acc)
            }) {
            Ok(out) => Ok((out.1 * out.2).to_string()),
            Err(e) => Err(e),
        }
    }

    fn part_2(&self) -> Result<String, String> {
        match adapter_arrangements(&self.input) {
            Ok(out) => Ok(out.to_string()),
            Err(e) => Err(e),
        }
    }
}

fn adapter_arrangements(input: &[usize]) -> Result<usize, String> {
    if input.is_empty() {
        return Err("empty input".to_owned());
    }
    let mut working_set = HashMap::new();
    let max = input.iter().max().unwrap();
    working_set.insert(max, 1);

    for idx in (0..input.len() as isize).rev() {
        let target = &input[idx as usize];
        let count = *working_set.get(target).unwrap_or(&0);
        for i in (idx - 3..idx).rev() {
            if i < 0 {
                continue;
            }
            let comparison = &input[i as usize];
            if target - comparison < 4 {
                working_set
                    .entry(comparison)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
            }
        }
    }

    let ones = *working_set.get(&1).unwrap_or(&0);
    let twos = *working_set.get(&2).unwrap_or(&0);
    let threes = *working_set.get(&3).unwrap_or(&0);

    Ok(ones + twos + threes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";

        let expected = vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];

        let mut cont = Container::new();

        assert_eq!(Ok(()), cont.parse_input(input));
        assert_eq!(expected, cont.input);
    }

    #[test]
    fn test_part_1_example_1() {
        let input = Container {
            input: vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19],
        };

        let expected = 35.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_1_example_2() {
        let input = Container {
            input: vec![
                1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34,
                35, 38, 39, 42, 45, 46, 47, 48, 49,
            ],
        };

        let expected = 220.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_2_example_1() {
        let input = Container {
            input: vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19],
        };

        let expected = 8.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }

    #[test]
    fn test_part_2_example_2() {
        let input = Container {
            input: vec![
                1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34,
                35, 38, 39, 42, 45, 46, 47, 48, 49,
            ],
        };

        let expected = 19208.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }

    #[test]
    fn test_adapter_arrangements() {
        let tests = vec![
            ("one valid entry", vec![3], Ok(1)),
            ("four valid", vec![1, 2, 3, 4], Ok(7)),
            ("larger span", vec![1, 2, 4, 7, 8, 10], Ok(6)),
        ];

        for test in tests.iter() {
            let output = adapter_arrangements(&test.1);
            assert_eq!(test.2, output, "\n  case: {}", test.0);
        }
    }
}
