use super::prelude::*;

struct RunicNote<'a> {
    needles: Vec<&'a str>,
    haystack: &'a str,
}

impl<'a> RunicNote<'a> {
    fn parse(input: &'a str) -> Self {
        Self {
            needles: (&input[6..input.find('\n').unwrap()]).split(",").collect(),
            haystack: &input[input.find('\n').unwrap() + 2..],
        }
    }
}

pub struct Part1 {}
impl QuestCompleter<i64> for Part1 {
    fn solve(input: &str) -> i64 {
        let RunicNote { needles, haystack } = RunicNote::parse(input);
        let mut result = 0;
        for n in 0..haystack.len() {
            for needle in &needles {
                if needle.len() >= (haystack.len() - n) {
                    // If the remainder of the haystack we're searching doesn't have enough characters, we skip on
                    continue;
                }
                if &haystack[n..n + needle.len()] == *needle {
                    result += 1
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod part1test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            Part1::solve(
                "WORDS:THE,OWE,MES,ROD,HER\n\nAWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE"
            ),
            4
        )
    }
}

pub struct Part2 {}
impl QuestCompleter<i64> for Part2 {
    fn solve(input: &str) -> i64 {
        let RunicNote { needles, haystack } = RunicNote::parse(input);
        let mut result = 0;
        for n in 0..haystack.len() {
            for needle in &needles {
                if needle.len() >= (haystack.len() - n) {
                    // If the remainder of the haystack we're searching doesn't have enough characters, we skip on
                    continue;
                }
                if &haystack[n..n + needle.len()] == *needle {
                    result += needle.len() as i64
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod part2test {
    use super::*;

    // #[test]
    fn example() {
        assert_eq!(
            Part2::solve(
                "WORDS:THE,OWE,MES,ROD,HER,QAQ

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END
QAQAQ"
            ),
            42
        )
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

    // #[test]
    fn example() {
        assert_eq!(Part3::solve("part3example"), 777)
    }
}
