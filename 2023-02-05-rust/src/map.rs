use crate::rover::RoverMap;

#[derive(Debug)]
pub struct Map {
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
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
