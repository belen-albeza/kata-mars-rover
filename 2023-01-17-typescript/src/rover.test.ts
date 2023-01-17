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

  describe("Program running", () => {
    it("Runs an empty program without throwing errors", () => {
      const r = new Rover();
      expect(() => {
        r.run("");
      }).not.toThrow();
      expect(r.position).toStrictEqual([0, 0]);
      expect(r.direction).toBe(Direction.North);
    });

    it("Runs a program with NoOp", () => {
      const r = new Rover();
      r.run(" ");
      expect(r.position).toStrictEqual([0, 0]);
      expect(r.direction).toBe(Direction.North);
    });

    it("Throws an error when given a program with unknown commands", () => {
      const r = new Rover();
      expect(() => {
        r.run("?");
      }).toThrowError(/syntax error/i);
    });

    describe("Moves forward", () => {
      it("Moves the rover forward when facing north", () => {
        const r = new Rover([0, 0], Direction.North);
        r.run("f");
        expect(r.position).toStrictEqual([0, -1]);
      });

      it("Moves the rover forward when facing east", () => {
        const r = new Rover([0, 0], Direction.East);
        r.run("f");
        expect(r.position).toStrictEqual([1, 0]);
      });

      it("Moves the rover forward when facing south", () => {
        const r = new Rover([0, 0], Direction.South);
        r.run("f");
        expect(r.position).toStrictEqual([0, 1]);
      });

      it("Moves the rover forward when facing west", () => {
        const r = new Rover([0, 0], Direction.West);
        r.run("f");
        expect(r.position).toStrictEqual([-1, 0]);
      });
    });

    describe("Moves backward", () => {
      it("Moves the rover backward when facing north", () => {
        const r = new Rover([0, 0], Direction.North);
        r.run("b");
        expect(r.position).toStrictEqual([0, 1]);
      });

      it("Moves the rover backward when facing east", () => {
        const r = new Rover([0, 0], Direction.East);
        r.run("b");
        expect(r.position).toStrictEqual([-1, 0]);
      });

      it("Moves the rover backward when facing south", () => {
        const r = new Rover([0, 0], Direction.South);
        r.run("b");
        expect(r.position).toStrictEqual([0, -1]);
      });

      it("Moves the rover backward when facing west", () => {
        const r = new Rover([0, 0], Direction.West);
        r.run("b");
        expect(r.position).toStrictEqual([1, 0]);
      });
    });
  });
});
