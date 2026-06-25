use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
    z: f64,
    body_rotation: i32,
    head_rotation: i32,
}

impl Position {
    pub const fn new(x: i32, y: i32, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            body_rotation: 0,
            head_rotation: 0,
        }
    }

    pub const fn with_rotation(x: i32, y: i32, z: f64, rotation: i32) -> Self {
        Self {
            x,
            y,
            z,
            body_rotation: rotation,
            head_rotation: rotation,
        }
    }

    pub fn parse(value: &str) -> Result<Self, ParsePositionError> {
        let (x, y) = value
            .split_once(',')
            .ok_or(ParsePositionError::MissingDelimiter)?;

        Ok(Self::new(
            x.parse().map_err(|_| ParsePositionError::InvalidX)?,
            y.parse().map_err(|_| ParsePositionError::InvalidY)?,
            0.0,
        ))
    }

    pub fn parse_xyz(value: &str) -> Result<Self, ParsePositionError> {
        let mut parts = value.split(',');
        let x = parts.next().ok_or(ParsePositionError::MissingDelimiter)?;
        let y = parts.next().ok_or(ParsePositionError::MissingDelimiter)?;
        let z = parts.next().unwrap_or("0");

        Ok(Self::new(
            x.parse().map_err(|_| ParsePositionError::InvalidX)?,
            y.parse().map_err(|_| ParsePositionError::InvalidY)?,
            z.parse().map_err(|_| ParsePositionError::InvalidZ)?,
        ))
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn rotation(&self) -> i32 {
        self.body_rotation
    }

    pub fn head_rotation(&self) -> i32 {
        self.head_rotation
    }

    pub fn set_rotation(&mut self, rotation: i32) {
        self.body_rotation = rotation;
        self.head_rotation = rotation;
    }

    pub fn set_body_rotation(&mut self, rotation: i32) {
        self.body_rotation = rotation;
    }

    pub fn set_head_rotation(&mut self, rotation: i32) {
        self.head_rotation = rotation;
    }

    pub fn add(&self, other: Self) -> Self {
        Self::new(other.x + self.x, other.y + self.y, other.z + self.z)
    }

    pub fn subtract(&self, other: Self) -> Self {
        Self::new(other.x - self.x, other.y - self.y, other.z - self.z)
    }

    pub fn distance_squared(&self, point: Self) -> i32 {
        let dx = self.x - point.x;
        let dy = self.y - point.y;

        (dx * dx) + (dy * dy)
    }

    pub fn distance(&self, point: Self) -> i32 {
        f64::hypot((self.x - point.x) as f64, (self.y - point.y) as f64) as i32
    }

    pub fn is_match(&self, point: Self) -> bool {
        self.x == point.x && self.y == point.y
    }

    pub fn square_in_front(&self) -> Self {
        let mut square = Self::new(self.x, self.y, 0.0);

        match self.body_rotation {
            0 => square.y -= 1,
            2 => square.x += 1,
            4 => square.y += 1,
            6 => square.x -= 1,
            _ => {}
        }

        square
    }

    pub fn square_left(&self) -> Self {
        let mut square = Self::new(self.x, self.y, 0.0);

        match self.body_rotation {
            0 => square.x -= 1,
            2 => square.y += 1,
            4 => square.x += 1,
            6 => square.y += 1,
            _ => {}
        }

        square
    }

    pub fn square_right(&self) -> Self {
        let mut square = Self::new(self.x, self.y, 0.0);

        match self.body_rotation {
            0 => square.x += 1,
            2 => square.y -= 1,
            4 => square.x -= 1,
            6 => square.y -= 1,
            _ => {}
        }

        square
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new(0, 0, 0.0)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsePositionError {
    MissingDelimiter,
    InvalidX,
    InvalidY,
    InvalidZ,
}

impl Display for ParsePositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingDelimiter => write!(f, "position is missing comma delimiter"),
            Self::InvalidX => write!(f, "position x coordinate is invalid"),
            Self::InvalidY => write!(f, "position y coordinate is invalid"),
            Self::InvalidZ => write!(f, "position z coordinate is invalid"),
        }
    }
}

impl std::error::Error for ParsePositionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_comma_position_like_java_constructor() {
        let position = Position::parse("17,19").unwrap();

        assert_eq!(position.x(), 17);
        assert_eq!(position.y(), 19);
        assert_eq!(position.z(), 0.0);
        assert_eq!(position.to_string(), "17,19");
    }

    #[test]
    fn parses_three_part_position_for_pool_splash_body() {
        let position = Position::parse_xyz("17,18,0.5").unwrap();

        assert_eq!(position, Position::new(17, 18, 0.5));
        assert_eq!(
            Position::parse_xyz("17,18").unwrap(),
            Position::new(17, 18, 0.0)
        );
    }

    #[test]
    fn calculates_distance_like_java_position() {
        let a = Position::new(1, 1, 0.0);
        let b = Position::new(4, 5, 0.0);

        assert_eq!(a.distance_squared(b), 25);
        assert_eq!(a.distance(b), 5);
    }

    #[test]
    fn keeps_java_left_and_right_square_semantics() {
        let east = Position::with_rotation(10, 10, 0.0, 2);
        let west = Position::with_rotation(10, 10, 0.0, 6);

        assert_eq!(east.square_left(), Position::new(10, 11, 0.0));
        assert_eq!(east.square_right(), Position::new(10, 9, 0.0));
        assert_eq!(west.square_left(), Position::new(10, 11, 0.0));
        assert_eq!(west.square_right(), Position::new(10, 9, 0.0));
    }
}
