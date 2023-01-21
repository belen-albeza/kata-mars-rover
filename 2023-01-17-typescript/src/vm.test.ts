import { VM } from "./vm";

const anyVehicle = () => {
  return {
    move: jest.fn(),
    turn: jest.fn(),
  };
};

describe("VM", () => {
  it("Runs an empty program without throwing errors", () => {
    const vm = new VM();
    const r = anyVehicle();

    expect(() => {
      vm.run("", r);
    }).not.toThrow();

    expect(r.move).not.toHaveBeenCalled();
  });

  it("Runs a program with NoOp", () => {
    const vm = new VM();
    const r = anyVehicle();

    vm.run(" ", r);

    expect(r.move).not.toHaveBeenCalled();
  });

  it("Throws an error when given a program with unknown commands", () => {
    const vm = new VM();
    const r = anyVehicle();

    expect(() => {
      vm.run("?", r);
    }).toThrowError(/syntax error/i);
  });

  it("Moves the vehicle forward when given a 'f' command", () => {
    const vm = new VM();
    const r = anyVehicle();

    vm.run("f", r);

    expect(r.move).toHaveBeenCalledWith(1);
  });

  it("Moves the vehicle backward when given a 'b' command", () => {
    const vm = new VM();
    const r = anyVehicle();

    vm.run("b", r);

    expect(r.move).toHaveBeenCalledWith(-1);
  });

  it("Turns the vehicle left when given a 'l' command", () => {
    const vm = new VM();
    const r = anyVehicle();

    vm.run("l", r);

    expect(r.turn).toHaveBeenCalledWith(-1);
  });

  it("Turns the vehicle right when given a 'r' command", () => {
    const vm = new VM();
    const r = anyVehicle();

    vm.run("r", r);

    expect(r.turn).toHaveBeenCalledWith(1);
  });
});
