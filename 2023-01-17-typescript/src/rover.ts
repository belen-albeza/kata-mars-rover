export type Point = [number, number];

export enum Direction {
  North = "north",
  East = "east",
  South = "south",
  West = "west",
}

export enum Command {
  NoOp = " ",
  Forward = "forward",
}

export class Rover {
  _position: Point;
  readonly direction: Direction;

  constructor(
    position: Point = [0, 0],
    direction: Direction = Direction.North
  ) {
    this._position = position;
    this.direction = direction;
  }

  run(program: string) {
    const commands = parseProgram(program);
    for (const cmd of commands) {
      switch (cmd) {
        case Command.NoOp:
          break;
        case Command.Forward:
          this.move();
      }
    }
  }

  move() {
    const deltaForDirection = () => {
      switch (this.direction) {
        case Direction.North:
          return [0, -1];
        case Direction.East:
          return [1, 0];
        case Direction.South:
          return [0, 1];
        case Direction.West:
          return [-1, 0];
      }
    };

    const [x, y] = deltaForDirection();
    this.position = [this.position[0] + x, this.position[1] + y];
  }

  get position(): Readonly<Point> {
    return this._position;
  }

  private set position(value: Readonly<Point>) {
    this._position = [value[0], value[1]];
  }
}

function parseProgram(program: string): Command[] {
  return [...program].map((x) => {
    switch (x) {
      case "f":
        return Command.Forward;
      case " ":
        return Command.NoOp;
      default:
        throw new Error(`Syntax error. Unrecognized command: '${x}'`);
    }
  });
}
