use super::solution::Container;

impl Container {
    pub fn process_input(&mut self, input: &str) -> Result<(), String> {
        let entry_chunks = input.split("\n\n").collect::<Vec<&str>>();

        if entry_chunks.len() != 3 {
            return Err(format!(
                "invalid data format: expected 3 sections, found {}",
                entry_chunks.len()
            ));
        }

        self.parse_rules(entry_chunks[0])?;

        self.parse_our_ticket(entry_chunks[1])?;

        self.parse_nearby_tickets(entry_chunks[2])?;

        Ok(())
    }

    fn parse_rules(&mut self, rules: &str) -> Result<(), String> {
        let label_range_separator = ": ";
        let range_bound_separator = "-";

        for rule in rules.lines() {
            let (label, ranges) = rule.split_at(
                rule.match_indices(label_range_separator)
                    .next()
                    .ok_or_else(|| format!("invalid rule format: {}", &rule))?
                    .0,
            );

            let mut range_mask = [0u128; 10];

            for range in ranges[label_range_separator.len()..].split(" or ") {
                let (lower_bound, upper_bound) = range.split_at(
                    range
                        .match_indices(range_bound_separator)
                        .next()
                        .ok_or_else(|| "invalid rule range".to_string())?
                        .0,
                );

                let lb = lower_bound
                    .parse::<usize>()
                    .map_err(|e| format!("invalid lower bound for range: {}", e))?;

                let ub = upper_bound[range_bound_separator.len()..]
                    .parse::<usize>()
                    .map_err(|e| format!("invalid upper bound for range: {}", e))?;

                for i in lb..=ub {
                    range_mask[i / 100] |= 1 << (i % 100);
                }
            }

            self.rules.push((label.to_owned(), range_mask));
        }
        Ok(())
    }

    fn parse_our_ticket(&mut self, ticket_chunk: &str) -> Result<(), String> {
        let mut ticket_lines = ticket_chunk.lines();

        if ticket_lines.next().unwrap_or_default().ne("your ticket:") {
            return Err("invalid header for our ticket".to_owned());
        }

        let ticket_numbers = ticket_lines
            .next()
            .ok_or_else(|| "invalid format for our ticket".to_string())?;

        self.our_ticket = parse_ticket(ticket_numbers)?;
        Ok(())
    }

    fn parse_nearby_tickets(&mut self, nearby_tickets: &str) -> Result<(), String> {
        let mut ticket_lines = nearby_tickets.lines();

        if ticket_lines
            .next()
            .unwrap_or_default()
            .ne("nearby tickets:")
        {
            return Err("invalid header for nearby ticket".to_owned());
        }

        for ticket_line in ticket_lines {
            let ticket_numbers = parse_ticket(ticket_line)?;
            self.nearby_tickets.push(ticket_numbers);
        }
        Ok(())
    }
}

fn parse_ticket(ticket_numbers: &str) -> Result<Vec<u16>, String> {
    let parsed_numbers = ticket_numbers
        .split(",")
        .try_fold::<_, _, Result<Vec<u16>, String>>(vec![], |mut acc, number| {
            let parsed_num = number
                .parse::<u16>()
                .map_err(|e| format!("failed to parse ticket number as u16: {}", e))?;

            acc.push(parsed_num);
            Ok(acc)
        })?;
    Ok(parsed_numbers)
}

#[cfg(test)]
mod rule_parsing_tests {
    use super::*;

    #[test]
    fn parse_rule() {
        let rule = "class: 1-3 or 5-7";

        let mut c = Container::new();
        let res = c.parse_rules(rule);
        assert_eq!(res, Ok(()));

        assert_eq!(
            c.rules,
            vec![(
                "class".to_string(),
                [
                    1 << 1 | 1 << 2 | 1 << 3 | 1 << 5 | 1 << 6 | 1 << 7,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0
                ]
            )]
        );
    }

    #[test]
    fn extremes() {
        let rule = "class: 0-1 or 998-999";

        let mut c = Container::new();
        let res = c.parse_rules(rule);
        assert_eq!(res, Ok(()));

        assert_eq!(
            c.rules,
            vec![(
                "class".to_string(),
                [1 << 0 | 1 << 1, 0, 0, 0, 0, 0, 0, 0, 0, 1 << 98 | 1 << 99],
            )]
        );
    }

    #[test]
    fn hundreds_boundaries() {
        let rule = "class: 99-101 or 899-901";

        let mut c = Container::new();
        let res = c.parse_rules(rule);
        assert_eq!(res, Ok(()));

        assert_eq!(
            c.rules,
            vec![(
                "class".to_string(),
                [
                    1 << 99,
                    1 << 0 | 1 << 1,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    1 << 99,
                    1 << 0 | 1 << 1,
                ]
            )]
        );
    }

    #[test]
    fn multiple_rules() {
        let rule = "rule1: 1-2 or 4-5
rule2: 203-204 or 312-313";

        let mut c = Container::new();
        let res = c.parse_rules(rule);
        assert_eq!(res, Ok(()));

        assert_eq!(
            c.rules,
            vec![
                (
                    "rule1".to_string(),
                    [1 << 1 | 1 << 2 | 1 << 4 | 1 << 5, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                ),
                (
                    "rule2".to_string(),
                    [0, 0, 1 << 3 | 1 << 4, 1 << 12 | 1 << 13, 0, 0, 0, 0, 0, 0]
                )
            ]
        );
    }
}

#[cfg(test)]
mod ticket_parsing_tests {
    use super::*;

    #[test]
    fn single_entry() {
        let ticket = "1";

        let res = parse_ticket(ticket);

        assert_eq!(res, Ok(vec![1]));
    }

    #[test]
    fn multiple_entries() {
        let ticket = "1,30,2";

        let res = parse_ticket(ticket);

        assert_eq!(res, Ok(vec![1, 30, 2]));
    }

    #[test]
    fn our_ticket_parsing() {
        let ticket = "your ticket:
1,30,2";

        let mut c = Container::new();

        let res = c.parse_our_ticket(ticket);

        assert_eq!(res, Ok(()));

        assert_eq!(c.our_ticket, vec![1, 30, 2]);
    }

    #[test]
    fn single_nearby_ticket() {
        let ticket = "nearby tickets:
1,30,2";

        let mut c = Container::new();

        let res = c.parse_nearby_tickets(ticket);

        assert_eq!(res, Ok(()));

        assert_eq!(c.nearby_tickets, vec![vec![1, 30, 2]]);
    }

    #[test]
    fn multiple_nearby_tickets() {
        let ticket = "nearby tickets:
1,30,2
20,4,1";

        let mut c = Container::new();

        let res = c.parse_nearby_tickets(ticket);

        assert_eq!(res, Ok(()));

        assert_eq!(c.nearby_tickets, vec![vec![1, 30, 2], vec![20, 4, 1]]);
    }
}

#[cfg(test)]
mod section_splitting_tests {
    use crate::Day;

    use super::*;

    #[test]
    fn split_blocks() {
        let rule = "class: 1-3 or 5-7

your ticket:
7,1,14

nearby tickets:
1,2,3
4,5,6";

        let mut c = Container::new();
        let res = c.parse_input(rule);
        assert_eq!(res, Ok(()));

        assert_eq!(
            c,
            Container {
                rules: vec![(
                    "class".to_string(),
                    [
                        1 << 1 | 1 << 2 | 1 << 3 | 1 << 5 | 1 << 6 | 1 << 7,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0
                    ]
                )],
                our_ticket: vec![7, 1, 14],
                nearby_tickets: vec![vec![1, 2, 3], vec![4, 5, 6]],
            }
        );
    }
}
