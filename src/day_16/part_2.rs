use super::solution::Container;

impl Container {
    pub(super) fn assign_rules_to_fields(&self) -> Result<Vec<usize>, String> {
        // Remove all erroneous tickets
        let permissive_ruleset = self.rules.iter().fold(vec![0u128; 10], |mut acc, rule| {
            for (idx, ruleset) in acc.iter_mut().enumerate().take(rule.1.len()) {
                *ruleset |= rule.1[idx];
            }
            acc
        });

        let valid_nearby_tickets: Vec<&Vec<u16>> = self
            .nearby_tickets
            .iter()
            .filter(|&ticket| {
                for &ticket_number in ticket {
                    if permissive_ruleset[ticket_number as usize / 100] & 1 << (ticket_number % 100)
                        == 0
                    {
                        return false;
                    }
                }
                true
            })
            .collect();

        let mut possibilities = vec![(1 << self.rules.len()) - 1_u128; self.rules.len()];

        // Find possible mappings for each valid ticket field
        for ticket in valid_nearby_tickets {
            for ticket_idx in 0..ticket.len() {
                let ticket_number = ticket[ticket_idx] as usize;
                for idx in 0..self.rules.len() {
                    if possibilities[ticket_idx] & 1 << idx == 0 {
                        continue;
                    }
                    if self.rules[idx].1[ticket_number / 100] & 1 << (ticket_number % 100) == 0 {
                        possibilities[ticket_idx] &= !(1 << idx);
                    }
                }
            }
        }

        // Assign each field deterministically
        let mut resulting_indices = vec![0usize; possibilities.len()];
        let mut last_sum = 0;
        loop {
            for idx in 0..possibilities.len() {
                let possibility = possibilities[idx];

                if possibility.count_ones() != 1 {
                    continue;
                }

                resulting_indices[idx] = (possibility as f64).log2() as usize;
                possibilities[idx] = 0;

                possibilities.iter_mut().for_each(|p| *p &= !(possibility));
            }

            let total = possibilities.iter().sum::<u128>();
            if total == last_sum {
                break;
            }

            last_sum = total;
        }

        if last_sum > 0 {
            return Err(format!(
                "not all rules mapped, remaining sum was: {:020b}\nMapped: {:?}",
                last_sum, resulting_indices,
            ));
        }

        Ok(resulting_indices)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn two_fields() {
        let c = Container {
            rules: vec![
                (
                    "rule1".to_owned(),
                    [1 << 20 | 1 << 21, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ),
                (
                    "rule2".to_owned(),
                    [1 << 10 | 1 << 11, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ),
            ],
            our_ticket: Vec::default(),
            nearby_tickets: vec![vec![10, 20]],
        };

        let res = c.assign_rules_to_fields();

        assert_eq!(res, Ok(vec![1, 0]))
    }

    #[test]
    fn mapped_field_is_removed_from_possibilities() {
        let c = Container {
            rules: vec![
                (
                    "rule1".to_owned(),
                    [
                        1 << 9 | 1 << 10 | 1 << 20 | 1 << 21,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ),
                (
                    "rule2".to_owned(),
                    [1 << 10 | 1 << 11, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ),
            ],
            our_ticket: Vec::default(),
            nearby_tickets: vec![vec![10, 20], vec![11, 9]],
        };

        let res = c.assign_rules_to_fields();

        assert_eq!(res, Ok(vec![1, 0]))
    }

    #[test]
    fn ignore_invalid_tickets() {
        let c = Container {
            rules: vec![
                (
                    "rule1".to_owned(),
                    [1 << 20 | 1 << 21, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ),
                (
                    "rule2".to_owned(),
                    [1 << 10 | 1 << 11, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ),
            ],
            our_ticket: Vec::default(),
            nearby_tickets: vec![vec![10, 20], vec![50, 10]],
        };

        let res = c.assign_rules_to_fields();

        assert_eq!(res, Ok(vec![1, 0]))
    }

    #[test]
    fn given_example() {
        let c = Container {
            rules: vec![
                (
                    "class".to_owned(),
                    [
                        1 << 0
                            | 1 << 1
                            | 1 << 4
                            | 1 << 5
                            | 1 << 6
                            | 1 << 7
                            | 1 << 8
                            | 1 << 9
                            | 1 << 10
                            | 1 << 11
                            | 1 << 12
                            | 1 << 13
                            | 1 << 14
                            | 1 << 15
                            | 1 << 16
                            | 1 << 17
                            | 1 << 18
                            | 1 << 19,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ),
                (
                    "row".to_owned(),
                    [
                        1 << 0
                            | 1 << 1
                            | 1 << 2
                            | 1 << 3
                            | 1 << 4
                            | 1 << 5
                            | 1 << 8
                            | 1 << 9
                            | 1 << 10
                            | 1 << 11
                            | 1 << 12
                            | 1 << 13
                            | 1 << 14
                            | 1 << 15
                            | 1 << 16
                            | 1 << 17
                            | 1 << 18
                            | 1 << 19,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ),
                (
                    "seat".to_owned(),
                    [
                        1 << 0
                            | 1 << 1
                            | 1 << 2
                            | 1 << 3
                            | 1 << 4
                            | 1 << 5
                            | 1 << 6
                            | 1 << 7
                            | 1 << 8
                            | 1 << 9
                            | 1 << 10
                            | 1 << 11
                            | 1 << 12
                            | 1 << 13
                            | 1 << 16
                            | 1 << 17
                            | 1 << 18
                            | 1 << 19,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ),
            ],
            our_ticket: vec![11, 12, 13],
            nearby_tickets: vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9]],
        };

        let res = c.assign_rules_to_fields();

        assert_eq!(res, Ok(vec![1, 0, 2]))
    }
}
