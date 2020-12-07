/* Source: https://adventofcode.com/2020/day/7
--- Day 7: Handy Haversacks ---

You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some
food: all flights are currently delayed due to issues in luggage processing.

Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags
must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for
these regulations considered how long they would take to enforce!

For example, consider the following rules:

light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.

These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant
plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.

You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be
valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)

In the above rules, the following options would be available to you:

    A bright white bag, which can hold your shiny gold bag directly.
    A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
    A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
    A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.

So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.

--- Part One ---

How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)

--- Part Two ---

It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!

Consider again your shiny gold bag and the rules from the above example:

    faded blue bags contain 0 other bags.
    dotted black bags contain 0 other bags.
    vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
    dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.

So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags
(and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!

Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all
of the bags, even if the nesting becomes topologically impractical!

Here's another example:

shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.

In this example, a single shiny gold bag must contain 126 other bags.

How many individual bags are required inside your single shiny gold bag?

 */

use crate::Day;
use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};

pub struct Container {
    input: HashMap<String, Bag>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            input: HashMap::new(),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Bag {
    parents: Vec<Entry>,
    children: Vec<Entry>,
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Entry {
    count: usize,
    name: String,
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        for line in input.trim().lines() {
            let mut bag_and_children = line.split(" bags contain ");

            let bag_name = bag_and_children
                .next()
                .ok_or_else(|| "no name for bag line".to_string())?;

            let children = bag_and_children
                .next()
                .ok_or_else(|| "no children for bag".to_string())?
                .trim_end_matches('.')
                .split(", ")
                .fold(Ok(vec![]), |res: Result<Vec<Entry>, String>, part| {
                    if let Ok(mut acc) = res {
                        if part.contains("no other") {
                            return Ok(acc);
                        }
                        acc.push(
                            part.split(" bag")
                                .next()
                                .ok_or_else(|| "no bag in tail of line".to_string())?
                                .split(' ')
                                .enumerate()
                                .fold(
                                    Ok(Entry {
                                        count: 0,
                                        name: String::new(),
                                    }),
                                    |res: Result<Entry, String>, (idx, chunk)| {
                                        if let Ok(mut entry) = res {
                                            if idx == 0 {
                                                entry.count = match chunk.parse::<usize>() {
                                                    Ok(c) => c,
                                                    Err(e) => return Err(e.to_string()),
                                                };
                                            } else if entry.name.is_empty() {
                                                entry.name = chunk.to_string();
                                            } else {
                                                entry.name.push(' ');
                                                entry.name.push_str(chunk);
                                            }
                                            return Ok(entry);
                                        }
                                        res
                                    },
                                )?,
                        );
                        return Ok(acc);
                    }
                    res
                })?;

            for child in children.iter() {
                self.input
                    .entry(child.name.to_string())
                    .and_modify(|bag| {
                        (*bag).parents.push(Entry {
                            count: child.count,
                            name: bag_name.to_string(),
                        })
                    })
                    .or_insert(Bag {
                        parents: vec![Entry {
                            count: child.count,
                            name: bag_name.to_string(),
                        }],
                        children: vec![],
                    });
            }

            match self.input.entry(bag_name.to_string()) {
                Occupied(bag_entry) => {
                    bag_entry.into_mut().children = children;
                }
                Vacant(bag_entry) => {
                    bag_entry.insert(Bag {
                        parents: vec![],
                        children,
                    });
                }
            }
        }
        Ok(())
    }

    fn part_1(&self) -> Result<String, String> {
        let mut out = count_parents(&self.input, "shiny gold")
            .ok_or_else(|| "no shiny gold bag".to_string())?;

        out.sort();
        out.dedup();

        Ok(out.len().to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        Ok(count_children(&self.input, "shiny gold")
            .ok_or_else(|| "no shiny gold bag".to_string())?
            .to_string())
    }
}

fn count_parents(map: &HashMap<String, Bag>, entry: &str) -> Option<Vec<String>> {
    if let Some(e) = map.get(entry) {
        let resp = e.parents.iter().fold(vec![], |mut acc, c| {
            if let Some(mut res) = count_parents(map, &c.name) {
                acc.append(&mut res);
            }
            acc.push(c.name.to_string());
            acc
        });

        return Some(resp);
    }
    None
}

fn count_children(map: &HashMap<String, Bag>, entry: &str) -> Option<usize> {
    if let Some(e) = map.get(entry) {
        let resp = e.children.iter().fold(0usize, |mut acc, c| {
            acc += c.count * (1 + count_children(map, &c.name).unwrap());
            acc
        });
        return Some(resp);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_simple() {
        let mut input = Container {
            input: HashMap::new(),
        };
        input.input.insert(
            "shiny gold".to_string(),
            Bag {
                parents: vec![],
                children: vec![Entry {
                    count: 2,
                    name: "bag2".to_string(),
                }],
            },
        );
        input.input.insert(
            "bag2".to_string(),
            Bag {
                parents: vec![],
                children: vec![Entry {
                    count: 3,
                    name: "bag3".to_string(),
                }],
            },
        );
        input.input.insert(
            "bag3".to_string(),
            Bag {
                parents: vec![],
                children: vec![],
            },
        );

        assert_eq!(Ok(8.to_string()), input.part_2());
    }

    fn make_example() -> HashMap<String, Bag> {
        let mut expected = HashMap::new();
        expected.insert(
            "light red".to_string(),
            Bag {
                parents: vec![],
                children: vec![
                    Entry {
                        count: 1,
                        name: "bright white".to_string(),
                    },
                    Entry {
                        count: 2,
                        name: "muted yellow".to_string(),
                    },
                ],
            },
        );
        expected.insert(
            "bright white".to_string(),
            Bag {
                parents: vec![
                    Entry {
                        count: 1,
                        name: "light red".to_string(),
                    },
                    Entry {
                        count: 3,
                        name: "dark orange".to_string(),
                    },
                ],
                children: vec![Entry {
                    count: 1,
                    name: "shiny gold".to_string(),
                }],
            },
        );
        expected.insert(
            "muted yellow".to_string(),
            Bag {
                parents: vec![
                    Entry {
                        count: 2,
                        name: "light red".to_string(),
                    },
                    Entry {
                        count: 4,
                        name: "dark orange".to_string(),
                    },
                ],
                children: vec![
                    Entry {
                        count: 2,
                        name: "shiny gold".to_string(),
                    },
                    Entry {
                        count: 9,
                        name: "faded blue".to_string(),
                    },
                ],
            },
        );
        expected.insert(
            "dark orange".to_string(),
            Bag {
                parents: vec![],
                children: vec![
                    Entry {
                        count: 3,
                        name: "bright white".to_string(),
                    },
                    Entry {
                        count: 4,
                        name: "muted yellow".to_string(),
                    },
                ],
            },
        );
        expected.insert(
            "shiny gold".to_string(),
            Bag {
                parents: vec![
                    Entry {
                        count: 1,
                        name: "bright white".to_string(),
                    },
                    Entry {
                        count: 2,
                        name: "muted yellow".to_string(),
                    },
                ],
                children: vec![
                    Entry {
                        count: 1,
                        name: "dark olive".to_string(),
                    },
                    Entry {
                        count: 2,
                        name: "vibrant plum".to_string(),
                    },
                ],
            },
        );
        expected.insert(
            "faded blue".to_string(),
            Bag {
                parents: vec![
                    Entry {
                        count: 9,
                        name: "muted yellow".to_string(),
                    },
                    Entry {
                        count: 3,
                        name: "dark olive".to_string(),
                    },
                    Entry {
                        count: 5,
                        name: "vibrant plum".to_string(),
                    },
                ],
                children: vec![],
            },
        );
        expected.insert(
            "dark olive".to_string(),
            Bag {
                parents: vec![Entry {
                    count: 1,
                    name: "shiny gold".to_string(),
                }],
                children: vec![
                    Entry {
                        count: 3,
                        name: "faded blue".to_string(),
                    },
                    Entry {
                        count: 4,
                        name: "dotted black".to_string(),
                    },
                ],
            },
        );
        expected.insert(
            "vibrant plum".to_string(),
            Bag {
                parents: vec![Entry {
                    count: 2,
                    name: "shiny gold".to_string(),
                }],
                children: vec![
                    Entry {
                        count: 5,
                        name: "faded blue".to_string(),
                    },
                    Entry {
                        count: 6,
                        name: "dotted black".to_string(),
                    },
                ],
            },
        );
        expected.insert(
            "dotted black".to_string(),
            Bag {
                parents: vec![
                    Entry {
                        count: 4,
                        name: "dark olive".to_string(),
                    },
                    Entry {
                        count: 6,
                        name: "vibrant plum".to_string(),
                    },
                ],
                children: vec![],
            },
        );
        expected
    }

    #[test]
    fn test_parse_input() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let expected = make_example();
        let mut sorted_expected = expected.iter().fold(vec![], |mut acc, entry| {
            acc.push(entry);
            acc
        });
        sorted_expected.sort_by(|a, b| a.0.cmp(b.0));

        let mut cont = Container::new();

        assert_eq!(Ok(()), cont.parse_input(input));

        let mut parsed_input = cont.input.iter().fold(vec![], |mut acc, entry| {
            acc.push(entry);
            acc
        });
        parsed_input.sort_by(|a, b| a.0.cmp(b.0));

        parsed_input
            .iter()
            .zip(sorted_expected.iter())
            .for_each(|(parsed_bag, expected_bag)| {
                assert_eq!(expected_bag, parsed_bag);
            });
    }

    #[test]
    fn test_part_1_example() {
        let input = Container {
            input: make_example(),
        };

        let expected = 4.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_2_example_1() {
        let input = Container {
            input: make_example(),
        };

        let expected = 32.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }

    #[test]
    fn test_part_2_example_2() {
        let mut input = Container {
            input: HashMap::new(),
        };
        input.input.insert(
            "shiny gold".to_string(),
            Bag {
                parents: vec![],
                children: vec![Entry {
                    count: 2,
                    name: "dark red".to_string(),
                }],
            },
        );
        input.input.insert(
            "dark red".to_string(),
            Bag {
                parents: vec![],
                children: vec![Entry {
                    count: 2,
                    name: "dark orange".to_string(),
                }],
            },
        );
        input.input.insert(
            "dark orange".to_string(),
            Bag {
                parents: vec![],
                children: vec![Entry {
                    count: 2,
                    name: "dark yellow".to_string(),
                }],
            },
        );
        input.input.insert(
            "dark yellow".to_string(),
            Bag {
                parents: vec![],
                children: vec![Entry {
                    count: 2,
                    name: "dark green".to_string(),
                }],
            },
        );
        input.input.insert(
            "dark green".to_string(),
            Bag {
                parents: vec![],
                children: vec![Entry {
                    count: 2,
                    name: "dark blue".to_string(),
                }],
            },
        );
        input.input.insert(
            "dark blue".to_string(),
            Bag {
                parents: vec![],
                children: vec![Entry {
                    count: 2,
                    name: "dark violet".to_string(),
                }],
            },
        );
        input.input.insert(
            "dark violet".to_string(),
            Bag {
                parents: vec![],
                children: vec![],
            },
        );

        let expected = 126.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }
}
