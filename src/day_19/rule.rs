#[derive(Debug, Clone, PartialEq)]
pub(super) enum Rule {
    None,
    AB(AB),
    Single(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub(super) enum AB {
    A,
    B,
}

use super::solution::Container;

impl Container {
    pub(super) fn get_valid_patterns_for_rule(
        &self,
        rule_idx: usize,
    ) -> Result<Vec<Vec<AB>>, String> {
        let mut possible_patterns = vec![vec![]];

        self.append_to_patterns(&mut possible_patterns, rule_idx)?;

        Ok(possible_patterns)
    }

    fn append_to_patterns<'s>(
        &'s self,
        existing_patterns: &'s mut Vec<Vec<AB>>,
        rule_idx: usize,
    ) -> Result<(), String> {
        let target_rule = &self.rules.borrow()[rule_idx];

        if let Rule::AB(letter) = target_rule {
            existing_patterns.iter_mut().for_each(|p| {
                p.push(*letter);
            });
            return Ok(());
        }

        if let Rule::Single(ruleset) = target_rule {
            for ruleset_idx in ruleset {
                self.append_to_patterns(existing_patterns, *ruleset_idx)?;
            }
            return Ok(());
        }

        if let Rule::Or(left_ruleset, right_ruleset) = target_rule {
            let largest_pattern = existing_patterns
                .iter()
                .map(|p| p.len())
                .max()
                .ok_or_else(|| "unable to get longest pattern".to_owned())?;

            if largest_pattern < self.longest_message_length {
                let mut right_patterns = existing_patterns.clone();

                for left_idx in left_ruleset {
                    self.append_to_patterns(existing_patterns, *left_idx)?;
                }

                for right_idx in right_ruleset {
                    self.append_to_patterns(&mut right_patterns, *right_idx)?;
                }
                existing_patterns.extend_from_slice(&right_patterns);
            }

            return Ok(());
        }

        Err(format!("unsupported rule type: {:?}", target_rule))
    }

    pub(super) fn is_message_valid(&self, message: &str) -> bool {
        false
    }
}

#[cfg(test)]
mod valid_patterns {

    use std::cell::RefCell;

    use super::*;

    #[test]
    fn single_letter() {
        let c = Container {
            rules: RefCell::new(vec![Rule::AB(AB::A)]),
            messages: Vec::default(),
            longest_message_length: 100,
        };

        let res = c.get_valid_patterns_for_rule(0);

        assert!(res.is_ok());

        assert_eq!(res.unwrap(), vec![vec![AB::A]])
    }

    #[test]
    fn selects_correct_index() {
        let c = Container {
            rules: RefCell::new(vec![Rule::AB(AB::A), Rule::AB(AB::B)]),
            messages: Vec::default(),
            longest_message_length: 100,
        };

        let res = c.get_valid_patterns_for_rule(1);

        assert!(res.is_ok());

        assert_eq!(res.unwrap(), vec![vec![AB::B]])
    }

    #[test]
    fn single_one_level() {
        let c = Container {
            rules: RefCell::new(vec![
                Rule::Single(vec![2, 1]),
                Rule::AB(AB::A),
                Rule::AB(AB::B),
            ]),
            messages: Vec::default(),
            longest_message_length: 100,
        };

        let res = c.get_valid_patterns_for_rule(0);

        assert!(res.is_ok());

        assert_eq!(res.unwrap(), vec![vec![AB::B, AB::A]])
    }

    #[test]
    fn single_recursive() {
        let c = Container {
            rules: RefCell::new(vec![
                Rule::Single(vec![2, 1]),
                Rule::Single(vec![2, 3]),
                Rule::AB(AB::A),
                Rule::AB(AB::B),
            ]),
            messages: Vec::default(),
            longest_message_length: 100,
        };

        let res = c.get_valid_patterns_for_rule(0);

        assert!(res.is_ok());

        assert_eq!(res.unwrap(), vec![vec![AB::A, AB::A, AB::B]])
    }

    #[test]
    fn simple_or() {
        let c = Container {
            rules: RefCell::new(vec![
                Rule::Or(vec![2, 1], vec![1, 2]),
                Rule::AB(AB::A),
                Rule::AB(AB::B),
            ]),
            messages: Vec::default(),
            longest_message_length: 100,
        };

        let res = c.get_valid_patterns_for_rule(0);

        assert!(res.is_ok());

        assert_eq!(res.unwrap(), vec![vec![AB::B, AB::A], vec![AB::A, AB::B]])
    }

    #[test]
    fn runaway_recursion_1() {
        let c = Container {
            rules: RefCell::new(vec![Rule::Or(vec![1], vec![1, 0]), Rule::AB(AB::A)]),
            messages: Vec::default(),
            longest_message_length: 10,
        };

        let res = c.get_valid_patterns_for_rule(0);

        assert!(res.is_ok());

        assert_eq!(res.unwrap(), vec![vec![AB::A]])
    }
}
