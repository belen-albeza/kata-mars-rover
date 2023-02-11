use clap::ValueEnum;
use std::fmt;

use crate::commands::{Movable, Opcode};

pub type Point = (i32, i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn normal(&self) -> Point {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }
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

    pub fn run(&self, _: &[Opcode]) {}
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

impl Movable for Rover {
    fn advance(&mut self, dir: i32) {
        let delta = (
            self.direction.normal().0 * dir,
            self.direction.normal().1 * dir,
        );

        self.position.0 += delta.0;
        self.position.1 += delta.1;
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

    #[test]
    fn test_advances_forward_when_facing_north() {
        let mut r = Rover::new((0, 5), Direction::North);
        r.advance(1);
        assert_eq!(r.position, (0, 4));
    }

    #[test]
    fn test_advances_forward_when_facing_east() {
        let mut r = Rover::new((0, 0), Direction::East);
        r.advance(1);
        assert_eq!(r.position, (1, 0));
    }

    #[test]
    fn test_advances_forward_when_facing_south() {
        let mut r = Rover::new((0, 0), Direction::South);
        r.advance(1);
        assert_eq!(r.position, (0, 1));
    }

    #[test]
    fn test_advances_forward_when_facing_west() {
        let mut r = Rover::new((5, 0), Direction::West);
        r.advance(1);
        assert_eq!(r.position, (4, 0));
    }
}
