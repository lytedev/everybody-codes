use super::prelude::*;

pub struct Part1 {}
impl QuestCompleter<i64> for Part1 {
    fn solve(input: &str) -> i64 {
        666
    }
}

#[cfg(test)]
mod part1test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(Part1::solve("part1example"), 777)
    }
}

pub struct Part2 {}
impl QuestCompleter<i64> for Part2 {
    fn solve(input: &str) -> i64 {
        666
    }
}

#[cfg(test)]
mod part2test {
    use super::*;

    #[test]
    fn part2_example() {
        assert_eq!(Part2::solve("part2example"), 777)
    }
}

pub struct Part3 {}
impl QuestCompleter<i64> for Part3 {
    fn solve(input: &str) -> i64 {
        666
    }
}

#[cfg(test)]
mod part3test {
    use super::*;

    #[test]
    fn part3_example() {
        assert_eq!(Part3::solve("part3example"), 777)
    }
}
