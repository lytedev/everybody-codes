use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn quest_map(input: TokenStream) -> TokenStream {
    let mut entries = Vec::new();
    let mut iter = input.into_iter();

    // Parse entries: "year", "quest", "part",
    while let Some(token) = iter.next() {
        // Expect a string literal for year
        let year = match token {
            TokenTree::Literal(lit) => lit,
            _ => continue,
        };

        // Expect comma
        if !matches!(iter.next(), Some(TokenTree::Punct(p)) if p.as_char() == ',') {
            continue;
        }

        // Expect a string literal for quest
        let quest = match iter.next() {
            Some(TokenTree::Literal(lit)) => lit,
            _ => continue,
        };

        // Expect comma
        if !matches!(iter.next(), Some(TokenTree::Punct(p)) if p.as_char() == ',') {
            continue;
        }

        // Expect a string literal for part
        let part = match iter.next() {
            Some(TokenTree::Literal(lit)) => lit,
            _ => continue,
        };

        // Optional trailing comma
        let _ = iter.next();

        entries.push((year, quest, part));
    }

    // Generate the output
    let mut output = String::new();
    output.push_str("{ fn make_solver<T: QuestCompleter + 'static>(f: impl Fn() -> T + 'static) -> Box<dyn Fn() -> Box<dyn QuestCompleter>> { Box::new(move || Box::new(f())) } HashMap::from([");

    for (year, quest, part) in entries {
        let year_str = year.to_string();
        let quest_str = quest.to_string();
        let part_str = part.to_string();

        // Remove quotes from literals
        let year_val = year_str.trim_matches('"');
        let quest_val = quest_str.trim_matches('"');
        let part_val = part_str.trim_matches('"');

        output.push_str(&format!(
            "(({}, {}, {}), make_solver(|| event{}::quest{}::Part{} {{}})),",
            year_str, quest_str, part_str, year_val, quest_val, part_val
        ));
    }

    output.push_str("]) }");

    output.parse().unwrap()
}
