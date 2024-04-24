export type Position = {
  x: number;
  y: number;
};

const directions = ["north", "east", "south", "west"] as const;

export type Direction = (typeof directions)[number];

export function isDirection(value: string): value is Direction {
  return directions.includes(value as Direction);
}

export default class Rover {
  #position: Position;
  #direction: Direction;

  constructor(position: Position, direction: Direction) {
    this.#position = position;
    this.#direction = direction;
  }

  toString() {
    return `[at (${this.#position.x}, ${this.position.y}) facing ${
      this.direction
    }]`;
  }

  get direction() {
    return this.#direction;
  }

  get position() {
    return this.#position;
  }
}
