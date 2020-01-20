fn last_verse(word: &str) -> String {
    format!("And all for the want of a {}.", word)
}

pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        1 => last_verse(list[0]),
        _ => {
            let verses = (0..list.len() - 1)
                .map(|i| format!("For want of a {} the {} was lost.", list[i], list[i + 1]))
                .collect::<Vec<String>>()
                .join("\n");

            format!("{}\n{}", verses, last_verse(list[0]))
        }
    }
}
