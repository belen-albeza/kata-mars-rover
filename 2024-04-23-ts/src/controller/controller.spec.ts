import { describe, it, expect, mock } from "bun:test";
import Controller from "./controller";

const anyTarget = () => ({ move: mock(), turn: mock() });
const anyCommand = () => ({ run: mock() });

describe("Controller", () => {
  it("Runs its command list", () => {
    const cmd1 = anyCommand();
    const cmd2 = anyCommand();
    const target = anyTarget();
    const controller = new Controller([cmd1, cmd2], target);

    controller.run();

    expect(cmd1.run).toHaveBeenNthCalledWith(1, target);
    expect(cmd2.run).toHaveBeenNthCalledWith(1, target);
  });
});
