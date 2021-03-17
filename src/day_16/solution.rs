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
            for idx in 0..rule.1.len() {
                acc[idx] |= rule.1[idx];
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

        for idx in 0..field_indices.len() {
            if self.rules[field_indices[idx]].0.starts_with("departure") {
                departure_total *= self.our_ticket[idx] as usize;
            }
        }

        Ok(departure_total.to_string())
    }
}
