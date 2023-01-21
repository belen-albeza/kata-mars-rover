export type Point = [number, number];

export enum Direction {
  North = "north",
  East = "east",
  South = "south",
  West = "west",
}

export interface Map {
  width: number;
  height: number;
}

interface CPU {
  run(program: string, rover: Rover): void;
}

export class Rover {
  _position: Point;
  _direction: Direction;
  readonly cpu: CPU;
  readonly map: Map;

  constructor(
    cpu: CPU,
    position: Point = [0, 0],
    direction: Direction = Direction.North,
    map: Map = { width: 10, height: 10 }
  ) {
    this._position = position;
    this._direction = direction;
    this.cpu = cpu;
    this.map = map;
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

  turn(way: 1 | -1) {
    const directions = [
      Direction.North,
      Direction.East,
      Direction.South,
      Direction.West,
    ];

    const i = directions.indexOf(this.direction);
    const j = modulo(i + way, directions.length);

    this.direction = directions[j];
  }

  get position(): Readonly<Point> {
    return this._position;
  }

  private set position(value: Readonly<Point>) {
    const x = modulo(value[0], this.map.width);
    const y = modulo(value[1], this.map.height);

    this._position = [x, y];
  }

  get direction(): Direction {
    return this._direction;
  }

  private set direction(value: Direction) {
    this._direction = value;
  }
}

function modulo(n: number, m: number) {
  return ((n % m) + m) % m;
}
