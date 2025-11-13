use crate::QuestCompleter;

pub struct Part1 {}
impl QuestCompleter for Part1 {
    fn solve(&self, input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str = r#"A=[25,9]"#;

    #[test]
    fn example1() {
        assert_eq!((Part1 {}).solve(EXAMPLE1), "[357,862]")
    }
}
