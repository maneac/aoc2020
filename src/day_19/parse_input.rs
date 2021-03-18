use std::collections::HashMap;

use super::rule::{Rule, AB};
use super::solution::Container;

impl Container {
    pub(super) fn process_input(&mut self, input: &str) -> Result<(), String> {
        let input_parts: Vec<&str> = input.splitn(2, "\n\n").collect();

        if input_parts.len().ne(&2) {
            return Err("invalid number of parts".to_owned());
        }

        self.process_rules(input_parts[0])?;

        self.process_messages(input_parts[1])?;

        Ok(())
    }

    fn process_rules(&mut self, rules: &str) -> Result<(), String> {
        let mut rule_map: HashMap<usize, Rule> = HashMap::default();

        for rule in rules.lines() {
            let rule_parts = rule.splitn(2, ": ").collect::<Vec<&str>>();

            if rule_parts.len().ne(&2) {
                return Err("invalid number of rule parts".to_owned());
            }

            let rule_label = rule_parts[0]
                .parse::<usize>()
                .map_err(|e| format!("unable to parse rule label as usize: {}", e))?;

            let raw_rule_contents = rule_parts[1];

            let rule_contents = if raw_rule_contents.eq("\"a\"") {
                Rule::AB(AB::A)
            } else if raw_rule_contents.eq("\"b\"") {
                Rule::AB(AB::B)
            } else if raw_rule_contents.contains(" | ") {
                let lr_rule_parts = raw_rule_contents.splitn(2, " | ").collect::<Vec<&str>>();

                Rule::Or(
                    parse_rule_nums(lr_rule_parts[0])?,
                    parse_rule_nums(lr_rule_parts[1])?,
                )
            } else {
                Rule::Single(parse_rule_nums(raw_rule_contents)?)
            };

            if let Some(old_value) = rule_map.insert(rule_label, rule_contents) {
                return Err(format!(
                    "value already existed for {} when parsing rules (old value: {:?})",
                    rule_label, old_value
                ));
            }
        }

        let largest_key = rule_map
            .keys()
            .max()
            .ok_or_else(|| "failed to get largest key from rule map".to_owned())?;

        self.rules.get_mut().resize(*largest_key + 1, Rule::None);

        for (k, v) in rule_map {
            self.rules.get_mut()[k] = v;
        }

        Ok(())
    }

    fn process_messages(&mut self, messages: &str) -> Result<(), String> {
        for message in messages.lines() {
            let parsed_message = message
                .chars()
                .try_fold::<Vec<AB>, _, Result<Vec<AB>, String>>(vec![], |mut acc, c| {
                    let cast_c = match c {
                        'a' => AB::A,
                        'b' => AB::B,
                        _ => return Err(format!("invalid message character: {}", c)),
                    };
                    acc.push(cast_c);
                    Ok(acc)
                })?;

            if parsed_message.len().gt(&self.longest_message_length) {
                self.longest_message_length = parsed_message.len();
            }

            self.messages.push(parsed_message);
        }

        Ok(())
    }
}

fn parse_rule_nums(rule_nums: &str) -> Result<Vec<usize>, String> {
    rule_nums
        .split(" ")
        .try_fold::<Vec<usize>, _, Result<Vec<usize>, String>>(vec![], |mut acc, rule_idx| {
            let rule_idx_num = rule_idx
                .parse::<usize>()
                .map_err(|e| format!("failed to parse rule index as usize: {}", e))?;
            acc.push(rule_idx_num);
            Ok(acc)
        })
}

#[cfg(test)]
mod process_rules {

    use super::*;

    #[test]
    fn detect_a() {
        let input = "0: \"a\"";

        let mut c = Container::new();

        assert_eq!(c.process_rules(input), Ok(()));

        assert_eq!(*c.rules.borrow(), vec![Rule::AB(AB::A)]);
    }

    #[test]
    fn detect_b() {
        let input = "0: \"b\"";

        let mut c = Container::new();

        assert_eq!(c.process_rules(input), Ok(()));

        assert_eq!(*c.rules.borrow(), vec![Rule::AB(AB::B)]);
    }

    #[test]
    fn parse_single() {
        let input = "0: 2 1";

        let mut c = Container::new();

        assert_eq!(c.process_rules(input), Ok(()));

        assert_eq!(*c.rules.borrow(), vec![Rule::Single(vec![2, 1])]);
    }

    #[test]
    fn parse_or() {
        let input = "0: 2 1 | 1 2";

        let mut c = Container::new();

        assert_eq!(c.process_rules(input), Ok(()));

        assert_eq!(*c.rules.borrow(), vec![Rule::Or(vec![2, 1], vec![1, 2])]);
    }

    #[test]
    fn multiple_rules() {
        let input = "0: \"b\"
1: \"a\"
2: 1 2
3: 1 2 | 2 1";

        let mut c = Container::new();

        assert_eq!(c.process_rules(input), Ok(()));

        assert_eq!(
            *c.rules.borrow(),
            vec![
                Rule::AB(AB::B),
                Rule::AB(AB::A),
                Rule::Single(vec![1, 2]),
                Rule::Or(vec![1, 2], vec![2, 1])
            ]
        );
    }

    #[test]
    fn sets_correct_rule_length() {
        let input = "3: \"b\"
4: \"a\"";

        let mut c = Container::new();

        assert_eq!(c.process_rules(input), Ok(()));

        assert_eq!(
            *c.rules.borrow(),
            vec![
                Rule::None,
                Rule::None,
                Rule::None,
                Rule::AB(AB::B),
                Rule::AB(AB::A)
            ]
        );
    }
}

#[cfg(test)]
mod process_messages {

    use super::*;

    #[test]
    fn invalid_character() {
        let input = "abc";

        let mut c = Container::new();

        let res = c.process_messages(input);

        assert_eq!(res.is_err(), true);

        assert_eq!(res.unwrap_err().contains("invalid message character"), true);
    }

    #[test]
    fn single_message() {
        let input = "abba";

        let mut c = Container::new();

        assert_eq!(c.process_messages(input), Ok(()));

        assert_eq!(c.messages, vec![vec![AB::A, AB::B, AB::B, AB::A]]);
    }

    #[test]
    fn multiple_messages() {
        let input = "abba
ba";

        let mut c = Container::new();

        assert_eq!(c.process_messages(input), Ok(()));

        assert_eq!(
            c.messages,
            vec![vec![AB::A, AB::B, AB::B, AB::A], vec![AB::B, AB::A]]
        );
    }
}
