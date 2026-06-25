use super::chat_utility::*;

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
