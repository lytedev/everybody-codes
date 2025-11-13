use std::{num::ParseIntError, str::FromStr};

use crate::QuestCompleter;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

enum DirectionParseError {
    Empty,
    Invalid(char),
}

impl std::fmt::Debug for DirectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionParseError::Empty => write!(f, "failed to parse empty string"),
            DirectionParseError::Invalid(s) => write!(f, "failed to parse invalid '{s}'"),
        }
    }
}

impl FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().ok_or(DirectionParseError::Empty)? {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            c => Err(DirectionParseError::Invalid(c)),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    magnitude: usize,
}

enum InstructionParseError {
    Direction(DirectionParseError),
    Magnitude(ParseIntError),
}

impl std::fmt::Debug for InstructionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionParseError::Direction(d) => write!(f, "failed to parse Direction: {d:?}"),
            InstructionParseError::Magnitude(m) => write!(f, "failed to parse Magnitude: {m:?}"),
        }
    }
}

impl From<DirectionParseError> for InstructionParseError {
    fn from(value: DirectionParseError) -> Self {
        Self::Direction(value)
    }
}

impl From<ParseIntError> for InstructionParseError {
    fn from(value: ParseIntError) -> Self {
        Self::Magnitude(value)
    }
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            direction: Direction::from_str(s)?,
            magnitude: usize::from_str(&s[1..])?,
        })
    }
}

#[derive(Debug)]
struct Eggshell {
    names: Vec<String>,
    instructions: Vec<Instruction>,
}

enum EggshellParseError {
    CouldNotSplit,
    Instruction(InstructionParseError),
}

impl std::fmt::Debug for EggshellParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EggshellParseError::CouldNotSplit => {
                write!(f, "failed to split names line from instructions line")
            }
            EggshellParseError::Instruction(i) => write!(f, "failed to parse Instruction: {i:?}"),
        }
    }
}

impl From<InstructionParseError> for EggshellParseError {
    fn from(value: InstructionParseError) -> Self {
        Self::Instruction(value)
    }
}

impl FromStr for Eggshell {
    type Err = EggshellParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_names, raw_instructions) = s
            .split_once("\n\n")
            .ok_or(EggshellParseError::CouldNotSplit)?;

        Ok(Self {
            names: raw_names.split(",").map(String::from).collect(),
            instructions: raw_instructions
                .split(",")
                .map(Instruction::from_str)
                .collect::<Result<Vec<Instruction>, _>>()?,
        })
    }
}

impl Eggshell {
    fn interpret_name<'a>(&'a self) -> &'a str {
        let mut location: usize = 0;
        for instruction in self.instructions.iter() {
            location = match instruction.direction {
                Direction::Left => location.saturating_sub(instruction.magnitude),
                Direction::Right => location.saturating_add(instruction.magnitude),
            }
            .clamp(0, self.names.len() - 1);
        }
        return &self.names[location];
    }

    fn interpret_ring_name<'a>(&'a self) -> &'a str {
        let mut location: usize = 0;
        for instruction in self.instructions.iter() {
            location = match instruction.direction {
                Direction::Left => location.overflowing_sub(instruction.magnitude).0,
                Direction::Right => location.overflowing_add(instruction.magnitude).0,
            }
            .rem_euclid(self.names.len());
        }
        return &self.names[location];
    }

    fn interpret_swap_ring_name<'a>(&'a mut self) -> &'a str {
        for instruction in self.instructions.iter() {
            let swap_location = match instruction.direction {
                Direction::Left => {
                    (self.names.len() * 100)
                        .overflowing_sub(instruction.magnitude)
                        .0
                }
                Direction::Right => {
                    (self.names.len() * 100)
                        .overflowing_add(instruction.magnitude)
                        .0
                }
            }
            .rem_euclid(self.names.len());
            self.names.swap(0, swap_location);
        }
        return &self.names[0];
    }
}

pub struct Part1 {}
impl QuestCompleter for Part1 {
    fn solve(&self, input: &str) -> String {
        Eggshell::from_str(input)
            .unwrap()
            .interpret_name()
            .to_string()
    }
}

pub struct Part2 {}
impl QuestCompleter for Part2 {
    fn solve(&self, input: &str) -> String {
        Eggshell::from_str(input)
            .unwrap()
            .interpret_ring_name()
            .to_string()
    }
}

pub struct Part3 {}
impl QuestCompleter for Part3 {
    fn solve(&self, input: &str) -> String {
        Eggshell::from_str(input)
            .unwrap()
            .interpret_swap_ring_name()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1"#;

    #[test]
    fn example_p1() {
        let shell = EXAMPLE.parse::<Eggshell>();
        assert!(shell.is_ok());
        let name = shell.unwrap().interpret_name().to_string();
        assert_eq!(name, "Fyrryn");
    }

    #[test]
    fn example_p2() {
        let shell = EXAMPLE.parse::<Eggshell>();
        assert!(shell.is_ok());
        let name = shell.unwrap().interpret_ring_name().to_string();
        assert_eq!(name, "Elarzris");
    }

    const EXAMPLE2: &str = r#"Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L3"#;

    #[test]
    fn example_p3() {
        let shell = EXAMPLE2.parse::<Eggshell>();
        assert!(shell.is_ok());
        let name = shell.unwrap().interpret_swap_ring_name().to_string();
        assert_eq!(name, "Drakzyph");
    }
}
