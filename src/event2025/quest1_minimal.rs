use crate::QuestCompleter;

pub struct Part1 {}
impl QuestCompleter for Part1 {
    fn solve(&self, input: &str) -> String {
        let (names, instructions) = input.split_once("\n\n").expect("malformed input");
        let names: Vec<_> = names.trim().split(",").collect();
        return names[instructions.split(",").fold(0usize, |i, n| {
            let mag = (&n[1..]).parse().unwrap();
            match &n[0..1] {
                "L" => i.saturating_sub(mag),
                _ => i.saturating_add(mag),
            }
            .clamp(0, names.len() - 1)
        })]
        .to_string();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            Part1::solve(
                &Part1 {},
                r#"Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1"#
            ),
            "Fyrryn"
        );
    }
}
