pub mod commands;
pub mod cpu;
pub mod rover;

use commands::Opcode;
use rover::{Direction, Rover};

pub fn run(x: i32, y: i32, position: Direction, commands: &str) {
    let r = Rover::new((x, y), position);
    let program = parse_commands(commands);

    r.run(&program);

    println!("{}", r);
}

fn parse_commands(commands: &str) -> Vec<Opcode> {
    commands
        .chars()
        .map(|x| Opcode::try_from(x).unwrap())
        .collect()
}
