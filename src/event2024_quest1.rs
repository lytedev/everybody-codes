use super::prelude::*;

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
            println!("result: {result} {} {}", a as char, b as char);
            result += num_potions_pair(a, b);
        }
        result
    }
}

mod part2test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn part2_example() {
        assert_eq!(Part2::solve("AxBCDDCAxD"), 28)
    }
}
