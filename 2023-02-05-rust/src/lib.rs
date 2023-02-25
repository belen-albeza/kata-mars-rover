pub mod commands;
pub mod cpu;
pub mod map;
pub mod rover;

use std::fs;
use std::path::PathBuf;

pub use map::Map;
pub use rover::Direction;

use cpu::Cpu;
use rover::Rover;

pub fn run(
    x: i32,
    y: i32,
    position: Direction,
    commands: &str,
    map_file: Option<PathBuf>,
) -> Result<String, String> {
    let map = map_file.map_or_else(Map::default, |x| Map::from(read_file(x).as_str()));

    let mut r = Rover::new((x, y), position, &map);
    let program = Cpu::parse(commands)?;
    let cpu = Cpu::new(&program);

    cpu.run(&mut r)?;

    Ok(format!("{}", r))
}

fn read_file(file_path: PathBuf) -> String {
    let contents = fs::read_to_string(file_path).expect("Could not read file");
    if contents.len() == 0 {
        panic!("Map file is empty");
    }

    return contents;
}
