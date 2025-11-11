use crate::prelude::*;

pub fn num_potions(b: u8) -> u32 {
    match b {
        b'B' => 1,
        b'C' => 3,
        b'D' => 5,
        _ => 0,
    }
}

pub struct Part1 {}
impl QuestCompleter<u32> for Part1 {
    fn solve(input: &str) -> u32 {
        input.bytes().map(num_potions).sum()
    }
}

pub fn num_potions_pair(a: u8, b: u8) -> u32 {
    match (a, b) {
        (b'x', b) => num_potions(b),
        (a, b'x') => num_potions(a),
        (a, b) => num_potions(a) + num_potions(b) + 2,
    }
}

pub struct Part2 {}
impl QuestCompleter<u32> for Part2 {
    fn solve(input: &str) -> u32 {
        let mut bytes = input.bytes();
        let mut result = 0;
        while let (Some(a), Some(b)) = (bytes.next(), bytes.next()) {
            result += num_potions_pair(a, b);
        }
        result
    }
}

#[cfg(test)]
mod part2test {
    use super::*;

    #[test]
    fn part2_example() {
        assert_eq!(Part2::solve("AxBCDDCAxD"), 28)
    }
}

pub fn num_potions_triple(a: u8, b: u8, c: u8) -> u32 {
    match (a, b, c) {
        (b'x', b, c) => num_potions_pair(b, c),
        (a, b'x', c) => num_potions_pair(a, c),
        (a, b, b'x') => num_potions_pair(a, b),
        (a, b, c) => num_potions(a) + num_potions(b) + num_potions(c) + 6,
    }
}

pub struct Part3 {}
impl QuestCompleter<u32> for Part3 {
    fn solve(input: &str) -> u32 {
        let mut bytes = input.bytes();
        let mut result = 0;
        while let (Some(a), Some(b), Some(c)) = (bytes.next(), bytes.next(), bytes.next()) {
            result += num_potions_triple(a, b, c);
        }
        result
    }
}

#[cfg(test)]
mod part3test {
    use super::*;

    #[test]
    fn part3_example() {
        assert_eq!(Part3::solve("xBxAAABCDxCC"), 30)
    }
}
