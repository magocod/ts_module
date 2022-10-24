import { increment } from "@crates/calculator-napi";

describe("increment", () => {
  it("increments positive numbers", () => {
    expect(increment(2)).toBe(3);
  });
  it("increments negative numbers", () => {
    expect(increment(-2)).toBe(-1);
  });
});
