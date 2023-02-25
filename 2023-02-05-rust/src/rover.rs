use clap::ValueEnum;
use std::fmt;

use crate::commands::{Movable, Opcode};

pub type Point = (i32, i32);

pub trait RoverMap: fmt::Debug {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn has_obstacle_at(&self, x: i32, y: i32) -> bool;
}

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

#[derive(Debug, Clone, Copy)]
pub struct Rover<'a> {
    position: Point,
    direction: Direction,
    map: &'a dyn RoverMap,
}

impl<'a> Rover<'a> {
    pub fn new(position: Point, direction: Direction, map: &'a impl RoverMap) -> Self {
        Self {
            position,
            direction,
            map,
        }
    }

    pub fn run(&self, _: &[Opcode]) {}
}

impl<'a> fmt::Display for Rover<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}) {}",
            self.position.0, self.position.1, self.direction
        )
    }
}

impl<'a> Movable for Rover<'a> {
    fn advance(&mut self, dir: i32) -> Result<(), String> {
        let delta = (
            self.direction.normal().0 * dir,
            self.direction.normal().1 * dir,
        );

        let x = (self.position.0 + delta.0).rem_euclid(self.map.width() as i32);
        let y = (self.position.1 + delta.1).rem_euclid(self.map.height() as i32);

        if self.map.has_obstacle_at(x, y) {
            Err(format!("Found obstacle at ({}, {})", x, y))
        } else {
            self.position = (x, y);
            Ok(())
        }
    }

    fn turn(&mut self, dir: i32) {
        self.direction = self.direction.turn(dir);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    mock! {
        #[derive(Debug)]
        MarsMap {}
        impl RoverMap for MarsMap {
            fn width(&self) -> usize;
            fn height(&self) -> usize;
            fn has_obstacle_at(&self, x: i32, y: i32) -> bool;
        }
    }

    fn any_map() -> impl RoverMap {
        any_map_with_size(10, 10)
    }

    fn any_map_with_size(width: usize, height: usize) -> impl RoverMap {
        let mut map = MockMarsMap::new();
        map.expect_width().return_const(width);
        map.expect_height().return_const(height);
        map.expect_has_obstacle_at().return_const(false);
        map
    }

    fn any_map_with_obstacle_at(x: usize, y: usize) -> impl RoverMap {
        let mut map = MockMarsMap::new();
        map.expect_width().return_const(std::cmp::max(10, x));
        map.expect_height().return_const(std::cmp::max(10, y));
        map.expect_has_obstacle_at()
            .with(eq(x as i32), eq(y as i32))
            .return_const(true);
        map.expect_has_obstacle_at().return_const(false);
        map
    }

    #[test]
    fn test_new_rover() {
        let map = any_map();
        let r = Rover::new((1, 2), Direction::East, &map);
        assert_eq!(r.position, (1, 2));
        assert_eq!(r.direction, Direction::East);
    }

    #[test]
    fn test_display_shows_position_and_dir() {
        let map = any_map();
        let r = Rover::new((1, 2), Direction::East, &map);
        let display = format!("{}", r);

        assert_eq!(display, "(1, 2) E");
    }

    #[test]
    fn test_advances_forward_when_facing_north() {
        let map = any_map();
        let mut r = Rover::new((0, 5), Direction::North, &map);

        let result = r.advance(1);

        assert!(result.is_ok());
        assert_eq!(r.position, (0, 4));
    }

    #[test]
    fn test_advances_forward_when_facing_east() {
        let map = any_map();
        let mut r = Rover::new((0, 0), Direction::East, &map);

        let result = r.advance(1);

        assert!(result.is_ok());
        assert_eq!(r.position, (1, 0));
    }

    #[test]
    fn test_advances_forward_when_facing_south() {
        let map = any_map();
        let mut r = Rover::new((0, 0), Direction::South, &map);

        let result = r.advance(1);

        assert!(result.is_ok());
        assert_eq!(r.position, (0, 1));
    }

    #[test]
    fn test_advances_forward_when_facing_west() {
        let map = any_map();
        let mut r = Rover::new((5, 0), Direction::West, &map);

        let result = r.advance(1);

        assert!(result.is_ok());
        assert_eq!(r.position, (4, 0));
    }

    #[test]
    fn test_advances_backwards_when_facing_north() {
        let map = any_map();
        let mut r = Rover::new((0, 0), Direction::North, &map);

        let result = r.advance(-1);

        assert!(result.is_ok());
        assert_eq!(r.position, (0, 1));
    }

    #[test]
    fn test_advances_backwards_when_facing_east() {
        let map = any_map();
        let mut r = Rover::new((5, 0), Direction::East, &map);

        let result = r.advance(-1);

        assert!(result.is_ok());
        assert_eq!(r.position, (4, 0));
    }

    #[test]
    fn test_advances_backwards_when_facing_south() {
        let map = any_map();
        let mut r = Rover::new((0, 5), Direction::South, &map);

        let result = r.advance(-1);

        assert!(result.is_ok());
        assert_eq!(r.position, (0, 4));
    }

    #[test]
    fn test_advances_backwards_when_facing_west() {
        let map = any_map();
        let mut r = Rover::new((0, 0), Direction::West, &map);

        let result = r.advance(-1);

        assert!(result.is_ok());
        assert_eq!(r.position, (1, 0));
    }

    #[test]
    fn test_advances_returns_error_when_an_obstacle_is_found() {
        let map = any_map_with_obstacle_at(1, 0);
        let mut r = Rover::new((0, 0), Direction::East, &map);
        let result = r.advance(1);
        assert!(result.is_err());
        assert_eq!(r.position, (0, 0));
    }

    #[test]
    fn test_turns_left_when_facing_north() {
        let map = any_map();
        let mut r = Rover::new((0, 0), Direction::North, &map);
        r.turn(-1);
        assert_eq!(r.direction, Direction::West);
    }

    #[test]
    fn test_turns_left_when_facing_east() {
        let map = any_map();
        let mut r = Rover::new((0, 0), Direction::East, &map);
        r.turn(-1);
        assert_eq!(r.direction, Direction::North);
    }

    #[test]
    fn test_turns_left_when_facing_south() {
        let map = any_map();
        let mut r = Rover::new((0, 0), Direction::South, &map);
        r.turn(-1);
        assert_eq!(r.direction, Direction::East);
    }

    #[test]
    fn test_turns_left_when_facing_west() {
        let map = any_map();
        let mut r = Rover::new((0, 0), Direction::West, &map);
        r.turn(-1);
        assert_eq!(r.direction, Direction::South);
    }

    #[test]
    fn test_turns_right_when_facing_north() {
        let map = any_map();
        let mut r = Rover::new((0, 0), Direction::North, &map);
        r.turn(1);
        assert_eq!(r.direction, Direction::East);
    }

    #[test]
    fn test_turns_right_when_facing_east() {
        let map = any_map();
        let mut r = Rover::new((0, 0), Direction::East, &map);
        r.turn(1);
        assert_eq!(r.direction, Direction::South);
    }

    #[test]
    fn test_turns_right_when_facing_south() {
        let map = any_map();
        let mut r = Rover::new((0, 0), Direction::South, &map);
        r.turn(1);
        assert_eq!(r.direction, Direction::West);
    }

    #[test]
    fn test_turns_right_when_facing_west() {
        let map = any_map();
        let mut r = Rover::new((0, 0), Direction::West, &map);
        r.turn(1);
        assert_eq!(r.direction, Direction::North);
    }

    #[test]
    fn test_wraps_around_top_edge() {
        let map = any_map_with_size(10, 10);
        let mut r = Rover::new((0, 0), Direction::North, &map);

        let result = r.advance(1);

        assert!(result.is_ok());
        assert_eq!(r.position, (0, 9));
    }

    #[test]
    fn test_move_wraps_around_right_edge() {
        let map = any_map_with_size(10, 10);
        let mut r = Rover::new((9, 0), Direction::East, &map);

        let result = r.advance(1);

        assert!(result.is_ok());
        assert_eq!(r.position, (0, 0));
    }

    #[test]
    fn test_move_wraps_around_bottom_edge() {
        let map = any_map_with_size(10, 10);
        let mut r = Rover::new((0, 9), Direction::South, &map);

        let result = r.advance(1);

        assert!(result.is_ok());
        assert_eq!(r.position, (0, 0));
    }

    #[test]
    fn test_wraps_around_left_edge() {
        let map = any_map_with_size(10, 10);
        let mut r = Rover::new((0, 0), Direction::West, &map);

        let result = r.advance(1);

        assert!(result.is_ok());
        assert_eq!(r.position, (9, 0));
    }
}
