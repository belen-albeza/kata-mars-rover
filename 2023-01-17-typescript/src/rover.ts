export type Point = [number, number];

export enum Direction {
  North = "north",
  East = "east",
  South = "south",
  West = "west",
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
}
