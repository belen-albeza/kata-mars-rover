export type Position = {
  x: number;
  y: number;
};

export type Direction = "north" | "east" | "south" | "west";

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
