import { Rover, Direction } from "./rover";

describe("Rover", () => {
  it("is created with default position and direction", () => {
    const r = new Rover();
    expect(r.position).toStrictEqual([0, 0]);
    expect(r.direction).toBe(Direction.North);
  });

  it("is created with a position an a direction", () => {
    const r = new Rover([1, 2], Direction.East);
    expect(r.position).toStrictEqual([1, 2]);
    expect(r.direction).toBe(Direction.East);
  });
});
