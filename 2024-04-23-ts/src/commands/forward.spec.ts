import { describe, it, expect, mock } from "bun:test";
import ForwardCommand from "./forward";

const anyTarget = () => ({
  move: mock(),
});

describe("Forward", () => {
  it("Moves the target 1 unit forward", () => {
    let cmd = new ForwardCommand();
    let target = anyTarget();

    cmd.run(target);

    expect(target.move).toHaveBeenCalledWith(1);
  });
});
