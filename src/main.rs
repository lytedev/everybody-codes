fn main() {
    let mut args = std::env::args().skip(1);

    let quest = {
        let event = args
            .next()
            .expect("no event command line argument provided")
            .trim()
            .to_string();

        let quest_id = args
            .next()
            .expect("no quest identifier command line argument provided")
            .trim()
            .to_string();

        Quest::new(event, quest_id)
    };

    let result: String = match quest.event.as_str() {
        "2024" => match quest.id.as_str() {
            "1" => event2024_quest1::Completer::solve(&quest),
            _ => {
                panic!("unknown quest: {}", &quest.id);
            }
        },
        _ => {
            panic!("unknown event: {}", &quest.event);
        }
    };

    println!("{} Result: {}", quest, result);
}

pub struct Quest {
    pub id: String,
    pub event: String,
    pub input: String,
}

impl Quest {
    fn new(event: String, id: String) -> Quest {
        let input = std::fs::read_to_string(format!("input/{}-{}.txt", event, id))
            .expect("failed to load input");
        Quest { event, id, input }
    }
}

impl std::fmt::Display for Quest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event {} Quest {}", self.event, self.id)
    }
}

pub trait QuestCompleter {
    fn solve(quest: &Quest) -> String;
}

mod prelude {
    pub use super::{Quest, QuestCompleter};
}

mod event2024_quest1;
