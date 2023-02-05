pub mod rover;
use rover::{Direction, Rover};

pub fn run(x: i32, y: i32, position: Direction) {
    let r = Rover::new((x, y), position);
    println!("{}", r);
}
