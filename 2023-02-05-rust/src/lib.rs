pub mod commands;
pub mod cpu;
pub mod map;
pub mod rover;

pub use map::Map;
pub use rover::Direction;

use cpu::Cpu;
use rover::Rover;

pub fn run(x: i32, y: i32, position: Direction, commands: &str) -> Result<String, String> {
    let map = Map::default();
    let mut r = Rover::new((x, y), position, &map);
    let program = Cpu::parse(commands)?;
    let cpu = Cpu::new(&program);

    cpu.run(&mut r)?;

    Ok(format!("{}", r))
}
