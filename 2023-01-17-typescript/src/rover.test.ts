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

  describe("Moving forward", () => {
    it("Moves forward when facing north", () => {
      const r = new Rover(anyCPU(), [1, 1], Direction.North);
      r.move(1);
      expect(r.position).toStrictEqual([1, 0]);
    });

    it("Moves forward when facing east", () => {
      const r = new Rover(anyCPU(), [1, 1], Direction.East);
      r.move(1);
      expect(r.position).toStrictEqual([2, 1]);
    });

    it("Moves forward when facing south", () => {
      const r = new Rover(anyCPU(), [1, 1], Direction.South);
      r.move(1);
      expect(r.position).toStrictEqual([1, 2]);
    });

    it("Moves forward when facing west", () => {
      const r = new Rover(anyCPU(), [1, 1], Direction.West);
      r.move(1);
      expect(r.position).toStrictEqual([0, 1]);
    });
  });

  describe("Moving backward", () => {
    it("Moves backward when facing north", () => {
      const r = new Rover(anyCPU(), [1, 1], Direction.North);
      r.move(-1);
      expect(r.position).toStrictEqual([1, 2]);
    });

    it("Moves backward when facing east", () => {
      const r = new Rover(anyCPU(), [1, 1], Direction.East);
      r.move(-1);
      expect(r.position).toStrictEqual([0, 1]);
    });

    it("Moves backward when facing south", () => {
      const r = new Rover(anyCPU(), [1, 1], Direction.South);
      r.move(-1);
      expect(r.position).toStrictEqual([1, 0]);
    });

    it("Moves backward when facing west", () => {
      const r = new Rover(anyCPU(), [1, 1], Direction.West);
      r.move(-1);
      expect(r.position).toStrictEqual([2, 1]);
    });
  });

  describe("Turning left", () => {
    it("Turns left when facing north", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.North);
      r.turn(-1);
      expect(r.direction).toBe(Direction.West);
    });

    it("Turns left when facing east", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.East);
      r.turn(-1);
      expect(r.direction).toBe(Direction.North);
    });

    it("Turns left when facing south", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.South);
      r.turn(-1);
      expect(r.direction).toBe(Direction.East);
    });

    it("Turns left when facing west", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.West);
      r.turn(-1);
      expect(r.direction).toBe(Direction.South);
    });
  });

  describe("Turning right", () => {
    it("Turns right when facing north", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.North);
      r.turn(1);
      expect(r.direction).toBe(Direction.East);
    });

    it("Turns right when facing east", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.East);
      r.turn(1);
      expect(r.direction).toBe(Direction.South);
    });

    it("Turns right when facing south", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.South);
      r.turn(1);
      expect(r.direction).toBe(Direction.West);
    });

    it("Turns right when facing west", () => {
      const r = new Rover(anyCPU(), [0, 0], Direction.West);
      r.turn(1);
      expect(r.direction).toBe(Direction.North);
    });
  });

  describe("Wrapping around the edges", () => {
    it("Wraps around the left edge", () => {
      const map = { width: 5, height: 5 };
      const r = new Rover(anyCPU(), [0, 1], Direction.West, map);

      r.move(1);

      expect(r.position).toStrictEqual([4, 1]);
    });

    it("Wraps around the right edge", () => {
      const map = { width: 5, height: 5 };
      const r = new Rover(anyCPU(), [4, 1], Direction.East, map);

      r.move(1);

      expect(r.position).toStrictEqual([0, 1]);
    });

    it("Wraps around the top edge", () => {
      const map = { width: 5, height: 5 };
      const r = new Rover(anyCPU(), [1, 0], Direction.North, map);

      r.move(1);

      expect(r.position).toStrictEqual([1, 4]);
    });

    it("Wraps around the bottom edge", () => {
      const map = { width: 5, height: 5 };
      const r = new Rover(anyCPU(), [1, 4], Direction.South, map);

      r.move(1);

      expect(r.position).toStrictEqual([1, 0]);
    });
  });
});
