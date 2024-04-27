import { describe, it, expect } from "bun:test";
import Rover from "./rover";

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
});
