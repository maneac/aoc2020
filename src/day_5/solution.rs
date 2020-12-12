use crate::Day;

pub struct Container {
    input: Vec<u16>,
}

impl Container {
    pub fn new() -> Self {
        Self { input: Vec::new() }
    }
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        for line in input.trim().lines() {
            self.input.push(
                line.trim()
                    .char_indices()
                    .fold(0u16, |mut acc, (idx, chr)| {
                        if chr == 'R' || chr == 'B' {
                            acc |= 1 << (line.len() - 1 - idx);
                        }
                        acc
                    }),
            );
        }
        self.input.sort_unstable();
        Ok(())
    }

    fn part_1(&self) -> Result<String, String> {
        match self.input.iter().next_back() {
            Some(val) => Ok(val.to_string()),
            None => Err("no value found".to_string()),
        }
    }

    fn part_2(&self) -> Result<String, String> {
        let mut prev = self.input[0] - 1;
        for seat in self.input.iter() {
            if *seat != (prev + 1) {
                return Ok((prev + 1).to_string());
            }
            prev = *seat;
        }
        Err("no seat found".to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

        let expected = vec![119, 567, 820];

        let mut cont = Container::new();
        assert_eq!(Ok(()), cont.parse_input(input));
        assert_eq!(expected, cont.input);
    }

    #[test]
    fn test_part_1_example() {
        let input = Container {
            input: vec![119, 567, 820],
        };

        let expected = 820.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_2() {
        let input = Container {
            input: vec![4, 5, 7],
        };

        let expected = 6.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }
}
