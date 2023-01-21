export enum Command {
  NoOp = " ",
  Forward = "forward",
  Backward = "backward",
  Left = "left",
  Right = "right",
}

export interface Vehicle {
  move(way: 1 | -1): void;
  turn(way: 1 | -1): void;
}

export class VM {
  run(program: string, vehicle: Vehicle) {
    const commands = parseProgram(program);
    for (const cmd of commands) {
      switch (cmd) {
        case Command.NoOp:
          break;
        case Command.Forward:
          vehicle.move(1);
          break;
        case Command.Backward:
          vehicle.move(-1);
          break;
        case Command.Left:
          vehicle.turn(-1);
          break;
        case Command.Right:
          vehicle.turn(1);
          break;
      }
    }
  }
}

function parseProgram(program: string): Command[] {
  return [...program].map((x) => {
    switch (x) {
      case "r":
        return Command.Right;
      case "l":
        return Command.Left;
      case "b":
        return Command.Backward;
      case "f":
        return Command.Forward;
      case " ":
        return Command.NoOp;
      default:
        throw new Error(`Syntax error. Unrecognized command: '${x}'`);
    }
  });
}
