use std::fmt::{self, Display};

use super::Position;

pub const OPEN: i32 = 0;
pub const CLOSED: i32 = 1;

#[derive(Debug, Clone, PartialEq)]
pub struct RoomModel {
    name: String,
    height_map: String,
    square_chars: Vec<Vec<String>>,
    door_x: i32,
    door_y: i32,
    door_z: i32,
    door_rotation: i32,
    map_size_x: usize,
    map_size_y: usize,
    squares: Vec<Vec<i32>>,
    square_heights: Vec<Vec<f64>>,
    has_pool: bool,
    disabled_height_check: bool,
}

impl RoomModel {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: impl Into<String>,
        height_map: impl AsRef<str>,
        door_x: i32,
        door_y: i32,
        door_z: i32,
        door_rotation: i32,
        has_pool: bool,
        disabled_height_check: bool,
    ) -> Result<Self, ParseRoomModelError> {
        let name = name.into();
        let raw_height_map = height_map.as_ref();
        let rows = raw_height_map.split(' ').collect::<Vec<_>>();
        let first_row = rows.first().ok_or(ParseRoomModelError::EmptyHeightMap)?;
        let map_size_x = first_row.chars().count();
        let map_size_y = rows.len();

        if map_size_x == 0 || map_size_y == 0 {
            return Err(ParseRoomModelError::EmptyHeightMap);
        }

        let mut squares = vec![vec![CLOSED; map_size_y]; map_size_x];
        let mut square_heights = vec![vec![0.0; map_size_y]; map_size_x];
        let mut square_chars = vec![vec![String::new(); map_size_y]; map_size_x];

        for (y, row) in rows.iter().enumerate() {
            if row.chars().count() != map_size_x {
                return Err(ParseRoomModelError::JaggedHeightMap);
            }

            for (x, square) in row.chars().enumerate() {
                let square = square.to_ascii_lowercase().to_string();

                if square == "x" {
                    squares[x][y] = CLOSED;
                } else if square.parse::<i32>().is_ok() {
                    squares[x][y] = OPEN;
                    square_heights[x][y] = square
                        .parse::<f64>()
                        .map_err(|_| ParseRoomModelError::InvalidHeight)?;
                }

                if door_x == x as i32 && door_y == y as i32 {
                    squares[x][y] = OPEN;
                    square_heights[x][y] = door_z as f64;
                }

                square_chars[x][y] = square;

                if name == "pub_a" && ((x == 9 && y == 9) || (x == 11 && y == 1)) {
                    squares[x][y] = CLOSED;
                }

                if name == "pool_a" && x == 6 && (y == 31 || y == 32) {
                    squares[x][y] = CLOSED;
                }
            }
        }

        Ok(Self {
            name,
            height_map: raw_height_map
                .replace('\r', "")
                .replace('\n', "")
                .replace(' ', "\r"),
            square_chars,
            door_x,
            door_y,
            door_z,
            door_rotation,
            map_size_x,
            map_size_y,
            squares,
            square_heights,
            has_pool,
            disabled_height_check,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn height_map(&self) -> &str {
        &self.height_map
    }

    pub fn door_x(&self) -> i32 {
        self.door_x
    }

    pub fn door_y(&self) -> i32 {
        self.door_y
    }

    pub fn door_z(&self) -> i32 {
        self.door_z
    }

    pub fn door_rotation(&self) -> i32 {
        self.door_rotation
    }

    pub fn map_size_x(&self) -> usize {
        self.map_size_x
    }

    pub fn map_size_y(&self) -> usize {
        self.map_size_y
    }

    pub fn height_at_position(&self, position: Position) -> f64 {
        self.height(position.x(), position.y())
    }

    pub fn height(&self, x: i32, y: i32) -> f64 {
        if self.invalid_xy_coords(x, y) {
            0.0
        } else {
            self.square_heights[x as usize][y as usize]
        }
    }

    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        self.invalid_xy_coords(x, y) || self.squares[x as usize][y as usize] == CLOSED
    }

    pub fn invalid_xy_coords(&self, x: i32, y: i32) -> bool {
        x < 0 || y < 0 || x as usize >= self.map_size_x || y as usize >= self.map_size_y
    }

    pub fn square_chars(&self) -> &[Vec<String>] {
        &self.square_chars
    }

    pub fn door_position(&self) -> Position {
        Position::new(self.door_x, self.door_y, self.door_z as f64)
    }

    pub fn has_pool(&self) -> bool {
        self.has_pool
    }

    pub fn has_disabled_height_check(&self) -> bool {
        self.disabled_height_check
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseRoomModelError {
    EmptyHeightMap,
    JaggedHeightMap,
    InvalidHeight,
}

impl Display for ParseRoomModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyHeightMap => write!(f, "room model height map is empty"),
            Self::JaggedHeightMap => write!(f, "room model height map rows are uneven"),
            Self::InvalidHeight => write!(f, "room model square height is invalid"),
        }
    }
}

impl std::error::Error for ParseRoomModelError {}

#[cfg(test)]
#[path = "room_model_tests.rs"]
mod tests;
