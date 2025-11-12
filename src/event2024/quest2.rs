use std::collections::HashSet;

use crate::prelude::*;

struct RunicNote<'a> {
    needles: Vec<&'a str>,
    haystack_lines: Vec<&'a str>,
}

impl<'a> RunicNote<'a> {
    fn parse(input: &'a str) -> Self {
        let mut needles = (&input[6..input.find('\n').unwrap()])
            .split(",")
            .collect::<Vec<&'a str>>();
        needles.sort_by(|a, b| std::cmp::Ord::cmp(&b.len(), &a.len()));
        Self {
            needles,
            haystack_lines: (&input[input.find('\n').unwrap() + 2..]).lines().collect(),
        }
    }
}

pub struct Part1 {}
impl QuestCompleter for Part1 {
    fn solve(&self, input: &str) -> String {
        let RunicNote {
            needles,
            haystack_lines,
        } = RunicNote::parse(input);
        let mut result = 0;
        for n in 0..haystack_lines[0].len() {
            for needle in &needles {
                if needle.len() >= (haystack_lines[0].len() - n) {
                    // If the remainder of the haystack_lines[0] we're searching doesn't have enough characters, we skip on
                    continue;
                }
                if &haystack_lines[0][n..n + needle.len()] == *needle {
                    result += 1
                }
            }
        }
        result.to_string()
    }
}

#[cfg(test)]
mod part1test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            Part1::solve(
                &Part1 {},
                "WORDS:THE,OWE,MES,ROD,HER\n\nAWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE"
            ),
            "4"
        )
    }
}

pub struct Part2 {}
impl QuestCompleter for Part2 {
    fn solve(&self, input: &str) -> String {
        let RunicNote {
            needles,
            haystack_lines,
        } = RunicNote::parse(input);
        let mut runic_symbols: HashSet<(usize, usize)> = HashSet::new();
        for line_index in 0..haystack_lines.len() {
            let line = haystack_lines[line_index];
            for i in 0..line.len() {
                for needle in &needles {
                    let l = &line[0..=i];
                    if l.ends_with(needle) {
                        let rrange = i - (needle.len() - 1)..=i;
                        for ii in rrange {
                            runic_symbols.insert((line_index, ii));
                        }
                    }
                    let r = &line[line.len() - i..];
                    if r.starts_with(&needle.chars().rev().collect::<String>()) {
                        let rrange = (line.len() - i)..((line.len() - i) + needle.len());
                        for ii in rrange {
                            runic_symbols.insert((line_index, ii));
                        }
                    }
                }
            }
        }
        runic_symbols.len().to_string()
    }
}

#[cfg(test)]
mod part2test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            Part2::solve(
                &Part2 {},
                "WORDS:THE,OWE,MES,ROD,HER,QAQ

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END
QAQAQ"
            ),
            "42"
        )
    }
}

pub struct Part3 {}
impl QuestCompleter for Part3 {
    fn solve(&self, _input: &str) -> String {
        666.to_string()
    }
}

// #[cfg(test)]
// mod part3test {
//     use super::*;

//     #[test]
//     fn example() {
//         assert_eq!(Part3::solve("part3example"), 777)
//     }
// }
