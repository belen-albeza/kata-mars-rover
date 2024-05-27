import { describe, it, expect } from "bun:test";
import {
  commandFromOpcode,
  ForwardCommand,
  BackwardCommand,
  LeftCommand,
  RightCommand,
} from "./command";

describe("Command builder", () => {
  it("Returns a ForwardCommand for `f` opcode", () => {
    expect(commandFromOpcode("f")).toBeInstanceOf(ForwardCommand);
  });

  it("Returns a BackwardCommand for `b` opcode", () => {
    expect(commandFromOpcode("b")).toBeInstanceOf(BackwardCommand);
  });

  it("Returns a LeftCommand for `l` opcode", () => {
    expect(commandFromOpcode("l")).toBeInstanceOf(LeftCommand);
  });

  it("Returns a RightCommand for `r` opcode", () => {
    expect(commandFromOpcode("r")).toBeInstanceOf(RightCommand);
  });
});
