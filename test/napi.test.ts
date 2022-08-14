import {
  sum,
  minus,
  AnimalWithDefaultConstructor,
  QueryEngine,
} from "crate_awesome";

describe("napi", function () {
  it("sum", function () {
    expect(sum(1, 2)).toBe(3);
  });

  it("minus", async function () {
    const r = await minus(10, 5);

    expect(r).toBe(5);
  });

  it("AnimalWithDefaultConstructor", function () {
    const i = new AnimalWithDefaultConstructor("a", 10);
    // console.log(i.name)

    expect(i).toBeInstanceOf(AnimalWithDefaultConstructor);
  });

  it("QueryEngine", async function () {
    const i = new QueryEngine();
    const q = "select";
    const r = await i.query(q);

    expect(i.status()).toEqual(1);
    expect(r).toEqual("query -> " + q);
  });
});
