import type { Command, CommandTarget } from "../commands";
export type { Command, CommandTarget } from "../commands";

export default class Controller {
  readonly commands: Command[];
  readonly target: CommandTarget;

  constructor(commands: Command[], target: CommandTarget) {
    this.commands = commands;
    this.target = target;
  }

  run() {
    for (const cmd of this.commands) {
      cmd.run(this.target);
    }
  }
}
