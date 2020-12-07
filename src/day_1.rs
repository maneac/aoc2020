/* Source: https://adventofcode.com/2020/day/1
--- Day 1: Report Repair ---

After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.

The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish;
the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty
of these coins by the time you arrive so you can pay the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle
is unlocked when you complete the first. Each puzzle grants one star. Good luck!

Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently,
something isn't quite adding up.

Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.

For example, suppose your expense report contained the following:


1721
979
366
299
675
1456

In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579,
so the correct answer is 514579.

Of course, your expense report is much larger.

--- Part One ---

Find the two entries that sum to 2020; what do you get if you multiply them together?

--- Part Two ---

The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from
a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.


Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to 2020?

*/

use crate::Day;

pub struct Container {
    input: Vec<i32>,
}

impl Container {
    pub fn new() -> Self {
        Self { input: Vec::new() }
    }
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        let out = input.lines().try_fold(Vec::new(), |mut acc, line| {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let line_res = line.trim().parse::<i32>();
                match line_res {
                    Ok(line) => acc.push(line),
                    Err(e) => return Err(e),
                }
            }
            Ok(acc)
        });
        match out {
            Ok(mut output) => {
                output.sort_unstable();
                self.input = output;
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }

    fn part_1(&self) -> Result<String, String> {
        for outer_idx in 0..self.input.len() {
            let outer = self.input[outer_idx];
            for inner in self.input.iter().skip(outer_idx) {
                if outer + inner == 2020 {
                    return Ok((outer * inner).to_string());
                }
            }
        }

        Err("no matching pair found".to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        for outer_idx in 0..self.input.len() {
            let outer = self.input[outer_idx];
            for middle_idx in outer_idx..self.input.len() {
                let middle = self.input[middle_idx];
                for inner in self.input.iter().skip(middle_idx) {
                    if outer + middle + inner == 2020 {
                        return Ok((outer * middle * inner).to_string());
                    }
                }
            }
        }

        Err("no matching triple found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "
            10
            20

            1
            ";

        let expected = vec![1, 10, 20];

        let mut cont = Container::new();
        assert_eq!(Ok(()), cont.parse_input(&input));
        assert_eq!(expected, cont.input);
    }

    #[test]
    fn test_part_1_examples() {
        let input = Container {
            input: vec![1721, 979, 366, 299, 675, 1456],
        };

        let expected = 514579.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_2_example() {
        let input = Container {
            input: vec![1721, 979, 366, 299, 675, 1456],
        };

        let expected = 241861950.to_string();

        assert_eq!(Ok(expected), input.part_2())
    }
}
