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
mod tests {
    use super::*;

    #[test]
    fn detects_classic_emotes_case_insensitively() {
        assert_eq!(
            ChatUtility::detect_emote(["hello", ":D"], true),
            Some("sml")
        );
        assert_eq!(
            ChatUtility::detect_emote(["hello", ">:("], true),
            Some("agr")
        );
    }

    #[test]
    fn respects_classic_only_emote_set() {
        assert_eq!(ChatUtility::detect_emote(["=]"], true), None);
        assert_eq!(ChatUtility::detect_emote(["=]"], false), Some("sml"));
    }

    #[test]
    fn garbles_only_java_letter_range_when_callback_allows() {
        let garbled = ChatUtility::garble_chat_with("Az[é 9", |_position, _character| true);

        assert_eq!(garbled, "...é 9");
    }

    #[test]
    fn filter_words_preserves_input_words() {
        assert_eq!(
            ChatUtility::filter_words(["one", "two"]),
            vec!["one", "two"]
        );
    }
}
