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
