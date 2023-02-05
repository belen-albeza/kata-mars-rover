pub mod commands;
pub mod rover;

use commands::Command;
use rover::{Direction, Rover};

pub fn run(x: i32, y: i32, position: Direction, commands: &str) {
    let r = Rover::new((x, y), position);
    let program = parse_commands(commands);

    r.run(&program);

    println!("{}", r);
}

fn parse_commands(commands: &str) -> Vec<Command> {
    commands
        .chars()
        .map(|x| Command::try_from(x).unwrap())
        .collect()
}
