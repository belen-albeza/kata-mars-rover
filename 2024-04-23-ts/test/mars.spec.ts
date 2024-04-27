import { describe, it, expect } from "bun:test";
import Controller from "../src/controller";
import Rover from "../src/rover";
import { commandFromOpcode, type Opcode } from "../src/commands";

function anyRoverAndController(opcodes: string): [Rover, Controller] {
  const rover = new Rover({ x: 0, y: 0 }, "north");
  const commands = opcodes.split("").map((x) => commandFromOpcode(x as Opcode));
  const controller = new Controller(commands, rover);

  return [rover, controller];
}

describe("Mars rover", () => {
  it("Follows command to move backward and forward", () => {
    const [rover, controller] = anyRoverAndController("ffbbb");
    controller.run();
    expect(rover.position).toEqual({ x: 0, y: 1 });
  });

  it("Follows command to turn left", () => {
    const [rover, controller] = anyRoverAndController("lll");
    controller.run();
    expect(rover.direction).toBe("east");
  });
});
