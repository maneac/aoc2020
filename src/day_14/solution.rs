use std::collections::HashMap;

use crate::Day;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Container {
    instructions: Vec<Instruction>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Instruction {
    Mask(Mask),
    Memory(MemIdx),
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Mask {
    zeroes: u64,
    ones: u64,
}

impl Mask {
    fn new() -> Self {
        Self {
            zeroes: !0,
            ones: 0,
        }
    }

    fn floating_indices(&self, raw_idx: u64) -> Vec<u64> {
        let floats = !(!self.zeroes | self.ones) & !(!0 << 36);
        if floats.count_ones() == 0 {
            return vec![raw_idx];
        }

        let mut output = Vec::<u64>::with_capacity(1 << floats.count_ones());
        output.push(raw_idx | self.ones);

        for i in 0..36 {
            if (1 << i) & floats == 0 {
                continue;
            }

            let out_len = output.len();
            output = output.repeat(2);

            output.iter_mut().enumerate().for_each(|(idx, entry)| {
                if idx < out_len {
                    *entry &= !(1 << i);
                } else {
                    *entry |= 1 << i;
                }
            });
        }
        output
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct MemIdx {
    index: u64,
    value: u64,
}

impl Day for Container {
    fn parse_input(&mut self, input: &str) -> Result<(), String> {
        input.trim().lines().try_for_each(|line| {
            let mut line_parts = line.trim().split(" = ");
            let instr = line_parts
                .next()
                .ok_or_else(|| "empty line parsed".to_owned())?;

            if instr == "mask" {
                self.instructions.push(Instruction::Mask(
                    line_parts
                        .next()
                        .ok_or_else(|| "no mask specified".to_owned())?
                        .chars()
                        .rev()
                        .enumerate()
                        .try_fold::<_, _, Result<Mask, String>>(
                            Mask::new(),
                            |mut acc, (idx, chr)| {
                                match chr {
                                    'X' => {}
                                    '1' => acc.ones |= 1 << idx,
                                    '0' => acc.zeroes &= !(1 << idx),
                                    _ => return Err(format!("invalid character in mask: {}", chr)),
                                }
                                Ok(acc)
                            },
                        )?,
                ));
                return Ok(());
            }

            if instr.starts_with("mem[") {
                let index = instr
                    .split('[')
                    .nth(1)
                    .ok_or_else(|| "no index for memory instruction".to_owned())?
                    .trim_end_matches(']')
                    .parse::<u64>()
                    .map_err(|e| format!("failed to parse memory index as u64: {}", e))?;

                let value = line_parts
                    .next()
                    .ok_or_else(|| "no value for memory index".to_owned())?
                    .parse::<u64>()
                    .map_err(|e| format!("failed to parse memory value as u64: {}", e))?;

                self.instructions
                    .push(Instruction::Memory(MemIdx { index, value }));
                return Ok(());
            }

            Err(format!("invalid instruction: {}", instr))
        })
    }

    fn part_1(&self) -> Result<String, String> {
        let mut mask = &Mask::new();
        let mut memory: HashMap<u64, u64> = HashMap::new();

        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Mask(m) => {
                    mask = m;
                }
                Instruction::Memory(midx) => {
                    memory.insert(midx.index, midx.value & mask.zeroes | mask.ones);
                }
            }
        }

        Ok(memory.values().sum::<u64>().to_string())
    }

    fn part_2(&self) -> Result<String, String> {
        let mut mask = &Mask::new();
        let mut memory: HashMap<u64, u64> = HashMap::new();

        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Mask(m) => {
                    mask = m;
                }
                Instruction::Memory(midx) => {
                    let indices = mask.floating_indices(midx.index);
                    for i in indices.iter() {
                        memory.insert(*i, midx.value);
                    }
                }
            }
        }

        Ok(memory.values().sum::<u64>().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        let expected = Container {
            instructions: vec![
                Instruction::Mask(Mask {
                    zeroes: !(1 << 1),
                    ones: 1 << 6,
                }),
                Instruction::Memory(MemIdx {
                    index: 8,
                    value: 11,
                }),
                Instruction::Memory(MemIdx {
                    index: 7,
                    value: 101,
                }),
                Instruction::Memory(MemIdx { index: 8, value: 0 }),
            ],
        };

        let mut cont = Container::new();

        assert_eq!(Ok(()), cont.parse_input(input));
        assert_eq!(expected, cont);
    }

    #[test]
    fn test_part_1_example() {
        let input = Container {
            instructions: vec![
                Instruction::Mask(Mask {
                    zeroes: !(1 << 1),
                    ones: 1 << 6,
                }),
                Instruction::Memory(MemIdx {
                    index: 8,
                    value: 11,
                }),
                Instruction::Memory(MemIdx {
                    index: 7,
                    value: 101,
                }),
                Instruction::Memory(MemIdx { index: 8, value: 0 }),
            ],
        };

        let expected = 165.to_string();

        assert_eq!(Ok(expected), input.part_1());
    }

    #[test]
    fn test_part_2_example() {
        let input = Container {
            instructions: vec![
                Instruction::Mask(Mask {
                    zeroes: 1 << 0 | 1 << 1 | 1 << 4 | 1 << 5,
                    ones: 1 << 1 | 1 << 4,
                }),
                Instruction::Memory(MemIdx {
                    index: 42,
                    value: 100,
                }),
                Instruction::Mask(Mask {
                    zeroes: 1 << 0 | 1 << 1 | 1 << 3,
                    ones: 0,
                }),
                Instruction::Memory(MemIdx {
                    index: 26,
                    value: 1,
                }),
            ],
        };

        let expected = 208.to_string();

        assert_eq!(Ok(expected), input.part_2());
    }
}
