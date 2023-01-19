import { Rover, Direction } from "./rover";

const anyCPU = () => {
  return {
    run: jest.fn(),
  };
};

describe("Rover", () => {
  it("is created with default position and direction", () => {
    const r = new Rover(anyCPU());
    expect(r.position).toStrictEqual([0, 0]);
    expect(r.direction).toBe(Direction.North);
  });

  it("is created with a position an a direction", () => {
    const r = new Rover(anyCPU(), [1, 2], Direction.East);
    expect(r.position).toStrictEqual([1, 2]);
    expect(r.direction).toBe(Direction.East);
  });

  describe("Moves forward", () => {
    it("Moves the rover forward when facing north", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.North);
      r.move(1);
      expect(r.position).toStrictEqual([0, -1]);
    });

    it("Moves the rover forward when facing east", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.East);
      r.move(1);
      expect(r.position).toStrictEqual([1, 0]);
    });

    it("Moves the rover forward when facing south", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.South);
      r.move(1);
      expect(r.position).toStrictEqual([0, 1]);
    });

    it("Moves the rover forward when facing west", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.West);
      r.move(1);
      expect(r.position).toStrictEqual([-1, 0]);
    });
  });

  describe("Moves backward", () => {
    it("Moves the rover backward when facing north", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.North);
      r.move(-1);
      expect(r.position).toStrictEqual([0, 1]);
    });

    it("Moves the rover backward when facing east", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.East);
      r.move(-1);
      expect(r.position).toStrictEqual([-1, 0]);
    });

    it("Moves the rover backward when facing south", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.South);
      r.move(-1);
      expect(r.position).toStrictEqual([0, -1]);
    });

    it("Moves the rover backward when facing west", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.West);
      r.move(-1);
      expect(r.position).toStrictEqual([1, 0]);
    });
  });
});
