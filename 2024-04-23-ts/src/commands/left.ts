import type { Command } from "./command";

export interface LeftTarget {
  turn: (delta: number) => void;
}

export default class LeftCommand implements Command {
  run(target: LeftTarget) {
    target.turn(-1);
  }
}
