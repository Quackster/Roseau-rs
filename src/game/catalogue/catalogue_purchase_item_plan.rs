#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CataloguePurchaseItemPlan {
    definition_id: i32,
    extra_data: String,
    teleporter_pair: bool,
}

impl CataloguePurchaseItemPlan {
    pub fn new(definition_id: i32, extra_data: impl Into<String>, teleporter_pair: bool) -> Self {
        Self {
            definition_id,
            extra_data: extra_data.into(),
            teleporter_pair,
        }
    }

    pub fn definition_id(&self) -> i32 {
        self.definition_id
    }

    pub fn extra_data(&self) -> &str {
        &self.extra_data
    }

    pub fn is_teleporter_pair(&self) -> bool {
        self.teleporter_pair
    }
}
