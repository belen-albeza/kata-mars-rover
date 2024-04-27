import { describe, it, expect, mock } from "bun:test";
import BackwardCommand from "./backward";

const anyTarget = () => ({
  move: mock(),
});

describe("Backward", () => {
  it("Moves the target 1 unit backward", () => {
    let cmd = new BackwardCommand();
    let target = anyTarget();

    cmd.run(target);

    expect(target.move).toHaveBeenCalledWith(-1);
  });
});
