import { decrement } from "@crates/calculator-napi";

describe("decrement", () => {
  it("decrements positive numbers", () => {
    expect(decrement(2)).toBe(1);
  });
  it("decrements negative numbers", () => {
    expect(decrement(-2)).toBe(-3);
  });
});
