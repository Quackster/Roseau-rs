use std::collections::HashMap;

pub struct ChatUtility;

impl ChatUtility {
    pub const HEARING_RADIUS_NORM: i32 = 5;
    pub const HEARING_RADIUS_MAX: i32 = 7;

    pub fn emotes(classic_only: bool) -> HashMap<&'static str, &'static str> {
        let mut emotes = HashMap::from([
            (":)", "sml"),
            (":-)", "sml"),
            (":d", "sml"),
            (":p", "sml"),
            (";)", "sml"),
            (";-)", "sml"),
            (":s", "sad"),
            (":(", "sad"),
            (":-(", "sad"),
            (":'(", "sad"),
            (":o", "srp"),
            (":@", "agr"),
            (">:(", "agr"),
        ]);

        if !classic_only {
            emotes.extend([
                (":]", "sml"),
                ("=)", "sml"),
                ("=]", "sml"),
                ("=d", "sml"),
                ("=(", "sad"),
                ("=[", "sad"),
                ("=o", "srp"),
            ]);
        }

        emotes
    }

    pub fn filter_words<'a>(words: impl IntoIterator<Item = &'a str>) -> Vec<&'a str> {
        words.into_iter().collect()
    }

    pub fn detect_emote<'a>(
        words: impl IntoIterator<Item = &'a str>,
        classic_only: bool,
    ) -> Option<&'static str> {
        let emotes = Self::emotes(classic_only);

        for word in words {
            if word.is_empty() {
                continue;
            }

            let Some(start) = word.chars().next() else {
                continue;
            };

            if !matches!(start, ':' | '>' | '=') {
                continue;
            }

            let word = word.to_ascii_lowercase();
            if let Some(emote) = emotes.get(word.as_str()) {
                return Some(*emote);
            }
        }

        None
    }

    pub fn garble_chat_with(
        text: &str,
        mut should_garble: impl FnMut(usize, char) -> bool,
    ) -> String {
        text.chars()
            .enumerate()
            .map(|(position, character)| {
                if character >= 'A' && character <= 'z' && should_garble(position, character) {
                    '.'
                } else {
                    character
                }
            })
            .collect()
    }
}

#[cfg(test)]
#[path = "chat_utility_tests.rs"]
mod tests;
