import type { Command } from "./command";

export interface BackwardTarget {
  move: (delta: number) => void;
}

export default class BackwardCommand implements Command {
  run(target: BackwardTarget) {
    target.move(-1);
  }
}
