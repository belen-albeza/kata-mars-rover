use std::fmt;

use clap::ValueEnum;
pub type Point = (i32, i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::North => "N",
                Self::East => "E",
                Self::South => "S",
                Self::West => "W",
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rover {
    position: Point,
    direction: Direction,
}

impl Rover {
    pub fn new(position: Point, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}

impl fmt::Display for Rover {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}) {}",
            self.position.0, self.position.1, self.direction
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_rover() {
        let r = Rover::new((1, 2), Direction::East);
        assert_eq!(r.position, (1, 2));
        assert_eq!(r.direction, Direction::East);
    }

    #[test]
    fn test_display_shows_position_and_dir() {
        let r = Rover::new((1, 2), Direction::East);
        let display = format!("{}", r);

        assert_eq!(display, "(1, 2) E");
    }
}
