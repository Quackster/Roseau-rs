use std::collections::HashMap;

use super::ItemDefinition;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemManager {
    definitions: HashMap<i32, ItemDefinition>,
}

impl ItemManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_definitions(definitions: impl IntoIterator<Item = ItemDefinition>) -> Self {
        Self {
            definitions: definitions
                .into_iter()
                .map(|definition| (definition.id(), definition))
                .collect(),
        }
    }

    pub fn load_definitions(&mut self, definitions: impl IntoIterator<Item = ItemDefinition>) {
        self.definitions = definitions
            .into_iter()
            .map(|definition| (definition.id(), definition))
            .collect();
    }

    pub fn clear_definitions(&mut self) {
        self.definitions.clear();
    }

    pub fn get_definition(&self, definition_id: i32) -> Option<&ItemDefinition> {
        self.definitions.get(&definition_id)
    }

    pub fn definitions(&self) -> &HashMap<i32, ItemDefinition> {
        &self.definitions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn definition(id: i32) -> ItemDefinition {
        ItemDefinition::new(id, "chair", "red", 1, 1, 1.0, "SFC", "Chair", "", "")
    }

    #[test]
    fn loads_and_replaces_definitions() {
        let mut manager = ItemManager::with_definitions([definition(1)]);
        assert!(manager.get_definition(1).is_some());

        manager.load_definitions([definition(2)]);

        assert!(manager.get_definition(1).is_none());
        assert_eq!(manager.get_definition(2).unwrap().sprite(), "chair");
    }
}
