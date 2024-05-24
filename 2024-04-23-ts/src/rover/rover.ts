import type { CommandTarget } from "../commands";

export type Position = {
  readonly x: number;
  readonly y: number;
};

const directions = ["north", "east", "south", "west"] as const;

export type Direction = (typeof directions)[number];

export function isDirection(value: string): value is Direction {
  return directions.includes(value as Direction);
}

export interface RoverMap {
  readonly width: number;
  readonly height: number;
}

export default class Rover implements CommandTarget {
  #position: Position;
  #direction: Direction;
  readonly #map: RoverMap;

  constructor(
    position: Position,
    direction: Direction,
    map: RoverMap = { width: 10, height: 10 }
  ) {
    this.#position = position;
    this.#direction = direction;
    this.#map = map;
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

  move(delta: number) {
    const [incx, incy] = [this.#xdir, this.#ydir];
    const x = modulo(this.position.x + incx * delta, this.#map.width);
    const y = modulo(this.position.y + incy * delta, this.#map.height);
    this.#position = { x, y };
  }

  turn(delta: number) {
    const directions = ["north", "east", "south", "west"] as Direction[];
    const currentIndex = directions.indexOf(this.direction);
    const i = modulo(currentIndex + delta, directions.length);

    this.#direction = directions[i];
  }

  get #xdir() {
    switch (this.#direction) {
      case "east":
        return 1;
      case "west":
        return -1;
      default:
        return 0;
    }
  }

  get #ydir() {
    switch (this.#direction) {
      case "north":
        return -1;
      case "south":
        return 1;
      default:
        return 0;
    }
  }
}

function modulo(n: number, m: number) {
  return ((n % m) + m) % m;
}
