export type Point = [number, number];

export enum Direction {
  North = "north",
  East = "east",
  South = "south",
  West = "west",
}

interface CPU {
  run(program: string, rover: Rover): void;
}

export class Rover {
  _position: Point;
  readonly direction: Direction;
  readonly cpu: CPU;

  constructor(
    cpu: CPU,
    position: Point = [0, 0],
    direction: Direction = Direction.North
  ) {
    this._position = position;
    this.direction = direction;
    this.cpu = cpu;
  }

  run(program: string) {
    this.cpu.run(program, this);
  }

  move(way: 1 | -1) {
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
    this.position = [this.position[0] + x * way, this.position[1] + y * way];
  }

  get position(): Readonly<Point> {
    return this._position;
  }

  private set position(value: Readonly<Point>) {
    this._position = [value[0], value[1]];
  }
}
