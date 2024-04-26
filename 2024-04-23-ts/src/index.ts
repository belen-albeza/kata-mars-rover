import { parseArgs } from "util";
import Rover from "./rover";
import type { Direction } from "./rover";
import { isDirection } from "./rover/rover";
import { isOpcode, commandFromOpcode } from "./commands";
import Controller from "./controller";

const args = parseArgs({
  args: Bun.argv.slice(2),
  options: {
    x: {
      type: "string",
      short: "x",
      default: "0",
    },
    y: {
      type: "string",
      short: "y",
      default: "0",
    },
    dir: {
      type: "string",
      short: "d",
      default: "north",
    },
  },
  allowPositionals: true,
});

const x = parseInt(args.values.x ?? "", 10);
const y = parseInt(args.values.y ?? "", 10);
if (Number.isNaN(x) || Number.isNaN(y)) {
  throw new Error("X and Y must be numbers");
}

if (!isDirection(args.values.dir ?? "")) {
  throw new Error("Dir must be: north, east, south or west");
}
const dir = args.values.dir as Direction;

const rawOpcodes = args.positionals.at(0)?.split("") ?? [];

const commands = rawOpcodes.map((x) => {
  if (isOpcode(x)) {
    return commandFromOpcode(x);
  }

  throw new Error(`Invalid opcode: ${x}`);
});

const rover = new Rover({ x, y }, dir);
console.log(`Rover ${rover}`);

const controller = new Controller(commands, rover);
console.log("Running commands…");
controller.run();

console.log(`----\nRover ${rover}`);
