import BackwardCommand, { type BackwardTarget } from "./backward";
export { default as BackwardCommand } from "./backward";
import ForwardCommand, { type ForwardTarget } from "./forward";
export { default as ForwardCommand } from "./forward";
import LeftCommand, { type LeftTarget } from "./left";
export { default as LeftCommand } from "./left";

const opcodes = ["l", "r", "f", "b"] as const;
export type Opcode = (typeof opcodes)[number];

export function isOpcode(value: string): value is Opcode {
  return opcodes.includes(value as Opcode);
}

export interface CommandTarget
  extends ForwardTarget,
    BackwardTarget,
    LeftTarget {}

export interface Command {
  run: (target: CommandTarget) => void;
}

interface CommandConstructor {
  new (): Command;
}

const commands: Map<Opcode, CommandConstructor> = new Map([
  ["f", ForwardCommand as CommandConstructor],
  ["b", BackwardCommand as CommandConstructor],
  ["l", LeftCommand as CommandConstructor],
]);

export function commandFromOpcode(opcode: Opcode): Command {
  let cmd = commands.get(opcode);
  if (cmd !== undefined) {
    return new cmd();
  }

  throw new Error(`Unsupported opcode: ${opcode}`);
}
