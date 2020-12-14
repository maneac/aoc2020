use crate::Day;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Container {
    target: usize,
    buses: Vec<usize>,
    minimum: usize,
}

impl Container {
    pub fn new() -> Self {
        Self {
            target: 0,
            buses: Vec::new(),
            minimum: 0,
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        self.minimum = 100000000000000;
        let mut lines = input.trim().lines();
        self.target = lines
            .next()
            .ok_or_else(|| "no target time".to_owned())?
            .trim()
            .parse::<usize>()
            .map_err(|e| format!("unable to parse target time as usize: {}", &e))?;

        lines
            .next()
            .ok_or_else(|| "no buses".to_owned())?
            .trim()
            .split(',')
            .try_for_each::<_, Result<(), String>>(|bus| {
                if bus == "x" {
                    self.buses.push(0);
                    return Ok(());
                }
                let round_trip_time = bus
                    .parse::<usize>()
                    .map_err(|e| format!("unable to parse bus as usize: {}", &e))?;
                self.buses.push(round_trip_time);
                Ok(())
            })
    }

    fn part_1(&self) -> Result<String, String> {
        let minimal_wait = self
            .buses
            .iter()
            .enumerate()
            .filter_map(|(idx, bus)| {
                if bus == &0 {
                    None
                } else {
                    Some((idx, bus - (self.target % bus)))
                }
            })
            .min_by(|a, b| a.1.cmp(&b.1))
            .ok_or_else(|| "unable to find shortest wait time".to_owned())?;
        Ok((self.buses[minimal_wait.0] * minimal_wait.1).to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        let mut step_size = self.buses[0] as u64;
        let mut bus_idx = 1;

        let mut target: u64 = 0;

        while bus_idx < self.buses.len() {
            let bus = self.buses[bus_idx] as u64;

            if bus == 0 {
                bus_idx += 1;
                continue;
            }

            target += step_size;

            if (target + bus_idx as u64) % bus == 0 {
                step_size *= bus;
                bus_idx += 1;
            }
        }
        Ok(target.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "939
7,13,x,x,59,x,31,19";

        let expected = Container {
            target: 939,
            buses: vec![7, 13, 0, 0, 59, 0, 31, 19],
            minimum: 100000000000000,
        };

        let mut cont = Container::new();

        assert_eq!(Ok(()), cont.parse_input(input));
        assert_eq!(expected, cont);
    }

    #[test]
    fn test_part_1_example() {
        let input = Container {
            target: 939,
            buses: vec![7, 13, 0, 0, 59, 0, 31, 19],
            minimum: 0,
        };

        let expected = 295.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_2_examples() {
        let tests = vec![
            (
                "part 1 example",
                Container {
                    target: 939,
                    buses: vec![7, 13, 0, 0, 59, 0, 31, 19],
                    minimum: 0,
                },
                1068781,
            ),
            (
                "bullet 1",
                Container {
                    target: 0,
                    buses: vec![17, 0, 13, 19],
                    minimum: 0,
                },
                3417,
            ),
            (
                "bullet 2",
                Container {
                    target: 0,
                    buses: vec![67, 7, 59, 61],
                    minimum: 0,
                },
                754018,
            ),
            (
                "bullet 3",
                Container {
                    target: 0,
                    buses: vec![67, 0, 7, 59, 61],
                    minimum: 0,
                },
                779210,
            ),
            (
                "bullet 4",
                Container {
                    target: 0,
                    buses: vec![67, 7, 0, 59, 61],
                    minimum: 0,
                },
                1261476,
            ),
            (
                "bullet 5",
                Container {
                    target: 0,
                    buses: vec![1789, 37, 47, 1889],
                    minimum: 0,
                },
                1202161486,
            ),
        ];

        for test in tests.iter() {
            assert_eq!(
                Ok(test.2.to_string()),
                test.1.part_2(),
                "  test: {}",
                test.0
            );
        }
    }
}
