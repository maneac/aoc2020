use std::cell::RefCell;

use super::rule::{Rule, AB};
use crate::Day;

#[derive(Debug)]
pub struct Container {
    pub(super) rules: RefCell<Vec<Rule>>,
    pub(super) messages: Vec<Vec<AB>>,
    pub(super) longest_message_length: usize,
}

impl Container {
    pub fn new() -> Self {
        Self {
            rules: RefCell::new(Vec::new()),
            messages: Vec::new(),
            longest_message_length: 0,
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        self.process_input(input)
    }

    fn part_1(&self) -> Result<String, String> {
        let valid_patterns = self.get_valid_patterns_for_rule(0)?;

        let valid_count = self
            .messages
            .iter()
            .filter(|message| valid_patterns.contains(message))
            .count();

        Ok(valid_count.to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        self.rules.borrow_mut()[8] = Rule::Or(vec![42], vec![42, 8]);
        self.rules.borrow_mut()[11] = Rule::Or(vec![42, 31], vec![42, 11, 31]);

        self.part_1()
    }
}

#[cfg(test)]
mod examples {

    use super::*;

    #[test]
    fn part_two() {
        let input = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

        let mut c = Container::new();

        assert_eq!(c.parse_input(input), Ok(()));

        assert_eq!(c.part_1(), Ok(3.to_string()));

        assert_eq!(c.part_2(), Ok(12.to_string()));
    }
}
