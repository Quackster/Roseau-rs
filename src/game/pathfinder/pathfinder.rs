use crate::game::room::model::Position;

use super::PathfinderNode;

pub const MOVE_POINTS: [Position; 8] = [
    Position::new(0, -1, 0.0),
    Position::new(0, 1, 0.0),
    Position::new(1, 0, 0.0),
    Position::new(-1, 0, 0.0),
    Position::new(1, -1, 0.0),
    Position::new(-1, 1, 0.0),
    Position::new(1, 1, 0.0),
    Position::new(-1, -1, 0.0),
];

pub fn make_path(
    start: Position,
    end: Position,
    map_size_x: usize,
    map_size_y: usize,
    mut is_valid_step: impl FnMut(Position, Position, bool) -> bool,
) -> Vec<Position> {
    let Some(mut node) = make_path_reversed(start, end, map_size_x, map_size_y, &mut is_valid_step)
    else {
        return Vec::new();
    };

    let mut squares = Vec::new();
    while let Some(next_node) = node.next_node() {
        squares.push(Position::new(node.position().x(), node.position().y(), 0.0));
        node = next_node.clone();
    }

    squares.reverse();
    squares
}

pub fn make_path_reversed(
    start: Position,
    end: Position,
    map_size_x: usize,
    map_size_y: usize,
    is_valid_step: &mut impl FnMut(Position, Position, bool) -> bool,
) -> Option<PathfinderNode> {
    if invalid_coords(start, map_size_x, map_size_y) || invalid_coords(end, map_size_x, map_size_y)
    {
        return None;
    }

    let mut map = vec![vec![None::<NodeState>; map_size_y]; map_size_x];
    let mut open_list = Vec::new();

    let mut current = NodeState::new(start);
    current.cost = 0;
    current.in_open = true;
    map[start.x() as usize][start.y() as usize] = Some(current.clone());
    open_list.push(start);

    while !open_list.is_empty() {
        let current_position = poll_lowest_cost(&mut open_list, &map)?;
        let current = {
            let state = map[current_position.x() as usize][current_position.y() as usize]
                .as_mut()
                .expect("open list only contains initialized nodes");
            state.in_closed = true;
            state.clone()
        };

        for move_point in MOVE_POINTS {
            let tmp = current.position.add(move_point);

            if invalid_coords(tmp, map_size_x, map_size_y) {
                continue;
            }

            let is_final_move = tmp.x() == end.x() && tmp.y() == end.y();
            if !is_valid_step(current.position, tmp, is_final_move) {
                continue;
            }

            if map[tmp.x() as usize][tmp.y() as usize].is_none() {
                map[tmp.x() as usize][tmp.y() as usize] = Some(NodeState::new(tmp));
            }

            let mut should_return = false;
            {
                let node = map[tmp.x() as usize][tmp.y() as usize]
                    .as_mut()
                    .expect("node initialized above");

                if node.in_closed {
                    continue;
                }

                let mut diff = 0;
                if current.position.x() != node.position.x() {
                    diff += 1;
                }
                if current.position.y() != node.position.y() {
                    diff += 1;
                }

                let cost = current.cost + diff + node.position.distance_squared(end);
                if cost < node.cost {
                    node.cost = cost;
                    node.next_position = Some(current.position);
                }

                if !node.in_open {
                    if node.position.x() == end.x() && node.position.y() == end.y() {
                        node.next_position = Some(current.position);
                        should_return = true;
                    } else {
                        node.in_open = true;
                        open_list.push(node.position);
                    }
                }
            }

            if should_return {
                return Some(build_pathfinder_node(end, &map));
            }
        }
    }

    None
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct NodeState {
    position: Position,
    next_position: Option<Position>,
    cost: i32,
    in_open: bool,
    in_closed: bool,
}

impl NodeState {
    fn new(position: Position) -> Self {
        Self {
            position,
            next_position: None,
            cost: i32::MAX,
            in_open: false,
            in_closed: false,
        }
    }
}

fn invalid_coords(position: Position, map_size_x: usize, map_size_y: usize) -> bool {
    position.x() < 0
        || position.y() < 0
        || position.x() as usize >= map_size_x
        || position.y() as usize >= map_size_y
}

fn poll_lowest_cost(
    open_list: &mut Vec<Position>,
    map: &[Vec<Option<NodeState>>],
) -> Option<Position> {
    let (index, _) = open_list.iter().enumerate().min_by_key(|(_, position)| {
        map[position.x() as usize][position.y() as usize]
            .as_ref()
            .map(|node| node.cost)
            .unwrap_or(i32::MAX)
    })?;

    Some(open_list.remove(index))
}

fn build_pathfinder_node(position: Position, map: &[Vec<Option<NodeState>>]) -> PathfinderNode {
    let state = map[position.x() as usize][position.y() as usize].expect("path node must exist");
    let mut node = PathfinderNode::new(state.position);
    node.set_cost(state.cost);
    node.set_in_open(state.in_open);
    node.set_in_closed(state.in_closed);

    if let Some(next_position) = state.next_position {
        node.set_next_node(Some(build_pathfinder_node(next_position, map)));
    }

    node
}

#[cfg(test)]
#[path = "pathfinder_tests.rs"]
mod tests;
