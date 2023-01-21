import { Rover, Point, Direction } from "../rover";
import { VM } from "../vm";

const anyRoverWithVM = (
  position: Point = [0, 0],
  direction: Direction = Direction.North
) => {
  return new Rover(new VM(), position, direction);
};

describe("Mars Rover integration", () => {
  it("Can create a rover with a VM", () => {
    const vm = new VM();
    const rover = new Rover(vm);

    expect(rover.cpu).toBe(vm);
  });

  describe("Program running", () => {
    it("Runs a program that moves and turns the rover", () => {
      const r = anyRoverWithVM([0, 0], Direction.East);
      r.run("fflb rr");
      expect(r.position).toStrictEqual([2, 1]);
      expect(r.direction).toBe(Direction.South);
    });
  });
});
