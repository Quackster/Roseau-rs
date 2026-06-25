use super::strip_info::*;

#[test]
fn composes_strip_info_packet() {
    let mut response = StripInfo::new([
        StripItem::new(
            1,
            "chair",
            "Chair",
            "red",
            StripItemKind::Stuff {
                length: 1,
                width: 2,
                color: "blue".to_owned(),
            },
        ),
        StripItem::new(
            2,
            "note",
            "Post-it",
            "1",
            StripItemKind::Item { post_it: true },
        ),
    ])
    .compose();

    assert_eq!(
        response.get(),
        "#STRIPINFO\rroseau;1;0;S;0;chair;Chair;red;1;2;blue/\rroseau;2;0;I;0;note;Post-it;2;2/##"
    );
}
