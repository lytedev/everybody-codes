mod event2024;
mod event2025;

fn main() {
    let mut args = std::env::args().skip(1);

    let quest = {
        let event = args
            .next()
            .expect("no 'event' command line argument provided")
            .trim()
            .to_string();

        let quest_id = args
            .next()
            .expect("no 'quest identifier' command line argument provided")
            .trim()
            .to_string();

        let part = args
            .next()
            .expect("no 'part' command line argument provided")
            .trim()
            .to_string();

        Quest::new(event, quest_id, part)
    };

    let input = quest.load_input();
    let result = match (quest.event.as_str(), quest.id.as_str(), quest.part.as_str()) {
        // TODO: ripe for a macro?
        ("2024", "1", "1") => event2024::quest1::Part1::solve(&input).to_string(),
        ("2024", "1", "2") => event2024::quest1::Part2::solve(&input).to_string(),
        ("2024", "1", "3") => event2024::quest1::Part3::solve(&input).to_string(),
        ("2024", "2", "1") => event2024::quest2::Part1::solve(&input).to_string(),
        ("2024", "2", "2") => event2024::quest2::Part2::solve(&input).to_string(),
        ("2024", "2", "3") => event2024::quest2::Part3::solve(&input).to_string(),
        ("2025", "1", "1") => event2025::quest1::Part1::solve(&input).to_string(),
        _ => panic!("no solution available for {}", quest),
    }
    .to_string();

    println!("{} Result: {}", quest, result);
}

pub struct Quest {
    pub id: String,
    pub event: String,
    pub part: String,
}

impl Quest {
    fn new(event: String, id: String, part: String) -> Quest {
        Quest { event, id, part }
    }

    fn load_input(&self) -> String {
        std::fs::read_to_string(format!(
            "input/{}/quest{}-part{}.txt",
            self.event, self.id, self.part
        ))
        .expect("failed to load input")
    }
}

impl std::fmt::Display for Quest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Event {}, Quest {}, Part {}",
            self.event, self.id, self.part
        )
    }
}

pub trait QuestCompleter<D: std::fmt::Display> {
    fn solve(input: &str) -> D;
}

mod prelude {
    pub use super::QuestCompleter;
}
