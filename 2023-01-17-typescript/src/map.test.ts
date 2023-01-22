import { Map, Cell } from "./map";

describe("Map", () => {
  it("Gets constructed from a grid and width", () => {
    const cells = Array.from("   🪨    🪨   ").map((x) => x as Cell);
    const m = new Map(cells, 4);
    expect(m.width).toBe(4);
    expect(m.height).toBe(3);
  });

  it("Gets built from a string", () => {
    const m = Map.fromString("   🪨\n    \n🪨   ");
    expect(m.width).toBe(4);
    expect(m.height).toBe(3);
  });

  describe("Reporting obstacles", () => {
    it("Returns whether there is an obstacle at the given position", () => {
      const m = Map.fromString("   🪨\n    \n🪨   ");
      expect(m.hasObstacleAt(3, 0)).toBe(true);
      expect(m.hasObstacleAt(0, 0)).toBe(false);
    });

    it("Returns whether there is an bostacle wrapping out of bound positions", () => {
      const m = Map.fromString("🪨   \n    \n🪨   ");
      expect(m.hasObstacleAt(-1, -1)).toBe(false); // -> 3, 2
      expect(m.hasObstacleAt(4, 3)).toBe(true); // -> 0, 0
    });
  });
});
