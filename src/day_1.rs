/* Source: https://adventofcode.com/2020/day1
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

use std::{error::Error, fs::read_to_string, path::Path};

pub fn run() -> Result<(String, String), Box<dyn Error>> {
    let input_string = read_to_string(Path::new("./data/day_1.txt"))?;

    let input = parse_input(&input_string);

    let part1 = part1(&input)?;

    let part2 = part2(&input)?;

    Ok((part1.to_string(), part2.to_string()))
}

fn parse_input(input_string: &str) -> Vec<i32> {
    let mut out = input_string.lines().fold(Vec::new(), |mut acc, line| {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            acc.push(
                line.trim()
                    .parse::<i32>()
                    .expect("Unable to parse line as integer"),
            );
        }
        acc
    });
    out.sort_unstable();
    out
}

fn part1(input: &[i32]) -> Result<i32, &str> {
    for outer_idx in 0..input.len() {
        let outer = input[outer_idx];
        for inner in input.iter().skip(outer_idx) {
            if outer + inner == 2020 {
                return Ok(outer * inner);
            }
        }
    }

    Err("no matching pair found")
}

fn part2(input: &[i32]) -> Result<i32, &str> {
    for outer_idx in 0..input.len() {
        let outer = input[outer_idx];
        for middle_idx in outer_idx..input.len() {
            let middle = input[middle_idx];
            for inner in input.iter().skip(middle_idx) {
                if outer + middle + inner == 2020 {
                    return Ok(outer * middle * inner);
                }
            }
        }
    }

    Err("no matching triple found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        let input: [i32; 6] = [1721, 979, 366, 299, 675, 1456];

        let expected = 514579;

        assert_eq!(expected, part1(&input).expect(""));
    }

    #[test]
    fn test_part2_example() {
        let input: [i32; 6] = [1721, 979, 366, 299, 675, 1456];

        let expected = 241861950;

        assert_eq!(expected, part2(&input).expect(""))
    }

    #[test]
    fn test_parse_input() {
        let input = "
            10
            20

            1
            ";

        let expected = vec![1, 10, 20];

        assert_eq!(expected, parse_input(&input));
    }
}
