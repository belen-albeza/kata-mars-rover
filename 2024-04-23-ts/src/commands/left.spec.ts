import { describe, expect, it, mock } from "bun:test";
import LeftCommand from "./left";

const anyTarget = () => ({
  turn: mock(),
});

describe("Left", () => {
  it("Turns the target anticlockwise", () => {
    const cmd = new LeftCommand();
    const target = anyTarget();

    cmd.run(target);

    expect(target.turn).toHaveBeenCalledWith(-1);
  });
});
