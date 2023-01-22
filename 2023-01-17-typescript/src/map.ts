import { modulo } from "./utils";

export enum Cell {
  Empty = " ",
  Obstacle = "🪨",
}

export class Map {
  private _grid: Cell[];
  readonly width: number;
  readonly height: number;

  static fromString(raw: string): Map {
    const rows = raw
      .split("\n")
      .map((row) => Array.from(row).map((x) => x as Cell));

    const width = rows[0].length;

    return new Map(rows.flat(), width);
  }

  constructor(grid: readonly Cell[], width: number) {
    this._grid = [...grid];
    this.width = width;
    this.height = grid.length / width;
  }

  hasObstacleAt(x: number, y: number): boolean {
    return this.cellAt(x, y) === Cell.Obstacle;
  }

  private cellAt(x: number, y: number): Cell {
    const _x = modulo(x, this.width);
    const _y = modulo(y, this.height);

    const i = _y * this.width + _x;

    return this._grid[i];
  }
}
