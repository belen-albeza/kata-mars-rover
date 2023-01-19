export enum Command {
  NoOp = " ",
  Forward = "forward",
  Backward = "backward",
}

export interface Vehicle {
  move(way: 1 | -1): void;
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
      }
    }
  }
}

function parseProgram(program: string): Command[] {
  return [...program].map((x) => {
    switch (x) {
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
