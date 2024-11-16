use super::prelude::*;

pub struct Completer {}

impl QuestCompleter for Completer {
    fn solve(quest: &Quest) -> String {
        let mut potions_needed = 0;
        for c in quest.input.chars() {
            potions_needed += match c {
                'B' => 1,
                'C' => 3,
                _ => 0,
            }
        }
        potions_needed.to_string()
    }
}
