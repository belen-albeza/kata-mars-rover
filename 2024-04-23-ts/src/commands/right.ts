import type { Command } from "./command";

export interface RightTarget {
  turn: (delta: number) => void;
}

export default class RightCommand implements Command {
  run(target: RightTarget) {
    target.turn(1);
  }
}
