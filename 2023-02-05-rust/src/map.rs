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

    pub fn cell(&self, x: i32, y: i32) -> Cell {
        let i = self.index_for(x, y);
        self.cells.get(i).unwrap().to_owned()
    }

    fn index_for(&self, x: i32, y: i32) -> usize {
        let x = x.rem_euclid(self.width as i32);
        let y = y.rem_euclid(self.height as i32);
        return self.width * y as usize + x as usize;
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

    fn has_obstacle_at(&self, x: i32, y: i32) -> bool {
        self.cell(x, y) == Cell::Obstacle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn any_map_with_obstacle_at(x: i32, y: i32) -> Map {
        let mut map = Map::default();
        let i = map.width * y as usize + x as usize;
        map.cells[i] = Cell::Obstacle;
        map
    }

    #[test]
    fn test_creates_cells_from_chars() {
        assert_eq!(Cell::from(' '), Cell::Empty);
        assert_eq!(Cell::from('*'), Cell::Obstacle);
    }

    #[test]
    fn test_creates_a_default_empty_map_with_size_10() {
        let map = Map::default();
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);
        assert_eq!(map.cells.contains(&Cell::Obstacle), false);
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

    #[test]
    fn test_returns_cell_at_position() {
        let map = any_map_with_obstacle_at(1, 1);
        assert_eq!(map.cell(0, 0), Cell::Empty);
        assert_eq!(map.cell(1, 1), Cell::Obstacle);
    }

    #[test]
    fn test_returns_cell_at_position_wraps_around_top_edge() {
        let map = any_map_with_obstacle_at(0, 9);
        assert_eq!(map.cell(0, -1), Cell::Obstacle);
    }

    #[test]
    fn test_returns_cell_at_position_wraps_around_bottom_edge() {
        let map = any_map_with_obstacle_at(0, 0);
        assert_eq!(map.cell(0, 10), Cell::Obstacle);
    }

    #[test]
    fn test_returns_cell_at_position_wraps_around_left_edge() {
        let map = any_map_with_obstacle_at(9, 0);
        assert_eq!(map.cell(-1, 0), Cell::Obstacle);
    }

    #[test]
    fn test_returns_cell_at_position_wraps_around_right_edge() {
        let map = any_map_with_obstacle_at(0, 0);
        assert_eq!(map.cell(0, 10), Cell::Obstacle);
    }

    #[test]
    fn test_has_obstacle_returns_true_when_cell_is_obstacle() {
        let map = any_map_with_obstacle_at(0, 0);
        assert_eq!(map.has_obstacle_at(0, 0), true);
    }
}
