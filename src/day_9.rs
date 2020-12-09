/* Source: https://adventofcode.com/2020/day/9
--- Day 9: Encoding Error ---

With your neighbor happily enjoying their video game, you turn your attention to an open data port on the little screen
in the seat in front of you.

Though the port is non-standard, you manage to connect it to your computer through the clever use of several paperclips.
Upon connection, the port outputs a series of numbers (your puzzle input).

The data appears to be encrypted with the eXchange-Masking Addition System (XMAS) which, conveniently for you, is an old
cypher with an important weakness.

XMAS starts by transmitting a preamble of 25 numbers. After that, each number you receive should be the sum of any two
of the 25 immediately previous numbers. The two numbers will have different values, and there might be more than one
such pair.

For example, suppose your preamble consists of the numbers 1 through 25 in a random order. To be valid, the next number
must be the sum of two of those numbers:

    26 would be a valid next number, as it could be 1 plus 25 (or many other pairs, like 2 and 24).
    49 would be a valid next number, as it is the sum of 24 and 25.
    100 would not be valid; no two of the previous 25 numbers sum to 100.
    50 would also not be valid; although 25 appears in the previous 25 numbers, the two numbers in the pair must be
        different.

Suppose the 26th number is 45, and the first number (no longer an option, as it is more than 25 numbers ago) was 20.
Now, for the next number to be valid, there needs to be some pair of numbers among 1-19, 21-25, or 45 that add up to it:

    26 would still be a valid next number, as 1 and 25 are still within the previous 25 numbers.
    65 would not be valid, as no two of the available numbers sum to it.
    64 and 66 would both be valid, as they are the result of 19+45 and 21+45 respectively.

Here is a larger example which only considers the previous 5 numbers (and has a preamble of length 5):

35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576

In this example, after the 5-number preamble, almost every number is the sum of two of the previous 5 numbers; the only
number that does not follow this rule is 127.

--- Part One ---

The first step of attacking the weakness in the XMAS data is to find the first number in the list (after the preamble)
which is not the sum of two of the 25 numbers before it. What is the first number that does not have this property?

--- Part Two ---

The final step in breaking the XMAS encryption relies on the invalid number you just found: you must find a contiguous
set of at least two numbers in your list which sum to the invalid number from step 1.

Again consider the above example:

35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576

In this list, adding up all of the numbers from 15 through 40 produces the invalid number from step 1, 127. (Of course,
the contiguous set of numbers in your actual list might be much longer.)

To find the encryption weakness, add together the smallest and largest number in this contiguous range; in this example,
these are 15 and 47, producing 62.

What is the encryption weakness in your XMAS-encrypted list of numbers?
 */

use crate::Day;
use std::cell::RefCell;

pub struct Container {
    preamble_len: usize,
    input: Vec<usize>,
    prev_num: RefCell<usize>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            preamble_len: 25,
            input: Vec::new(),
            prev_num: RefCell::new(0),
        }
    }

    // returns the index of the first matching pair, or none if no match
    fn check_previous_preamble(&self, idx: usize) -> Option<(usize, usize)> {
        let target = self.input[idx];
        for i in (idx - self.preamble_len)..idx {
            let outer = self.input[i];
            for j in i..idx {
                if self.input[j] + outer == target {
                    return Some((i, j));
                }
            }
        }
        self.prev_num.replace(target);
        None
    }
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        input.trim().lines().try_for_each(|line| {
            let num = line.parse::<usize>();
            match num {
                Ok(i) => {
                    self.input.push(i);
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            }
        })
    }

    fn part_1(&self) -> Result<String, String> {
        for idx in self.preamble_len..self.input.len() {
            if self.check_previous_preamble(idx).is_none() {
                return Ok(self.input[idx].to_string());
            }
        }
        Err("no invalid entry found".to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        let target = *self.prev_num.borrow();
        for idx in 0..self.input.len() {
            let attempt = self.input.iter().skip(idx).try_fold(vec![], |mut acc, i| {
                acc.push(*i);
                let sum = acc.iter().sum::<usize>();
                match sum.cmp(&target) {
                    std::cmp::Ordering::Less => Ok(acc),
                    std::cmp::Ordering::Equal => Err(Some(acc)),
                    std::cmp::Ordering::Greater => Err(None),
                }
            });
            if let Err(Some(res)) = attempt {
                return Ok(
                    (res.iter().min().unwrap_or(&0) + res.iter().max().unwrap_or(&0)).to_string(),
                );
            }
        }
        Err("no sequence found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        let expected = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let mut cont = Container::new();
        assert_eq!(Ok(()), cont.parse_input(&input));
        assert_eq!(expected, cont.input);
    }

    #[test]
    fn test_part_1_example() {
        let input = Container {
            preamble_len: 5,
            input: vec![
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576,
            ],
            prev_num: RefCell::new(0),
        };

        let expected = 127;

        assert_eq!(Ok(expected.to_string()), input.part_1());
        assert_eq!(expected, *input.prev_num.borrow());
    }

    #[test]
    fn test_part_2_example() {
        let input = Container {
            preamble_len: 5,
            input: vec![
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576,
            ],
            prev_num: RefCell::new(127),
        };

        let expected = 62.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }
}
