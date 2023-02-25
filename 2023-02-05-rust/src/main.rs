use std::path::PathBuf;

use clap::Parser;
use kata_mars_rover::{run, Direction};

/// Mars Rover kata
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// X coordinate for the rover
    #[arg(short, long, default_value_t = 0)]
    x: i32,

    /// Y coordinate for the rover
    #[arg(short, long, default_value_t = 0)]
    y: i32,

    /// Direction the rover is facing
    #[arg(short, long = "dir", value_enum, default_value_t = Direction::North)]
    direction: Direction,

    /// Commands for the rover
    #[arg(short, long = "cmd", default_value_t = String::new())]
    commands: String,

    /// File path to Mars map
    #[arg(short, long)]
    map: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let output = run(args.x, args.y, args.direction, &args.commands, args.map).unwrap();
    println!("{}", output);
}
