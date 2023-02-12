use crate::rover::RoverMap;

use std::convert::From;

#[derive(Debug, PartialEq, Clone)]
pub struct Map {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Cell {
    Empty,
    Obstacle,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            ' ' => Self::Empty,
            _ => Self::Obstacle,
        }
    }
}

impl Map {
    pub fn new(width: usize, height: usize, cells: &[Cell]) -> Self {
        Self {
            width,
            height,
            cells: cells.to_owned(),
        }
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let height = value.lines().count();
        let cells: Vec<Cell> = value
            .lines()
            .flat_map(|x| x.chars())
            .map(Cell::from)
            .collect();
        let width = cells.len() / height;

        Map::new(width, height, &cells)
    }
}

impl Default for Map {
    fn default() -> Self {
        const DEFAULT_WIDTH: usize = 10;
        const DEFAULT_HEIGHT: usize = 10;
        let cells: [Cell; DEFAULT_WIDTH * DEFAULT_HEIGHT] =
            [Cell::Empty; DEFAULT_WIDTH * DEFAULT_HEIGHT];

        Map::new(DEFAULT_WIDTH, DEFAULT_HEIGHT, &cells)
    }
}

impl RoverMap for Map {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creates_cells_from_chars() {
        assert_eq!(Cell::from(' '), Cell::Empty);
        assert_eq!(Cell::from('*'), Cell::Obstacle);
    }

    #[test]
    fn test_creates_map_from_string() {
        let raw = "* **\n*  *\n* **";
        let map = Map::from(raw);

        assert_eq!(
            map.cells,
            vec![
                Cell::Obstacle,
                Cell::Empty,
                Cell::Obstacle,
                Cell::Obstacle,
                Cell::Obstacle,
                Cell::Empty,
                Cell::Empty,
                Cell::Obstacle,
                Cell::Obstacle,
                Cell::Empty,
                Cell::Obstacle,
                Cell::Obstacle,
            ]
        );
        assert_eq!(map.width, 4);
        assert_eq!(map.height, 3);
    }

    #[test]
    fn test_creates_map_from_string_ignores_trailing_endline() {
        let no_trail = "* **\n*  *\n* **";
        let with_trail = "* **\n*  *\n* **\n";

        let map_no_trail = Map::from(no_trail);
        let map_with_trail = Map::from(with_trail);

        assert_eq!(map_no_trail, map_with_trail);
    }
}
