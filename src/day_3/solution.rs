use crate::Day;
use std::cmp::max;

pub struct Container {
    input: Trees,
}

impl Container {
    pub fn new() -> Self {
        Self {
            input: Trees::new(),
        }
    }
}

// This is sufficient as the input data has length 31
#[derive(Debug, PartialOrd, PartialEq)]
struct Trees {
    trees: Vec<u32>,
    row_len: usize,
}

impl Trees {
    fn new() -> Self {
        Self {
            trees: Vec::new(),
            row_len: 0,
        }
    }
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        let mut out = Trees::new();
        for line in input.lines() {
            out.row_len = max(out.row_len, line.trim().len());
            out.trees.push(
                line.trim()
                    .char_indices()
                    .fold(0u32, |mut acc, (idx, chr)| {
                        if chr == '#' {
                            acc |= 1 << idx;
                        }
                        acc
                    }),
            );
        }
        self.input = out;
        Ok(())
    }

    fn part_1(&self) -> Result<String, String> {
        Ok(self
            .input
            .trees
            .iter()
            .enumerate()
            .filter(|(row_num, row)| {
                // D1, R3
                ((1 << ((3 * row_num) % self.input.row_len)) as u32 & *row) > 0
            })
            .count()
            .to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        Ok(self
            .input
            .trees
            .iter()
            .enumerate()
            .fold([0usize; 5], |mut acc, (row_num, row)| {
                // D1, R1
                if ((1 << (row_num % self.input.row_len)) as u32 & *row) > 0 {
                    acc[0] += 1;
                }
                // D1, R3
                if ((1 << ((3 * row_num) % self.input.row_len)) as u32 & *row) > 0 {
                    acc[1] += 1;
                }
                // D1, R5
                if ((1 << ((5 * row_num) % self.input.row_len)) as u32 & *row) > 0 {
                    acc[2] += 1;
                }
                // D1, R7
                if ((1 << ((7 * row_num) % self.input.row_len)) as u32 & *row) > 0 {
                    acc[3] += 1;
                }
                // D2, R1
                if row_num % 2 == 0
                    && ((1 << ((row_num / 2) % self.input.row_len)) as u32 & *row) > 0
                {
                    acc[4] += 1;
                }
                acc
            })
            .iter()
            .fold(0usize, |acc, &count| {
                if acc == 0 {
                    return count;
                }
                acc * count
            })
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        let expected = Trees {
            trees: vec![
                (1 << 2) | (1 << 3),
                (1 << 0) | (1 << 4) | (1 << 8),
                (1 << 1) | (1 << 6) | (1 << 9),
                (1 << 2) | (1 << 4) | (1 << 8) | (1 << 10),
                (1 << 1) | (1 << 5) | (1 << 6) | (1 << 9),
                (1 << 2) | (1 << 4) | (1 << 5),
                (1 << 1) | (1 << 3) | (1 << 5) | (1 << 10),
                (1 << 1) | (1 << 10),
                (1 << 0) | (1 << 2) | (1 << 3) | (1 << 7),
                (1 << 0) | (1 << 4) | (1 << 5) | (1 << 10),
                (1 << 1) | (1 << 4) | (1 << 8) | (1 << 10),
            ],
            row_len: 11,
        };

        let mut cont = Container::new();
        assert_eq!(Ok(()), cont.parse_input(&input));
        assert_eq!(expected, cont.input);
    }

    #[test]
    fn test_part_1_example() {
        let input = Container {
            input: Trees {
                trees: vec![
                    (1 << 2) | (1 << 3),
                    (1 << 0) | (1 << 4) | (1 << 8),
                    (1 << 1) | (1 << 6) | (1 << 9),
                    (1 << 2) | (1 << 4) | (1 << 8) | (1 << 10),
                    (1 << 1) | (1 << 5) | (1 << 6) | (1 << 9),
                    (1 << 2) | (1 << 4) | (1 << 5),
                    (1 << 1) | (1 << 3) | (1 << 5) | (1 << 10),
                    (1 << 1) | (1 << 10),
                    (1 << 0) | (1 << 2) | (1 << 3) | (1 << 7),
                    (1 << 0) | (1 << 4) | (1 << 5) | (1 << 10),
                    (1 << 1) | (1 << 4) | (1 << 8) | (1 << 10),
                ],
                row_len: 11,
            },
        };

        let expected = 7.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_2_example() {
        let input = Container {
            input: Trees {
                trees: vec![
                    (1 << 2) | (1 << 3),
                    (1 << 0) | (1 << 4) | (1 << 8),
                    (1 << 1) | (1 << 6) | (1 << 9),
                    (1 << 2) | (1 << 4) | (1 << 8) | (1 << 10),
                    (1 << 1) | (1 << 5) | (1 << 6) | (1 << 9),
                    (1 << 2) | (1 << 4) | (1 << 5),
                    (1 << 1) | (1 << 3) | (1 << 5) | (1 << 10),
                    (1 << 1) | (1 << 10),
                    (1 << 0) | (1 << 2) | (1 << 3) | (1 << 7),
                    (1 << 0) | (1 << 4) | (1 << 5) | (1 << 10),
                    (1 << 1) | (1 << 4) | (1 << 8) | (1 << 10),
                ],
                row_len: 11,
            },
        };

        let expected = 336.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }
}
