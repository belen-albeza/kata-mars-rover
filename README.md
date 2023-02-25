# kata-mars-rover

My take(s) at the Mars Rover kata.

## Requirements

From [Kata-log](https://kata-log.rocks/mars-rover-kata).

Your Task:

You’re part of the team that explores Mars by sending remotely controlled vehicles to the surface of the planet. Develop an API that translates the commands sent from earth to instructions that are understood by the rover.

Requirements:

- You are given the initial starting point (x,y) of a rover and the direction (N,S,E,W) it is facing.
- The rover receives a character array of commands.
- Implement commands that move the rover forward/backward (f,b).
- Implement commands that turn the rover left/right (l,r).
- Implement wrapping at edges. But be careful, planets are spheres.
- Implement obstacle detection before each move to a new square. If a given sequence of commands encounters an obstacle, the rover moves up to the last possible point, aborts the sequence and reports the obstacle.

## Log

- [2023-01-17](2023-01-17-typescript/) (TypeScript)
  - Classic VM style
- [2023-02-05](2023-02-05-rust/) (Rust)
  - Command pattern
  - CLI bin (run `cargo run -- --help` to see usage)
 
