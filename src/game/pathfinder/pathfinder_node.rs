use crate::game::room::model::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct PathfinderNode {
    position: Position,
    next_node: Option<Box<PathfinderNode>>,
    cost: i32,
    in_open: bool,
    in_closed: bool,
}

impl PathfinderNode {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            next_node: None,
            cost: i32::MAX,
            in_open: false,
            in_closed: false,
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn next_node(&self) -> Option<&PathfinderNode> {
        self.next_node.as_deref()
    }

    pub fn set_next_node(&mut self, next_node: Option<PathfinderNode>) {
        self.next_node = next_node.map(Box::new);
    }

    pub fn cost(&self) -> i32 {
        self.cost
    }

    pub fn set_cost(&mut self, cost: i32) {
        self.cost = cost;
    }

    pub fn is_in_open(&self) -> bool {
        self.in_open
    }

    pub fn set_in_open(&mut self, in_open: bool) {
        self.in_open = in_open;
    }

    pub fn is_in_closed(&self) -> bool {
        self.in_closed
    }

    pub fn set_in_closed(&mut self, in_closed: bool) {
        self.in_closed = in_closed;
    }

    pub fn equals_position(&self, other: &Self) -> bool {
        other.position.is_match(self.position)
    }
}

#[cfg(test)]
#[path = "pathfinder_node_tests.rs"]
mod tests;
