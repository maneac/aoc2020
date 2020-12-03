/* Source: https://adventofcode.com/2020/day/3
--- Day 3: Toboggan Trajectory ---

With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy, it's
certainly not safe: there's very minimal steering and the area is covered in trees. You'll need to see which angles will take you near the fewest trees.

Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map
(your puzzle input) of the open squares (.) and trees (#) you can see. For example:

..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#

These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability,
the same pattern repeats to the right many times:

..##.........##.........##.........##.........##.........##.......  --->
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->

You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).

The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start
by counting all the trees you would encounter for the slope right 3, down 1:

From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that
is right 3 and down 1 from there, and so on until you go past the bottom of the map.

The locations you'd check in the above example are marked here with O where there was an open square and X where there was a tree:

..##.........##.........##.........##.........##.........##.......  --->
#..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........X.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...#X....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->

In this example, traversing the map using this slope would cause you to encounter 7 trees.

--- Part 1 ---

Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?

--- Part Two ---

Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.

Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner
and traverse the map all the way to the bottom:

    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked.)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2.

In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.

What do you get if you multiply together the number of trees encountered on each of the listed slopes?
*/

use std::path::Path;
use std::{cmp::max, fs};

pub fn run() {
    let input_string = match fs::read_to_string(Path::new("./data/day_3.txt")) {
        Ok(l) => l,
        Err(e) => panic!("Failed to open data file for day 3: {}", e),
    };

    let input = parse_input(&input_string);

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

// This is sufficient as the input data has length 31
#[derive(Debug, PartialOrd, PartialEq)]
struct Trees {
    trees: Vec<u32>,
    row_len: usize,
}

impl Trees {
    fn new() -> Self {
        return Self {
            trees: Vec::new(),
            row_len: 0,
        };
    }
}

fn parse_input(input: &str) -> Trees {
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
                    return acc;
                }),
        );
    }
    return out;
}

fn part_1(input: &Trees) -> usize {
    return input
        .trees
        .iter()
        .enumerate()
        .filter(|(row_num, row)| {
            // D1, R3
            ((1 << ((3 * row_num) % input.row_len)) as u32 & *row) > 0
        })
        .count();
}

fn part_2(input: &Trees) -> usize {
    return input
        .trees
        .iter()
        .enumerate()
        .fold([0usize; 5], |mut acc, (row_num, row)| {
            // D1, R1
            if ((1 << row_num % input.row_len) as u32 & *row) > 0 {
                acc[0] += 1;
            }
            // D1, R3
            if ((1 << ((3 * row_num) % input.row_len)) as u32 & *row) > 0 {
                acc[1] += 1;
            }
            // D1, R5
            if ((1 << ((5 * row_num) % input.row_len)) as u32 & *row) > 0 {
                acc[2] += 1;
            }
            // D1, R7
            if ((1 << ((7 * row_num) % input.row_len)) as u32 & *row) > 0 {
                acc[3] += 1;
            }
            // D2, R1
            if row_num % 2 == 0 && ((1 << ((row_num / 2) % input.row_len)) as u32 & *row) > 0 {
                acc[4] += 1;
            }
            return acc;
        })
        .iter()
        .fold(0usize, |acc, &count| {
            if acc == 0 {
                return count;
            }
            return acc * count;
        });
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

        assert_eq!(expected, parse_input(&input));
    }

    #[test]

    fn test_part1_example() {
        let input = Trees {
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

        let expected = 7;

        assert_eq!(expected, part_1(&input));
    }

    #[test]
    fn test_part2_example() {
        let input = Trees {
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

        let expected = 336;

        assert_eq!(expected, part_2(&input));
    }
}
