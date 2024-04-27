import type { Command } from "./command";

export interface ForwardTarget {
  move: (delta: number) => void;
}

export default class ForwardCommand implements Command {
  run(target: ForwardTarget) {
    target.move(1);
  }
}
