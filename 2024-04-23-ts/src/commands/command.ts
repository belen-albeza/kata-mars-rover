import type { BackwardTarget } from "./backward";
import BackwardCommand from "./backward";
import ForwardCommand, { type ForwardTarget } from "./forward";

const opcodes = ["l", "r", "f", "b"] as const;
export type Opcode = (typeof opcodes)[number];

export function isOpcode(value: string): value is Opcode {
  return opcodes.includes(value as Opcode);
}

export interface CommandTarget extends ForwardTarget, BackwardTarget {}

export interface Command {
  run: (target: CommandTarget) => void;
}

interface CommandConstructor {
  new (): Command;
}

const commands: Map<Opcode, CommandConstructor> = new Map([
  ["f", ForwardCommand],
  ["b", BackwardCommand],
]);

export function commandFromOpcode(opcode: Opcode): Command {
  let cmd = commands.get(opcode);
  if (cmd !== undefined) {
    return new cmd();
  }

  throw new Error(`Unsupported opcode: ${opcode}`);
}
