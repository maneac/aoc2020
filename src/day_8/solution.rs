use crate::Day;

pub struct Container {
    input: Vec<Instruction>,
}

impl Container {
    pub fn new() -> Self {
        Self { input: Vec::new() }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Instruction {
    ACC(isize),
    JMP(isize),
    NOP(isize),
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        for line in input.trim().lines() {
            let chunks = line.split(' ').collect::<Vec<&str>>();
            let num = match chunks[1].parse::<isize>() {
                Ok(i) => i,
                Err(e) => {
                    return Err(e.to_string());
                }
            };
            match chunks[0] {
                "acc" => {
                    self.input.push(Instruction::ACC(num));
                }
                "jmp" => {
                    self.input.push(Instruction::JMP(num));
                }
                "nop" => {
                    self.input.push(Instruction::NOP(num));
                }
                _ => {
                    return Err("unknown instruction".to_string());
                }
            };
        }
        Ok(())
    }

    fn part_1(&self) -> Result<String, String> {
        let mut visited = vec![];
        let mut idx = 0isize;
        let mut acc = 0;
        while !visited.contains(&idx) {
            visited.push(idx);
            match self.input[idx as usize] {
                Instruction::NOP(_) => {}
                Instruction::ACC(i) => {
                    acc += i;
                }
                Instruction::JMP(i) => {
                    idx += i - 1;
                }
            };
            idx += 1;
        }
        Ok(acc.to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        for (alter_idx, instr) in self.input.iter().enumerate() {
            if let Instruction::ACC(_i) = instr {
                continue;
            }
            let mut visited = vec![];
            let mut idx = 0isize;
            let mut acc = 0;
            while !visited.contains(&idx) {
                visited.push(idx);
                if idx as usize >= self.input.len() {
                    return Ok(acc.to_string());
                }
                match self.input[idx as usize] {
                    Instruction::NOP(i) => {
                        if idx == alter_idx as isize {
                            idx += i - 1;
                        }
                    }
                    Instruction::ACC(i) => {
                        acc += i;
                    }
                    Instruction::JMP(i) => {
                        if idx != alter_idx as isize {
                            idx += i - 1;
                        }
                    }
                };
                idx += 1;
            }
        }
        Err("no change broke the loop".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_example() -> Vec<Instruction> {
        vec![
            Instruction::NOP(0),
            Instruction::ACC(1),
            Instruction::JMP(4),
            Instruction::ACC(3),
            Instruction::JMP(-3),
            Instruction::ACC(-99),
            Instruction::ACC(1),
            Instruction::JMP(-4),
            Instruction::ACC(6),
        ]
    }

    #[test]
    fn test_parse_input() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let expected = make_example();

        let mut cont = Container::new();

        assert_eq!(Ok(()), cont.parse_input(input));

        assert_eq!(expected, cont.input);
    }

    #[test]
    fn test_part_1_example() {
        let input = Container {
            input: make_example(),
        };

        let expected = 5.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_2_example() {
        let input = Container {
            input: make_example(),
        };

        let expected = 8.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }
}
