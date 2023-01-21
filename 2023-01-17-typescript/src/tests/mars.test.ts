import { Rover, Point, Direction } from "../rover";
import { VM } from "../vm";
import { Map } from "../map";

const anyRoverWithVM = (
  position: Point = [0, 0],
  direction: Direction = Direction.North,
  map: Map = anyEmptyMap()
) => {
  return new Rover(new VM(), map, position, direction);
};

const anyEmptyMap = () => {
  return Map.fromString("     \n     \n     \n     \n     \n");
};

describe("Mars Rover integration", () => {
  it("Can create a rover with a VM and a map", () => {
    const vm = new VM();
    const map = anyEmptyMap();
    const rover = new Rover(vm, map, [0, 0], Direction.North);

    expect(rover.cpu).toBe(vm);
  });

  describe("Program running", () => {
    it("Runs a program that moves and turns the rover", () => {
      const r = anyRoverWithVM([0, 0], Direction.East);

      r.run("fflb rr");

      expect(r.position).toStrictEqual([2, 1]);
      expect(r.direction).toBe(Direction.South);
    });

    it("Runs a program that moves the rover towards an obstacle", () => {
      const map = Map.fromString(" 🪨\n  ");
      const r = anyRoverWithVM([0, 0], Direction.North, map);

      r.run("rfrf");

      expect(r.position).toStrictEqual([0, 0]);
      expect(r.direction).toBe(Direction.East);
    });
  });
});
