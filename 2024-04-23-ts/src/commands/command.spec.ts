import { describe, it, expect } from "bun:test";
import { commandFromOpcode } from "./command";
import ForwardCommand from "./forward";
import BackwardCommand from "./backward";

describe("Command builder", () => {
  it("Returns a ForwardCommand for `f` opcode", () => {
    expect(commandFromOpcode("f")).toBeInstanceOf(ForwardCommand);
  });

  it("Returns a BackwardCommand for `b` opcode", () => {
    expect(commandFromOpcode("b")).toBeInstanceOf(BackwardCommand);
  });
});
