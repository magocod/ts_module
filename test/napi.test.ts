import { sum } from "crate_awesome";

describe("napi", function () {
  it("sum", function () {
    expect(sum(1, 2)).toBe(3);
  });
});
