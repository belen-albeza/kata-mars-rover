use clap::ValueEnum;
use std::fmt;

use crate::commands::{Movable, Opcode};

pub type Point = (i32, i32);

pub type Map = (i32, i32);

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

    pub fn turn(&self, dir: i32) -> Self {
        let directions = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        let current_index = directions.iter().position(|x| x == self).unwrap();
        let index = (current_index as i32 + dir.signum()).rem_euclid(directions.len() as i32);

        directions[index as usize]
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
    map: Map,
}

impl Rover {
    pub fn new(position: Point, direction: Direction, map: Map) -> Self {
        Self {
            position,
            direction,
            map,
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

        self.position.0 = (self.position.0 + delta.0).rem_euclid(self.map.0);
        self.position.1 = (self.position.1 + delta.1).rem_euclid(self.map.1);
    }

    fn turn(&mut self, dir: i32) {
        self.direction = self.direction.turn(dir);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn any_map() -> Map {
        (10, 10)
    }

    #[test]
    fn test_new_rover() {
        let r = Rover::new((1, 2), Direction::East, any_map());
        assert_eq!(r.position, (1, 2));
        assert_eq!(r.direction, Direction::East);
    }

    #[test]
    fn test_display_shows_position_and_dir() {
        let r = Rover::new((1, 2), Direction::East, any_map());
        let display = format!("{}", r);

        assert_eq!(display, "(1, 2) E");
    }

    #[test]
    fn test_advances_forward_when_facing_north() {
        let mut r = Rover::new((0, 5), Direction::North, any_map());
        r.advance(1);
        assert_eq!(r.position, (0, 4));
    }

    #[test]
    fn test_advances_forward_when_facing_east() {
        let mut r = Rover::new((0, 0), Direction::East, any_map());
        r.advance(1);
        assert_eq!(r.position, (1, 0));
    }

    #[test]
    fn test_advances_forward_when_facing_south() {
        let mut r = Rover::new((0, 0), Direction::South, any_map());
        r.advance(1);
        assert_eq!(r.position, (0, 1));
    }

    #[test]
    fn test_advances_forward_when_facing_west() {
        let mut r = Rover::new((5, 0), Direction::West, any_map());
        r.advance(1);
        assert_eq!(r.position, (4, 0));
    }

    #[test]
    fn test_advances_backwards_when_facing_north() {
        let mut r = Rover::new((0, 0), Direction::North, any_map());
        r.advance(-1);
        assert_eq!(r.position, (0, 1));
    }

    #[test]
    fn test_advances_backwards_when_facing_east() {
        let mut r = Rover::new((5, 0), Direction::East, any_map());
        r.advance(-1);
        assert_eq!(r.position, (4, 0));
    }

    #[test]
    fn test_advances_backwards_when_facing_south() {
        let mut r = Rover::new((0, 5), Direction::South, any_map());
        r.advance(-1);
        assert_eq!(r.position, (0, 4));
    }

    #[test]
    fn test_advances_backwards_when_facing_west() {
        let mut r = Rover::new((0, 0), Direction::West, any_map());
        r.advance(-1);
        assert_eq!(r.position, (1, 0));
    }

    #[test]
    fn test_turns_left_when_facing_north() {
        let mut r = Rover::new((0, 0), Direction::North, any_map());
        r.turn(-1);
        assert_eq!(r.direction, Direction::West);
    }

    #[test]
    fn test_turns_left_when_facing_east() {
        let mut r = Rover::new((0, 0), Direction::East, any_map());
        r.turn(-1);
        assert_eq!(r.direction, Direction::North);
    }

    #[test]
    fn test_turns_left_when_facing_south() {
        let mut r = Rover::new((0, 0), Direction::South, any_map());
        r.turn(-1);
        assert_eq!(r.direction, Direction::East);
    }

    #[test]
    fn test_turns_left_when_facing_west() {
        let mut r = Rover::new((0, 0), Direction::West, any_map());
        r.turn(-1);
        assert_eq!(r.direction, Direction::South);
    }

    #[test]
    fn test_turns_right_when_facing_north() {
        let mut r = Rover::new((0, 0), Direction::North, any_map());
        r.turn(1);
        assert_eq!(r.direction, Direction::East);
    }

    #[test]
    fn test_turns_right_when_facing_east() {
        let mut r = Rover::new((0, 0), Direction::East, any_map());
        r.turn(1);
        assert_eq!(r.direction, Direction::South);
    }

    #[test]
    fn test_turns_right_when_facing_south() {
        let mut r = Rover::new((0, 0), Direction::South, any_map());
        r.turn(1);
        assert_eq!(r.direction, Direction::West);
    }

    #[test]
    fn test_turns_right_when_facing_west() {
        let mut r = Rover::new((0, 0), Direction::West, any_map());
        r.turn(1);
        assert_eq!(r.direction, Direction::North);
    }

    #[test]
    fn test_wraps_around_top_edge() {
        let mut r = Rover::new((0, 0), Direction::North, (10, 10));
        r.advance(1);
        assert_eq!(r.position, (0, 9));
    }

    #[test]
    fn test_move_wraps_around_right_edge() {
        let mut r = Rover::new((9, 0), Direction::East, (10, 10));
        r.advance(1);
        assert_eq!(r.position, (0, 0));
    }

    #[test]
    fn test_move_wraps_around_bottom_edge() {
        let mut r = Rover::new((0, 9), Direction::South, (10, 10));
        r.advance(1);
        assert_eq!(r.position, (0, 0));
    }

    #[test]
    fn test_wraps_around_left_edge() {
        let mut r = Rover::new((0, 0), Direction::West, (10, 10));
        r.advance(1);
        assert_eq!(r.position, (9, 0));
    }
}
