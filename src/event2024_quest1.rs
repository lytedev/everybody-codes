use super::prelude::*;

pub fn num_potions(b: u8) -> u32 {
    match b {
        b'B' => 1,
        b'C' => 3,
        _ => 0,
    }
}

pub fn num_potions_pair(a: u8, b: u8) -> u32 {
    match (a, b) {
        (b'x', b) => num_potions(b),
        (a, b'x') => num_potions(a),
        (a, b) => num_potions(a) + num_potions(b) + 2,
    }
}

pub struct Part1 {}
impl QuestCompleter for Part1 {
    fn solve(quest: &Quest) -> u32 {
        quest.input.bytes().map(num_potions).sum::<u32>()
    }
}

pub struct Part2 {}
impl QuestCompleter for Part2 {
    fn solve(quest: &Quest) -> u32 {
        quest.input.bytes()[..]
            .chunks_exact(2)
            .map(|c| (c[0], c[1]))
            .map(num_potions_pair)
            .sum::<u32>()
            .to_string()
    }
}
