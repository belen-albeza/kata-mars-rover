import { describe, it, expect } from "bun:test";
import Rover, { type Direction } from "./rover";

describe("Rover", () => {
  it("Gets created with the given position and direction", () => {
    const r = new Rover({ x: 1, y: 2 }, "east");
    expect(r.position).toEqual({ x: 1, y: 2 });
    expect(r.direction).toBe("east");
  });
});

describe("Rover movement", () => {
  it("Moves north", () => {
    const r = new Rover({ x: 3, y: 3 }, "north");
    r.move(2);
    expect(r.position).toEqual({ x: 3, y: 1 });
  });

  it("Moves east", () => {
    const r = new Rover({ x: 3, y: 3 }, "east");
    r.move(2);
    expect(r.position).toEqual({ x: 5, y: 3 });
  });

  it("Moves south", () => {
    const r = new Rover({ x: 3, y: 3 }, "south");
    r.move(2);
    expect(r.position).toEqual({ x: 3, y: 5 });
  });

  it("Moves west", () => {
    const r = new Rover({ x: 3, y: 3 }, "west");
    r.move(2);
    expect(r.position).toEqual({ x: 1, y: 3 });
  });

  describe("Wrapping at the edges", () => {
    it("Wraps at the north edge", () => {
      const r = new Rover({ x: 0, y: 0 }, "north", { width: 5, height: 5 });
      r.move(1);
      expect(r.position.y).toBe(4);
    });

    it("Wraps at the south edge", () => {
      const r = new Rover({ x: 0, y: 4 }, "south", { width: 5, height: 5 });
      r.move(1);
      expect(r.position.y).toBe(0);
    });

    it("Wraps at the right edge", () => {
      const r = new Rover({ x: 4, y: 0 }, "east", { width: 5, height: 5 });
      r.move(1);
      expect(r.position.x).toBe(0);
    });

    it("Wraps at the left edge", () => {
      const r = new Rover({ x: 0, y: 0 }, "west", { width: 5, height: 5 });
      r.move(1);
      expect(r.position.x).toBe(4);
    });
  });
});

describe("Rover turning", () => {
  const anyRoverWithDir = (dir: Direction) => {
    return new Rover({ x: 0, y: 0 }, dir);
  };

  it("Turns anticlockwise from north", () => {
    const r = anyRoverWithDir("north");
    r.turn(-1);
    expect(r.direction).toBe("west");
  });

  it("Turns clockwise from north", () => {
    const r = anyRoverWithDir("north");
    r.turn(1);
    expect(r.direction).toBe("east");
  });

  it("Turns clockwise n times from west", () => {
    const r = anyRoverWithDir("west");
    r.turn(3);
    expect(r.direction).toBe("south");
  });

  it("Turns anticlockwise n times from east", () => {
    const r = anyRoverWithDir("east");
    r.turn(3);
    expect(r.direction).toBe("north");
  });
});
