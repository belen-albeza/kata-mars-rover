import { describe, it, expect } from "bun:test";
import Rover from "./rover";

describe("Rover", () => {
  it("Gets created with the given position and direction", () => {
    const r = new Rover({ x: 1, y: 2 }, "east");
    expect(r.position).toEqual({ x: 1, y: 2 });
    expect(r.direction).toBe("east");
  });
});
