use std::{num::ParseIntError, str::FromStr};

use crate::QuestCompleter;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum DirectionParseError {
    Empty,
    Invalid(char),
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

#[derive(Debug)]
enum InstructionParseError {
    Direction(DirectionParseError),
    Magnitude(ParseIntError),
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

#[derive(Debug)]
enum EggshellParseError {
    Malformed,
    Instruction(InstructionParseError),
}

impl From<InstructionParseError> for EggshellParseError {
    fn from(value: InstructionParseError) -> Self {
        Self::Instruction(value)
    }
}

impl FromStr for Eggshell {
    type Err = EggshellParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_names, raw_instructions) =
            s.split_once("\n\n").ok_or(EggshellParseError::Malformed)?;

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
            };
            location = location.clamp(0, self.names.len() - 1);
        }
        return &self.names[location];
    }
}

pub struct Part1 {}
impl QuestCompleter<String> for Part1 {
    fn solve(input: &str) -> String {
        Eggshell::from_str(input)
            .unwrap()
            .interpret_name()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let shell = r#"Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1"#
            .parse::<Eggshell>();
        assert!(shell.is_ok());
        println!("{shell:?}");
        let name = shell.unwrap().interpret_name().to_string();
        println!("{name}");
        assert_eq!(name, "Fyrryn");
    }
}
