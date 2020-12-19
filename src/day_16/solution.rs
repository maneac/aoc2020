use crate::Day;

type Ticket = Vec<usize>;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Container {
    notes: Vec<Note>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            my_ticket: Ticket::new(),
            nearby_tickets: Vec::new(),
        }
    }

    fn get_valid_tickets(&self) -> (Vec<&Ticket>, usize) {
        let mut valid_range = self.notes.iter().fold(Vec::new(), |mut acc, note| {
            for range in note.valid_ranges.iter() {
                for num in range.min..=range.max {
                    acc.push(num);
                }
            }
            acc
        });
        valid_range.sort_unstable();
        valid_range.dedup();

        let mut err_rate = 0;
        (
            self.nearby_tickets
                .iter()
                .filter(|ticket| {
                    for val in ticket.iter() {
                        if !valid_range.contains(val) {
                            err_rate += val;
                            return false;
                        }
                    }
                    true
                })
                .collect::<Vec<&Ticket>>(),
            err_rate,
        )
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Note {
    name: String,
    valid_ranges: [Range; 2],
}

impl Note {
    fn new() -> Self {
        Self {
            name: String::new(),
            valid_ranges: [Range::new(); 2],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn new() -> Self {
        Self { min: 0, max: 0 }
    }
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        let mut sections = input.trim().split("\n\n");

        for line in sections
            .next()
            .ok_or_else(|| "failed to find note section".to_owned())?
            .trim()
            .lines()
        {
            let mut note = Note::new();
            let mut line_parts = line.split(':');
            note.name = line_parts
                .next()
                .ok_or_else(|| "failed to get note name".to_owned())?
                .trim()
                .to_owned();

            line_parts
                .next()
                .ok_or_else(|| "failed to get note ranges".to_owned())?
                .trim()
                .split(" or ")
                .enumerate()
                .try_for_each::<_, Result<(), String>>(|(idx, range_str)| {
                    range_str
                        .split('-')
                        .enumerate()
                        .try_for_each(|(chunk_idx, chunk)| {
                            let num = chunk.parse::<usize>().map_err(|e| {
                                format!("failed to parse range bound as usize: {}", e)
                            })?;
                            match chunk_idx {
                                0 => note.valid_ranges[idx].min = num,
                                1 => note.valid_ranges[idx].max = num,
                                _ => {
                                    return Err(format!("invalid range part index: {}", chunk_idx))
                                }
                            }
                            Ok(())
                        })
                })?;

            self.notes.push(note);
        }

        self.my_ticket = sections
            .next()
            .ok_or_else(|| "failed to get my ticket section".to_owned())?
            .lines()
            .nth(1)
            .ok_or_else(|| "failed to get my ticket".to_owned())?
            .trim()
            .split(',')
            .try_fold::<_, _, Result<Vec<usize>, String>>(Vec::new(), |mut acc, entry| {
                acc.push(
                    entry
                        .parse::<usize>()
                        .map_err(|e| format!("failed to parse my ticket entry as usize: {}", e))?,
                );
                Ok(acc)
            })?;

        for line in sections
            .next()
            .ok_or_else(|| "failed to get nearby ticket section".to_owned())?
            .trim()
            .lines()
            .skip(1)
        {
            let ticket = line
                .trim()
                .split(',')
                .try_fold::<_, _, Result<Vec<usize>, String>>(Vec::new(), |mut acc, entry| {
                    acc.push(
                        entry
                            .parse::<usize>()
                            .map_err(|e| format!("failed to parse ticket entry as usize: {}", e))?,
                    );
                    Ok(acc)
                })?;
            self.nearby_tickets.push(ticket);
        }

        Ok(())
    }

    fn part_1(&self) -> Result<String, String> {
        let (_, error_rate) = self.get_valid_tickets();

        Ok(error_rate.to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        let mut possible_classes: Vec<u32> =
            vec![(1 << self.my_ticket.len()) - 1; self.my_ticket.len()];

        let (valid_tickets, _) = self.get_valid_tickets();

        for (idx, possibility) in possible_classes.iter_mut().enumerate() {
            for ticket in valid_tickets.iter() {
                let entry = ticket[idx];
                for (note_idx, note) in self.notes.iter().enumerate() {
                    if entry < note.valid_ranges[0].min
                        || entry > note.valid_ranges[1].max
                        || (entry > note.valid_ranges[0].max && entry < note.valid_ranges[1].min)
                    {
                        *possibility &= !(1 << note_idx);
                    }
                }
            }
        }

        let mut output = 1;
        for _ in 0..self.notes.len() {
            let (idx, &mask) = possible_classes
                .iter()
                .enumerate()
                .find(|(_, c)| c.count_ones() == 1)
                .ok_or_else(|| "no possible values left".to_owned())?;

            let class_name = &self.notes[idx].name;
            let col = (mask as f64).log2() as usize;
            let val = self.my_ticket[col];

            if class_name.starts_with("departure") {
                output *= val;
            }

            for class in possible_classes.iter_mut() {
                *class &= !(mask);
            }
        }

        Ok(output.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "class: 1-3 or 5-7
departure row: 6-11 or 33-44
seat: 13-40 or 45-500

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

        let expected = Container {
            notes: vec![
                Note {
                    name: "class".to_owned(),
                    valid_ranges: [Range { min: 1, max: 3 }, Range { min: 5, max: 7 }],
                },
                Note {
                    name: "departure row".to_owned(),
                    valid_ranges: [Range { min: 6, max: 11 }, Range { min: 33, max: 44 }],
                },
                Note {
                    name: "seat".to_owned(),
                    valid_ranges: [Range { min: 13, max: 40 }, Range { min: 45, max: 500 }],
                },
            ],
            my_ticket: vec![7, 1, 14],
            nearby_tickets: vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12],
            ],
        };

        let mut cont = Container::new();

        assert_eq!(Ok(()), cont.parse_input(input));
        assert_eq!(expected, cont);
    }

    #[test]
    fn test_part_1_example() {
        let input = Container {
            notes: vec![
                Note {
                    name: "class".to_owned(),
                    valid_ranges: [Range { min: 1, max: 3 }, Range { min: 5, max: 7 }],
                },
                Note {
                    name: "row".to_owned(),
                    valid_ranges: [Range { min: 6, max: 11 }, Range { min: 33, max: 44 }],
                },
                Note {
                    name: "seat".to_owned(),
                    valid_ranges: [Range { min: 13, max: 40 }, Range { min: 45, max: 50 }],
                },
            ],
            my_ticket: vec![7, 1, 14],
            nearby_tickets: vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12],
            ],
        };

        let expected = 71.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_2_example() {
        let input = Container {
            notes: vec![
                Note {
                    name: "class".to_owned(),
                    valid_ranges: [Range { min: 0, max: 1 }, Range { min: 4, max: 19 }],
                },
                Note {
                    name: "departure row".to_owned(),
                    valid_ranges: [Range { min: 0, max: 5 }, Range { min: 8, max: 19 }],
                },
                Note {
                    name: "departure seat".to_owned(),
                    valid_ranges: [Range { min: 0, max: 13 }, Range { min: 16, max: 19 }],
                },
            ],
            my_ticket: vec![11, 12, 13],
            nearby_tickets: vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9]],
        };

        let expected = 143.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }

    #[test]
    fn test_part_2_invalid_tickets() {
        let input = Container {
            notes: vec![
                Note {
                    name: "class".to_owned(),
                    valid_ranges: [Range { min: 0, max: 1 }, Range { min: 4, max: 19 }],
                },
                Note {
                    name: "departure row".to_owned(),
                    valid_ranges: [Range { min: 0, max: 5 }, Range { min: 8, max: 19 }],
                },
                Note {
                    name: "departure seat".to_owned(),
                    valid_ranges: [Range { min: 0, max: 13 }, Range { min: 16, max: 19 }],
                },
            ],
            my_ticket: vec![11, 12, 13],
            nearby_tickets: vec![
                vec![3, 9, 18],
                vec![15, 1, 5],
                vec![5, 14, 9],
                vec![6, 20, 11],
                vec![6, 0, 20],
            ],
        };

        let expected = 143.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }

    #[test]
    fn test_part_2_boundaries() {
        let input = Container {
            notes: vec![
                Note {
                    name: "class".to_owned(),
                    valid_ranges: [Range { min: 0, max: 1 }, Range { min: 4, max: 19 }],
                },
                Note {
                    name: "departure row".to_owned(),
                    valid_ranges: [Range { min: 0, max: 5 }, Range { min: 8, max: 19 }],
                },
                Note {
                    name: "departure seat".to_owned(),
                    valid_ranges: [Range { min: 0, max: 13 }, Range { min: 16, max: 19 }],
                },
            ],
            my_ticket: vec![11, 12, 13],
            nearby_tickets: vec![
                vec![0, 0, 0],
                vec![5, 0, 0],
                vec![8, 0, 0],
                vec![19, 0, 0],
                vec![0, 1, 0],
                vec![0, 4, 0],
                vec![0, 19, 0],
                vec![0, 0, 13],
                vec![0, 0, 16],
                vec![0, 0, 19],
                vec![8, 14, 3],
                vec![8, 6, 3],
                vec![0, 0, 6],
            ],
        };

        let expected = 143.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }
}
