/* Source: https://adventofcode.com/2020/day/6
 --- Day 6: Custom Customs ---

As your flight approaches the regional airport where you'll switch to a much larger plane, customs declaration forms are
distributed to the passengers.

The form asks a series of 26 yes-or-no questions marked a through z. All you need to do is identify the questions for
which anyone in your group answers "yes". Since your group is just you, this doesn't take very long.

However, the person sitting next to you seems to be experiencing a language barrier and asks if you can help. For each
of the people in their group, you write down the questions for which they answer "yes", one per line. For example:

abcx
abcy
abcz

In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z. (Duplicate answers to the
same question don't count extra; each question counts at most once.)

Another group asks for your help, then another, and eventually you've collected answers from every group on the plane
(your puzzle input). Each group's answers are separated by a blank line, and within each group, each person's answers
are on a single line. For example:

abc

a
b
c

ab
ac

a
a
a
a

b

This list represents answers from five groups:

    The first group contains one person who answered "yes" to 3 questions: a, b, and c.
    The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
    The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
    The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
    The last group contains one person who answered "yes" to only 1 question, b.

In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.

--- Part One ---

For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?

--- Part Two ---

As you finish the last group's customs declaration, you notice that you misread one word in the instructions:

You don't need to identify the questions to which anyone answered "yes"; you need to identify the questions to which
everyone answered "yes"!

Using the same example as above:

abc

a
b
c

ab
ac

a
a
a
a

b

This list represents answers from five groups:

    - In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
    - In the second group, there is no question to which everyone answered "yes".
    - In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or
        c, they don't count.
    - In the fourth group, everyone answered yes to only 1 question, a.
    - In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.

In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.

For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?

*/

pub struct Container {
    input: Vec<Group>,
}

impl Container {
    pub fn new() -> Self {
        Self { input: Vec::new() }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Group {
    or: u32,
    and: u32,
}

impl crate::Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        self.input = input
            .trim()
            .split("\n\n")
            .map(|group| {
                group
                    .trim()
                    .lines()
                    .fold(Group { or: 0, and: !0 }, |mut acc, line| {
                        let entry = line.chars().fold(0u32, |mut acc, chr| {
                            acc |= 1 << (chr as u8 - b'a');
                            acc
                        });
                        acc.or |= entry;
                        acc.and &= entry;
                        acc
                    })
            })
            .collect();
        Ok(())
    }

    fn part_1(&self) -> Result<String, String> {
        Ok(self
            .input
            .iter()
            .fold(0u32, |mut acc, group| {
                acc += group.or.count_ones();
                acc
            })
            .to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        Ok(self
            .input
            .iter()
            .fold(0u32, |mut acc, group| {
                acc += group.and.count_ones();
                acc
            })
            .to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let expected = vec![
            Group {
                or: 1 << 0 | 1 << 1 | 1 << 2,
                and: 1 << 0 | 1 << 1 | 1 << 2,
            },
            Group {
                or: 1 << 0 | 1 << 1 | 1 << 2,
                and: 0,
            },
            Group {
                or: 1 << 0 | 1 << 1 | 1 << 2,
                and: 1 << 0,
            },
            Group {
                or: 1 << 0,
                and: 1 << 0,
            },
            Group {
                or: 1 << 1,
                and: 1 << 1,
            },
        ];

        assert_eq!(expected, parse_input(input));
    }

    #[test]
    fn test_part1_example() {
        let input = vec![
            Group {
                or: 1 << 0 | 1 << 1 | 1 << 2,
                and: 1 << 0 | 1 << 1 | 1 << 2,
            },
            Group {
                or: 1 << 0 | 1 << 1 | 1 << 2,
                and: 0,
            },
            Group {
                or: 1 << 0 | 1 << 1 | 1 << 2,
                and: 1 << 0,
            },
            Group {
                or: 1 << 0,
                and: 1 << 0,
            },
            Group {
                or: 1 << 1,
                and: 1 << 1,
            },
        ];

        let expected = 11;

        assert_eq!(expected, part_1(&input));
    }

    #[test]
    fn test_part2_example() {
        let input = vec![
            Group {
                or: 1 << 0 | 1 << 1 | 1 << 2,
                and: 1 << 0 | 1 << 1 | 1 << 2,
            },
            Group {
                or: 1 << 0 | 1 << 1 | 1 << 2,
                and: 0,
            },
            Group {
                or: 1 << 0 | 1 << 1 | 1 << 2,
                and: 1 << 0,
            },
            Group {
                or: 1 << 0,
                and: 1 << 0,
            },
            Group {
                or: 1 << 1,
                and: 1 << 1,
            },
        ];

        let expected = 6;

        assert_eq!(expected, part_2(&input));
    }
}
