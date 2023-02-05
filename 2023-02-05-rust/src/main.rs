use clap::Parser;
use kata_mars_rover::{rover, run};

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
    #[arg(short, long = "dir", value_enum, default_value_t = rover::Direction::North)]
    direction: rover::Direction,
}

fn main() {
    let args = Args::parse();
    run(args.x, args.y, args.direction);
}
