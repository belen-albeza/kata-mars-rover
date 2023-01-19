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
    it("Moves the rover forward when receiving a 'f' command", () => {
      const r = anyRoverWithVM();
      r.run("f");
      expect(r.position).toStrictEqual([0, -1]);
    });

    it("Moves the rover backward when receiving a 'b' command", () => {
      const r = anyRoverWithVM();
      r.run("b");
      expect(r.position).toStrictEqual([0, 1]);
    });
  });
});
