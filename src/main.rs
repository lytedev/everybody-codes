use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};

use macros::quest_map;

mod event2024;
mod event2025;

pub type Key = (&'static str, &'static str, &'static str);

fn mapping() -> HashMap<Key, Box<dyn Fn() -> Box<dyn QuestCompleter>>> {
    quest_map! {
        "2024", "1", "1",
        "2024", "1", "2",
        "2024", "1", "3",
        "2024", "2", "1",
        "2024", "2", "2",
        "2025", "1", "1",
        "2025", "1", "2",
        "2025", "1", "3",
        "2025", "2", "1",
    }
}
fn main() {
    let mut args = std::env::args().skip(1);

    let event = args.next().map(|s| s.trim().to_string());
    let quest_id = args.next().map(|s| s.trim().to_string());
    let part = args.next().map(|s| s.trim().to_string());

    if let (Some(event), Some(quest_id), Some(part)) = (&event, &quest_id, &part) {
        let quest = Quest::new(event, quest_id, part);

        let map = mapping();
        let dispatcher = map
            .get(&(quest.event.as_str(), quest.id.as_str(), quest.part.as_str()))
            .expect("no solver for the specified quest");
        let result = dispatcher().solve(&quest.load_input());

        println!("{} Result: {}", quest, result);
    } else if let Some(e) = &event {
        match e.as_str() {
            "--all" => {
                for ((e, q, p), f) in mapping() {
                    let quest = Quest::new(e, q, p);
                    let result = f().solve(&quest.load_input());
                    println!("{} Result: {}", quest, result);
                }
            }
            _ => panic!("unknown event"),
        }
    }
}

pub struct Quest {
    pub id: String,
    pub event: String,
    pub part: String,
}

impl Quest {
    pub fn new<E: AsRef<str>, I: AsRef<str>, P: AsRef<str>>(event: E, id: I, part: P) -> Quest {
        Quest {
            event: event.as_ref().to_string(),
            id: id.as_ref().to_string(),
            part: part.as_ref().to_string(),
        }
    }

    fn input_path(&self) -> PathBuf {
        Self::cache_dir()
            .join(&self.event)
            .join(&self.id)
            .join("input")
            .join(&self.part)
            .with_extension("txt")
    }

    fn load_input(&self) -> String {
        let mut has_downloaded = false;
        loop {
            match File::open(self.input_path()) {
                Ok(mut f) => {
                    let mut s = String::new();
                    if File::read_to_string(&mut f, &mut s).is_err() || s.is_empty() {
                        // remove malformed file to force redownload
                        // TODO: could loop forever?
                        let _ = std::fs::remove_file(self.input_path());
                        continue;
                    }
                    return s;
                }
                Err(e) => {
                    eprintln!("{}: {e}", self.input_path().to_string_lossy());
                }
            }
            if has_downloaded {
                panic!("failed to download inputs")
            }
            has_downloaded = true;
            Command::new("nix")
                .args(["run", ".#download-input", "--", &self.event, &self.id])
                .status()
                .expect("failed to download inputs");
        }
    }

    pub fn cache_dir() -> PathBuf {
        if let Ok(cache_dir) = std::env::var("XDG_CACHE_HOME") {
            PathBuf::from(cache_dir)
        } else {
            Path::new(&std::env::var("HOME").expect("HOME not set")).join(".cache")
        }
        .join("everybody-codes")
    }
}

#[cfg(test)]
mod test {
    use crate::Quest;

    #[test]
    fn quest_path_as_expected() {
        assert_eq!(
            Quest::new("2024", "1", "1").input_path(),
            Quest::cache_dir().join("2024/1/input/1.txt")
        );
        assert!(Quest::new("2024", "1", "1")
            .input_path()
            .ends_with("everybody-codes/2024/1/input/1.txt"));
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

pub trait QuestCompleter {
    fn solve(&self, input: &str) -> String;
}

mod prelude {
    pub use super::QuestCompleter;
}
