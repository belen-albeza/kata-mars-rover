const opcodes = ["l", "r", "f", "b"] as const;
export type Opcode = (typeof opcodes)[number];

export function isOpcode(value: string): value is Opcode {
  return opcodes.includes(value as Opcode);
}

export interface CommandTarget {}

export interface Command {
  run: (target: CommandTarget) => {};
}

interface CommandConstructor {
  new (): Command;
}

const commands: Map<Opcode, CommandConstructor> = new Map();

export function commandFromOpcode(opcode: Opcode): Command {
  let cmd = commands.get(opcode);
  if (cmd !== undefined) {
    return new cmd();
  }

  throw new Error(`Unsupported opcode: ${opcode}`);
}
