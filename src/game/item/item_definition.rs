use super::ItemBehaviour;

#[derive(Debug, Clone, PartialEq)]
pub struct ItemDefinition {
    id: i32,
    sprite: String,
    color: String,
    length: i32,
    width: i32,
    height: f64,
    behaviour_flags: String,
    behaviour: ItemBehaviour,
    name: String,
    description: String,
    data_class: String,
}

impl ItemDefinition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        sprite: impl Into<String>,
        color: impl Into<String>,
        length: i32,
        width: i32,
        height: f64,
        behaviour_flags: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        data_class: impl Into<String>,
    ) -> Self {
        let behaviour_flags = behaviour_flags.into();
        let behaviour = ItemBehaviour::parse(&behaviour_flags);
        let height = if !behaviour.can_sit_on_top()
            && !behaviour.can_lay_on_top()
            && !behaviour.can_stack_on_top()
        {
            0.001
        } else {
            height
        };

        Self {
            id,
            sprite: sprite.into(),
            color: color.into(),
            length,
            width,
            height,
            behaviour_flags,
            behaviour,
            name: name.into(),
            description: description.into(),
            data_class: data_class.into(),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn sprite(&self) -> &str {
        &self.sprite
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn length(&self) -> i32 {
        self.length
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn with_height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    pub fn behaviour_flags(&self) -> &str {
        &self.behaviour_flags
    }

    pub fn behaviour(&self) -> &ItemBehaviour {
        &self.behaviour
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn data_class(&self) -> &str {
        &self.data_class
    }
}

#[cfg(test)]
#[path = "item_definition_tests.rs"]
mod tests;
