use crate::Day;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Container {
    pub(super) rules: Vec<(String, [u128; 10])>,
    pub(super) our_ticket: Vec<u16>,
    pub(super) nearby_tickets: Vec<Vec<u16>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            rules: Vec::default(),
            our_ticket: Vec::default(),
            nearby_tickets: Vec::default(),
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
        let permissive_ruleset = self.rules.iter().fold(vec![0u128; 10], |mut acc, rule| {
            for (idx, ruleset) in acc.iter_mut().enumerate().take(rule.1.len()) {
                *ruleset |= rule.1[idx];
            }
            acc
        });

        let error_rate = self.nearby_tickets.iter().fold(0u16, |mut acc, ticket| {
            for &ticket_number in ticket {
                if permissive_ruleset[ticket_number as usize / 100] & 1 << (ticket_number % 100)
                    == 0
                {
                    acc += ticket_number;
                }
            }
            acc
        });

        Ok(error_rate.to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        let field_indices = self.assign_rules_to_fields()?;

        let mut departure_total: usize = 1;

        for (idx, &rule_idx) in field_indices.iter().enumerate() {
            if self.rules[rule_idx].0.starts_with("departure") {
                departure_total *= self.our_ticket[idx] as usize;
            }
        }

        Ok(departure_total.to_string())
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn part_one() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        let mut c = Container::new();

        assert_eq!(c.parse_input(input), Ok(()));

        assert_eq!(c.part_1(), Ok(71.to_string()));
    }

    #[test]
    fn part_two() {
        let input = "class: 0-1 or 4-19
departure row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

        let mut c = Container::new();

        assert_eq!(c.parse_input(input), Ok(()));

        assert_eq!(c.part_2(), Ok(11.to_string()));
    }
}
