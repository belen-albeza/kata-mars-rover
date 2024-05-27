import { describe, it, expect, mock } from "bun:test";

import RightCommand from "./right";

const anyTarget = () => ({
  turn: mock(),
});

describe("Right", () => {
  it("Turns the target 1 unit clockwise", () => {
    const cmd = new RightCommand();
    const target = anyTarget();

    cmd.run(target);

    expect(target.turn).toHaveBeenCalledWith(1);
  });
});
