use crate::Day;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Container {
    input: Vec<String>,
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
        todo!()
    }

    fn part_1(&self) -> Result<String, String> {
        todo!()
    }

    fn part_2(&self) -> Result<String, String> {
        todo!()
    }
}