#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogueItem {
    call_id: String,
    definition_id: i32,
    credits: i32,
    extra_data: Option<String>,
}

impl CatalogueItem {
    pub fn new(call_id: impl Into<String>, definition_id: i32, credits: i32) -> Self {
        Self {
            call_id: call_id.into(),
            definition_id,
            credits,
            extra_data: None,
        }
    }

    pub fn call_id(&self) -> &str {
        &self.call_id
    }

    pub fn definition_id(&self) -> i32 {
        self.definition_id
    }

    pub fn credits(&self) -> i32 {
        self.credits
    }

    pub fn extra_data(&self) -> Option<&str> {
        self.extra_data.as_deref()
    }

    pub fn with_extra_data(mut self, extra_data: impl Into<String>) -> Self {
        self.extra_data = Some(extra_data.into());
        self
    }

    pub fn set_extra_data(&mut self, extra_data: impl Into<String>) {
        self.extra_data = Some(extra_data.into());
    }
}

#[cfg(test)]
#[path = "catalogue_item_tests.rs"]
mod tests;
