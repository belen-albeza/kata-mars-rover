export type Point = [number, number];

export enum Direction {
  North = "north",
  East = "east",
  South = "south",
  West = "west",
}

export enum Command {
  NoOp = " ",
}

export class Rover {
  readonly position: Point;
  readonly direction: Direction;

  constructor(
    position: Point = [0, 0],
    direction: Direction = Direction.North
  ) {
    this.position = position;
    this.direction = direction;
  }

  run(program: string) {
    const _ = parseProgram(program);
  }
}

function parseProgram(program: string): Command[] {
  return [...program].map((x) => {
    switch (x) {
      case " ":
        return Command.NoOp;
      default:
        throw new Error(`Syntax error. Unrecognized command: '${x}'`);
    }
  });
}
