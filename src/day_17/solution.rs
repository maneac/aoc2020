use std::cmp::Ordering;

use crate::Day;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Container {
    input: Vec<Point>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.w != other.w {
            return self.w.cmp(&other.w);
        }
        if self.z != other.z {
            return self.z.cmp(&other.z);
        }
        if self.y != other.y {
            return other.y.cmp(&self.y);
        }
        self.x.cmp(&other.x)
    }
}

impl Container {
    pub fn new() -> Self {
        Self { input: Vec::new() }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        input
            .trim()
            .lines()
            .enumerate()
            .try_for_each::<_, Result<(), String>>(|(y, line)| {
                line.trim()
                    .char_indices()
                    .try_for_each::<_, Result<(), String>>(|(x, chr)| {
                        match chr {
                            '.' => {}
                            '#' => {
                                self.input.push(Point {
                                    w: 0,
                                    z: 0,
                                    y: -(y as isize),
                                    x: x as isize,
                                });
                            }
                            _ => return Err(format!("invalid character in line: {}", chr)),
                        }
                        Ok(())
                    })
            })?;
        self.input.sort_unstable();
        Ok(())
    }

    fn part_1(&self) -> Result<String, String> {
        let mut state = self.input.clone();
        for _ in 0..6 {
            let ranges = state
                .iter()
                .fold(vec![(!0isize >> 1, !0isize); 4], |mut acc, point| {
                    if point.x < acc[0].0 {
                        acc[0].0 = point.x
                    }
                    if point.x > acc[0].1 {
                        acc[0].1 = point.x
                    }
                    if point.y < acc[1].0 {
                        acc[1].0 = point.y
                    }
                    if point.y > acc[1].1 {
                        acc[1].1 = point.y
                    }
                    if point.z < acc[2].0 {
                        acc[2].0 = point.z
                    }
                    if point.z > acc[2].1 {
                        acc[2].1 = point.z
                    }
                    acc
                });
            let mut delete_set: Vec<usize> = Vec::new();
            let mut add_set: Vec<Point> = Vec::new();

            // delete any active points with insufficient neighbours
            for (point_idx, point) in state.iter().enumerate() {
                let mut neighbours = 0;
                for x in point.x - 1..=point.x + 1 {
                    for y in point.y - 1..=point.y + 1 {
                        for z in point.z - 1..=point.z + 1 {
                            let new_point = Point { x, y, z, w: 0 };
                            if new_point.ne(point) && state.contains(&new_point) {
                                neighbours += 1;
                            }
                        }
                    }
                }
                if neighbours < 2 || neighbours > 3 {
                    delete_set.push(point_idx);
                }
            }

            // create any newly active points
            for o_x in ranges[0].0 - 1..=ranges[0].1 + 1 {
                for o_y in ranges[1].0 - 1..=ranges[1].1 + 1 {
                    for o_z in ranges[2].0 - 1..=ranges[2].1 + 1 {
                        let mut neighbours = 0;
                        for z in o_z - 1..=o_z + 1 {
                            for y in o_y - 1..=o_y + 1 {
                                for x in o_x - 1..=o_x + 1 {
                                    if (x, y, z) != (o_x, o_y, o_z)
                                        && state.contains(&Point { x, y, z, w: 0 })
                                    {
                                        neighbours += 1;
                                    }
                                }
                            }
                        }
                        if neighbours == 3 {
                            add_set.push(Point {
                                x: o_x,
                                y: o_y,
                                z: o_z,
                                w: 0,
                            });
                        }
                    }
                }
            }

            delete_set.sort_unstable();
            let mut i = 0;
            state.retain(|_| (!delete_set.contains(&i), i += 1).0);
            state.append(&mut add_set);
            state.sort_unstable();
            state.dedup();
        }
        Ok(state.iter().count().to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        let mut state = self.input.clone();
        for _ in 0..6 {
            let ranges = state
                .iter()
                .fold(vec![(!0isize >> 1, !0isize); 4], |mut acc, point| {
                    if point.x < acc[0].0 {
                        acc[0].0 = point.x
                    }
                    if point.x > acc[0].1 {
                        acc[0].1 = point.x
                    }
                    if point.y < acc[1].0 {
                        acc[1].0 = point.y
                    }
                    if point.y > acc[1].1 {
                        acc[1].1 = point.y
                    }
                    if point.z < acc[2].0 {
                        acc[2].0 = point.z
                    }
                    if point.z > acc[2].1 {
                        acc[2].1 = point.z
                    }
                    if point.w < acc[3].0 {
                        acc[3].0 = point.w
                    }
                    if point.w > acc[3].1 {
                        acc[3].1 = point.w
                    }
                    acc
                });
            let mut delete_set: Vec<usize> = Vec::new();
            let mut add_set: Vec<Point> = Vec::new();

            // delete any active points with insufficient neighbours
            for (point_idx, point) in state.iter().enumerate() {
                let mut neighbours = 0;
                for x in point.x - 1..=point.x + 1 {
                    for y in point.y - 1..=point.y + 1 {
                        for z in point.z - 1..=point.z + 1 {
                            for w in point.w - 1..=point.w + 1 {
                                let new_point = Point { x, y, z, w };
                                if new_point.ne(point) && state.contains(&new_point) {
                                    neighbours += 1;
                                }
                            }
                        }
                    }
                }
                if neighbours < 2 || neighbours > 3 {
                    delete_set.push(point_idx);
                }
            }

            // create any newly active points
            for o_x in ranges[0].0 - 1..=ranges[0].1 + 1 {
                for o_y in ranges[1].0 - 1..=ranges[1].1 + 1 {
                    for o_z in ranges[2].0 - 1..=ranges[2].1 + 1 {
                        for o_w in ranges[3].0 - 1..=ranges[3].1 + 1 {
                            let mut neighbours = 0;
                            for w in o_w - 1..=o_w + 1 {
                                for z in o_z - 1..=o_z + 1 {
                                    for y in o_y - 1..=o_y + 1 {
                                        for x in o_x - 1..=o_x + 1 {
                                            if (x, y, z, w) != (o_x, o_y, o_z, o_w)
                                                && state.contains(&Point { x, y, z, w })
                                            {
                                                neighbours += 1;
                                            }
                                        }
                                    }
                                }
                            }
                            if neighbours == 3 {
                                add_set.push(Point {
                                    x: o_x,
                                    y: o_y,
                                    z: o_z,
                                    w: o_w,
                                });
                            }
                        }
                    }
                }
            }

            delete_set.sort_unstable();
            let mut i = 0;
            state.retain(|_| (!delete_set.contains(&i), i += 1).0);
            state.append(&mut add_set);
            state.sort_unstable();
            state.dedup();
        }
        Ok(state.iter().count().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = ".#.
..#
###";

        let expected = Container {
            input: vec![
                Point {
                    w: 0,
                    z: 0,
                    y: 0,
                    x: 1,
                },
                Point {
                    w: 0,
                    z: 0,
                    y: -1,
                    x: 2,
                },
                Point {
                    w: 0,
                    z: 0,
                    y: -2,
                    x: 0,
                },
                Point {
                    w: 0,
                    z: 0,
                    y: -2,
                    x: 1,
                },
                Point {
                    w: 0,
                    z: 0,
                    y: -2,
                    x: 2,
                },
            ],
        };

        let mut cont = Container::new();

        assert_eq!(Ok(()), cont.parse_input(input));
        assert_eq!(expected, cont);
    }

    #[test]
    fn test_part_1_example() {
        let input = Container {
            input: vec![
                Point {
                    w: 0,
                    z: 0,
                    y: 0,
                    x: 1,
                },
                Point {
                    w: 0,
                    z: 0,
                    y: -1,
                    x: 2,
                },
                Point {
                    w: 0,
                    z: 0,
                    y: -2,
                    x: 0,
                },
                Point {
                    w: 0,
                    z: 0,
                    y: -2,
                    x: 1,
                },
                Point {
                    w: 0,
                    z: 0,
                    y: -2,
                    x: 2,
                },
            ],
        };

        let expected = 112.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_2_example() {
        let input = Container {
            input: vec![
                Point {
                    w: 0,
                    z: 0,
                    y: 0,
                    x: 1,
                },
                Point {
                    w: 0,
                    z: 0,
                    y: -1,
                    x: 2,
                },
                Point {
                    w: 0,
                    z: 0,
                    y: -2,
                    x: 0,
                },
                Point {
                    w: 0,
                    z: 0,
                    y: -2,
                    x: 1,
                },
                Point {
                    w: 0,
                    z: 0,
                    y: -2,
                    x: 2,
                },
            ],
        };

        let expected = 848.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }
}
