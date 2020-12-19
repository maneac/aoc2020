use crate::Day;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Container {
    input: Vec<String>,
}

impl Container {
    pub fn new() -> Self {
        Self { input: Vec::new() }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        input.trim().lines().try_for_each(|line| {
            self.input.push(line.trim().to_owned());
            Ok(())
        })
    }

    fn part_1(&self) -> Result<String, String> {
        Ok(self
            .input
            .iter()
            .fold(0usize, |mut acc, line| {
                acc += evaluate(line);
                acc
            })
            .to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        let mut inputs = self.input.clone();

        for line in inputs.iter_mut() {
            let mut idx = 0;
            loop {
                if line.chars().nth(idx).unwrap() == '*' {
                    let mut level = 0;
                    let (last_idx, _) = line
                        .char_indices()
                        .rev()
                        .skip(line.len() - idx)
                        .find(|&(_, c)| {
                            if c == ')' {
                                level += 1;
                            }
                            if c == '(' {
                                level -= 1;
                                if level < 0 {
                                    return true;
                                }
                            }
                            false
                        })
                        .unwrap_or_default();
                    line.insert(last_idx, '(');
                    line.insert(idx, ')');
                    idx += 4;

                    level = 0;
                    let (last_idx, _) = line
                        .char_indices()
                        .skip(idx)
                        .find(|&(_, c)| {
                            if c == '(' {
                                level += 1;
                            }
                            if c == ')' {
                                level -= 1;
                                if level < 0 {
                                    return true;
                                }
                            }
                            false
                        })
                        .unwrap_or((line.len(), ' '));
                    line.insert(last_idx, ')');
                    line.insert(idx, '(');
                }
                idx += 1;
                if idx >= line.len() {
                    break;
                }
            }
        }

        Ok(inputs
            .iter()
            .fold(0usize, |mut acc, line| {
                acc += evaluate(line);
                acc
            })
            .to_string())
    }
}

fn evaluate(input: &str) -> usize {
    let mut output: Option<usize> = None;

    let mut mult = false;
    let mut idx = 0;
    while idx < input.len() {
        let chr = input.chars().nth(idx).unwrap();
        match chr {
            ' ' => {}
            '0'..='9' => {
                let num = chr.to_digit(10).unwrap() as usize;
                match output {
                    Some(v) => {
                        if mult {
                            output = Some(v * num);
                        } else {
                            output = Some(v + num);
                        }
                    }
                    None => output = Some(num),
                }
            }
            '+' => {
                mult = false;
            }
            '*' => {
                mult = true;
            }
            '(' => {
                let mut level = 0;
                let (last_idx, _) = input
                    .char_indices()
                    .skip(idx)
                    .find(|&(_, c)| {
                        if c == '(' {
                            level += 1;
                        }
                        if c == ')' {
                            level -= 1;
                            if level <= 0 {
                                return true;
                            }
                        }
                        false
                    })
                    .unwrap();

                let num = evaluate(
                    &input
                        .char_indices()
                        .filter(|&(i, _)| i > idx && i < last_idx)
                        .map(|(_, c)| c)
                        .fold(String::new(), |mut acc, chr| {
                            acc.push(chr);
                            acc
                        }),
                );
                match output {
                    Some(v) => {
                        if mult {
                            output = Some(v * num);
                        } else {
                            output = Some(v + num);
                        }
                    }
                    None => output = Some(num),
                };
                idx = last_idx;
            }
            _ => {}
        };
        idx += 1;
    }
    output.unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1 + 2 * 3 + 4 * 5 + 6
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";

        let expected = Container {
            input: vec![
                "1 + 2 * 3 + 4 * 5 + 6".to_owned(),
                "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_owned(),
            ],
        };

        let mut cont = Container::new();

        assert_eq!(Ok(()), cont.parse_input(input));
        assert_eq!(expected, cont);
    }

    #[test]
    fn test_part_1_example_1() {
        let input = Container {
            input: vec!["1 + 2 * 3 + 4 * 5 + 6".to_owned()],
        };

        let expected = 71.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_1_example_2() {
        let input = Container {
            input: vec!["1 + (2 * 3) + (4 * (5 + 6))".to_owned()],
        };

        let expected = 51.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_1_example_bullet_1() {
        let input = Container {
            input: vec!["2 * 3 + (4 * 5)".to_owned()],
        };

        let expected = 26.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_1_example_bullet_2() {
        let input = Container {
            input: vec!["5 + (8 * 3 + 9 + 3 * 4 * 3)".to_owned()],
        };

        let expected = 437.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_1_example_bullet_3() {
        let input = Container {
            input: vec!["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_owned()],
        };

        let expected = 12240.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_1_example_bullet_4() {
        let input = Container {
            input: vec!["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_owned()],
        };

        let expected = 13632.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_2_example_1() {
        let input = Container {
            input: vec!["1 + 2 * 3 + 4 * 5 + 6".to_owned()],
        };

        let expected = 231.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }

    #[test]
    fn test_part_2_example_bullet_1() {
        let input = Container {
            input: vec!["1 + (2 * 3) + (4 * (5 + 6))".to_owned()],
        };

        let expected = 51.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }

    #[test]
    fn test_part_2_example_bullet_2() {
        let input = Container {
            input: vec!["2 * 3 + (4 * 5)".to_owned()],
        };

        let expected = 46.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }

    #[test]
    fn test_part_2_example_bullet_3() {
        let input = Container {
            input: vec!["5 + (8 * 3 + 9 + 3 * 4 * 3)".to_owned()],
        };

        let expected = 1445.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }

    #[test]
    fn test_part_2_example_bullet_4() {
        let input = Container {
            input: vec!["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_owned()],
        };

        let expected = 669060.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }

    #[test]
    fn test_part_2_example_bullet_5() {
        let input = Container {
            input: vec!["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_owned()],
        };

        let expected = 23340.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }
}
