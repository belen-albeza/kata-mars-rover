import { VM } from "./vm";

const anyVehicle = () => {
  return {
    move: jest.fn(),
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
});
