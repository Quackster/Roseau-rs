use super::*;

#[test]
fn records_item_definition_reload_effect() {
    assert_eq!(
        ReloadDefinitionsCommand.handle(&CommandContext::new(), ":reloaddef"),
        vec![CommandEffect::ReloadItemDefinitions]
    );
}
