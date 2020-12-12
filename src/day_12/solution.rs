use crate::Day;

pub struct Container {
    actions: Vec<Action>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Action {
    Shift(Compass),
    Rotate(Rotate),
    Forward(usize),
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Compass {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Rotate {
    Left,
    Right,
    About,
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        input.trim().lines().try_for_each(|line| {
            let instruction_parts = line.trim().split_at(1);
            let instr_num = match instruction_parts.1.parse::<usize>() {
                Ok(val) => val,
                Err(e) => {
                    return Err(format!(
                        "failed to parse instruction {}: {}",
                        line,
                        e.to_string()
                    ))
                }
            };

            match instruction_parts.0 {
                "N" => self.actions.push(Action::Shift(Compass::North(instr_num))),
                "S" => self.actions.push(Action::Shift(Compass::South(instr_num))),
                "E" => self.actions.push(Action::Shift(Compass::East(instr_num))),
                "W" => self.actions.push(Action::Shift(Compass::West(instr_num))),
                "L" => match instr_num {
                    90 => self.actions.push(Action::Rotate(Rotate::Left)),
                    180 => self.actions.push(Action::Rotate(Rotate::About)),
                    270 => self.actions.push(Action::Rotate(Rotate::Right)),
                    _ => return Err(format!("invalid L rotation value: {}", instr_num)),
                },
                "R" => match instr_num {
                    90 => self.actions.push(Action::Rotate(Rotate::Right)),
                    180 => self.actions.push(Action::Rotate(Rotate::About)),
                    270 => self.actions.push(Action::Rotate(Rotate::Left)),
                    _ => return Err(format!("invalid R rotation value: {}", instr_num)),
                },
                "F" => self.actions.push(Action::Forward(instr_num)),
                _ => {
                    return Err(format!(
                        "invalid instruction character: {}",
                        instruction_parts.0
                    ))
                }
            };

            Ok(())
        })
    }

    fn part_1(&self) -> Result<String, String> {
        let mut location: (isize, isize) = (0, 0);
        let mut facing = Compass::East(0);

        self.actions.iter().for_each(|action| match action {
            Action::Forward(distance) => match facing {
                Compass::North(_) => location.1 += *distance as isize,
                Compass::South(_) => location.1 -= *distance as isize,
                Compass::East(_) => location.0 += *distance as isize,
                Compass::West(_) => location.0 -= *distance as isize,
            },
            Action::Shift(direction) => match direction {
                Compass::North(distance) => location.1 += *distance as isize,
                Compass::South(distance) => location.1 -= *distance as isize,
                Compass::East(distance) => location.0 += *distance as isize,
                Compass::West(distance) => location.0 -= *distance as isize,
            },
            Action::Rotate(rotation) => match facing {
                Compass::North(_) => match rotation {
                    Rotate::Left => facing = Compass::West(0),
                    Rotate::Right => facing = Compass::East(0),
                    Rotate::About => facing = Compass::South(0),
                },
                Compass::South(_) => match rotation {
                    Rotate::Left => facing = Compass::East(0),
                    Rotate::Right => facing = Compass::West(0),
                    Rotate::About => facing = Compass::North(0),
                },
                Compass::East(_) => match rotation {
                    Rotate::Left => facing = Compass::North(0),
                    Rotate::Right => facing = Compass::South(0),
                    Rotate::About => facing = Compass::West(0),
                },
                Compass::West(_) => match rotation {
                    Rotate::Left => facing = Compass::South(0),
                    Rotate::Right => facing = Compass::North(0),
                    Rotate::About => facing = Compass::East(0),
                },
            },
        });

        Ok((location.0.abs() + location.1.abs()).to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        let mut ship_location: (isize, isize) = (0, 0);
        let mut waypoint_location: (isize, isize) = (10, 1);

        self.actions.iter().for_each(|action| match action {
            Action::Forward(repetitions) => {
                for _ in 0..*repetitions {
                    ship_location.0 += waypoint_location.0;
                    ship_location.1 += waypoint_location.1;
                }
            }
            Action::Shift(direction) => match direction {
                Compass::North(distance) => waypoint_location.1 += *distance as isize,
                Compass::South(distance) => waypoint_location.1 -= *distance as isize,
                Compass::East(distance) => waypoint_location.0 += *distance as isize,
                Compass::West(distance) => waypoint_location.0 -= *distance as isize,
            },
            Action::Rotate(rotation) => match rotation {
                Rotate::Left => {
                    let swp = waypoint_location.0;
                    waypoint_location.0 = -waypoint_location.1;
                    waypoint_location.1 = swp;
                }
                Rotate::Right => {
                    let swp = waypoint_location.1;
                    waypoint_location.1 = -waypoint_location.0;
                    waypoint_location.0 = swp;
                }
                Rotate::About => {
                    waypoint_location.0 = -waypoint_location.0;
                    waypoint_location.1 = -waypoint_location.1;
                }
            },
        });

        Ok((ship_location.0.abs() + ship_location.1.abs()).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example_input() {
        let input = "F10
N3
F7
R90
F11";

        let expected = vec![
            Action::Forward(10),
            Action::Shift(Compass::North(3)),
            Action::Forward(7),
            Action::Rotate(Rotate::Right),
            Action::Forward(11),
        ];

        let mut cont = Container::new();

        assert_eq!(Ok(()), cont.parse_input(input));
        assert_eq!(expected, cont.actions);
    }

    #[test]
    fn test_parse_input_extended() {
        let input = "F10
        N3
        F7
        R90
        F11
        L270
        W5
        S3
        E19
        R180";

        let expected = vec![
            Action::Forward(10),
            Action::Shift(Compass::North(3)),
            Action::Forward(7),
            Action::Rotate(Rotate::Right),
            Action::Forward(11),
            Action::Rotate(Rotate::Right),
            Action::Shift(Compass::West(5)),
            Action::Shift(Compass::South(3)),
            Action::Shift(Compass::East(19)),
            Action::Rotate(Rotate::About),
        ];

        let mut cont = Container::new();

        assert_eq!(Ok(()), cont.parse_input(input));
        assert_eq!(expected, cont.actions);
    }

    #[test]
    fn test_part_1_example() {
        let input = Container {
            actions: vec![
                Action::Forward(10),
                Action::Shift(Compass::North(3)),
                Action::Forward(7),
                Action::Rotate(Rotate::Right),
                Action::Forward(11),
            ],
        };

        let expected = 25.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_1() {
        let tests = vec![
            (
                "forward no rotate",
                Container {
                    actions: vec![Action::Forward(10)],
                },
                Ok(10.to_string()),
            ),
            (
                "shift, rotate left circle, forward",
                Container {
                    actions: vec![
                        Action::Shift(Compass::West(1)),
                        Action::Rotate(Rotate::Left),
                        Action::Rotate(Rotate::Left),
                        Action::Rotate(Rotate::Left),
                        Action::Rotate(Rotate::Left),
                        Action::Forward(1),
                    ],
                },
                Ok(0.to_string()),
            ),
            (
                "forward, rotate left, forward",
                Container {
                    actions: vec![
                        Action::Forward(1),
                        Action::Rotate(Rotate::Left),
                        Action::Forward(10),
                    ],
                },
                Ok(11.to_string()),
            ),
            (
                "forward, rotate right, forward",
                Container {
                    actions: vec![
                        Action::Forward(2),
                        Action::Rotate(Rotate::Right),
                        Action::Forward(10),
                    ],
                },
                Ok(12.to_string()),
            ),
            (
                "forward, rotate about, forward",
                Container {
                    actions: vec![
                        Action::Forward(9),
                        Action::Rotate(Rotate::About),
                        Action::Forward(10),
                    ],
                },
                Ok(1.to_string()),
            ),
            (
                "forward, shift north",
                Container {
                    actions: vec![Action::Forward(1), Action::Shift(Compass::North(5))],
                },
                Ok(6.to_string()),
            ),
            (
                "forward, shift south",
                Container {
                    actions: vec![Action::Forward(1), Action::Shift(Compass::South(7))],
                },
                Ok(8.to_string()),
            ),
            (
                "forward, shift east",
                Container {
                    actions: vec![Action::Forward(5), Action::Shift(Compass::East(5))],
                },
                Ok(10.to_string()),
            ),
            (
                "forward, shift west",
                Container {
                    actions: vec![Action::Forward(3), Action::Shift(Compass::West(2))],
                },
                Ok(1.to_string()),
            ),
        ];

        for test in tests.iter() {
            assert_eq!(test.2, test.1.part_1(), "\n  test: {}", test.0);
        }
    }

    #[test]
    fn test_part_2_example() {
        let input = Container {
            actions: vec![
                Action::Forward(10),
                Action::Shift(Compass::North(3)),
                Action::Forward(7),
                Action::Rotate(Rotate::Right),
                Action::Forward(11),
            ],
        };

        let expected = 286.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }

    #[test]
    fn test_part_2() {
        let input = Container {
            actions: vec![
                // (0, 0), (10, 1)
                Action::Forward(10),
                // (100, 10), (10, 1)
                Action::Shift(Compass::North(3)),
                // (100, 10), (10, 4)
                Action::Forward(7),
                // (170, 38), (10, 4)
                Action::Rotate(Rotate::Right),
                // (170, 38), (4, -10)
                Action::Forward(11),
                // (214, -72), (4, -10)
                Action::Rotate(Rotate::Right),
                // (214, -72), (-10, -4)
                Action::Shift(Compass::West(5)),
                // (214, -72), (-15, -4)
                Action::Shift(Compass::South(3)),
                // (214, -72), (-15, -7)
                Action::Shift(Compass::East(19)),
                // (214, -72), (4, -7)
                Action::Rotate(Rotate::About),
                // (214, -72), (-4, 7)
                Action::Forward(10),
                // (174, -2), (-4, 7)
            ],
        };

        let expected = 176.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }
}
