use crate::Day;
use std::cell::RefCell;

pub struct Container {
    preamble_len: usize,
    input: Vec<usize>,
    prev_num: RefCell<usize>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            preamble_len: 25,
            input: Vec::new(),
            prev_num: RefCell::new(0),
        }
    }

    // returns the index of the first matching pair, or none if no match
    fn check_previous_preamble(&self, idx: usize) -> Option<(usize, usize)> {
        let target = self.input[idx];
        for i in (idx - self.preamble_len)..idx {
            let outer = self.input[i];
            for j in (i + 1)..idx {
                if self.input[j] + outer == target {
                    return Some((i, j));
                }
            }
        }
        self.prev_num.replace(target);
        None
    }
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        input.trim().lines().try_for_each(|line| {
            let num = line.parse::<usize>();
            match num {
                Ok(i) => {
                    self.input.push(i);
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            }
        })
    }

    fn part_1(&self) -> Result<String, String> {
        for idx in self.preamble_len..self.input.len() {
            if self.check_previous_preamble(idx).is_none() {
                return Ok(self.input[idx].to_string());
            }
        }
        Err("no invalid entry found".to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        let target = *self.prev_num.borrow();
        for idx in 0..self.input.len() {
            let attempt = self.input.iter().skip(idx).try_fold(vec![], |mut acc, i| {
                acc.push(*i);
                let sum = acc.iter().sum::<usize>();
                match sum.cmp(&target) {
                    std::cmp::Ordering::Less => Ok(acc),
                    std::cmp::Ordering::Equal => Err(Some(acc)),
                    std::cmp::Ordering::Greater => Err(None),
                }
            });
            if let Err(Some(res)) = attempt {
                return Ok(
                    (res.iter().min().unwrap_or(&0) + res.iter().max().unwrap_or(&0)).to_string(),
                );
            }
        }
        Err("no sequence found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        let expected = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let mut cont = Container::new();
        assert_eq!(Ok(()), cont.parse_input(&input));
        assert_eq!(expected, cont.input);
    }

    #[test]
    fn test_part_1_example() {
        let input = Container {
            preamble_len: 5,
            input: vec![
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576,
            ],
            prev_num: RefCell::new(0),
        };

        let expected = 127;

        assert_eq!(Ok(expected.to_string()), input.part_1());
        assert_eq!(expected, *input.prev_num.borrow());
    }

    #[test]
    fn test_part_2_example() {
        let input = Container {
            preamble_len: 5,
            input: vec![
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576,
            ],
            prev_num: RefCell::new(127),
        };

        let expected = 62.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }

    #[test]
    fn test_check_preamble() {
        let inputs = vec![
            (
                Container {
                    preamble_len: 3,
                    input: vec![1, 2, 6, 4, 5],
                    prev_num: RefCell::new(0),
                },
                3,
                None,
            ),
            (
                Container {
                    preamble_len: 3,
                    input: vec![1, 2, 3, 4, 5],
                    prev_num: RefCell::new(0),
                },
                4,
                Some((1, 2)),
            ),
        ];

        inputs
            .iter()
            .enumerate()
            .for_each(|(entry_idx, (container, idx, expected))| {
                if let Some((a, b)) = container.check_previous_preamble(*idx) {
                    assert!(expected.is_some(), "Input {}: {:?}", entry_idx, (a, b));
                    if let Some((e_a, e_b)) = expected {
                        assert_eq!((a, b), (*e_a, *e_b));
                    }
                } else {
                    assert!(expected.is_none(), "Input {}", entry_idx);
                }
            });
    }
}
