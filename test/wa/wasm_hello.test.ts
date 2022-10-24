import { hello_js, Config } from "practice_wasm";

describe("wasm", function () {
  const name = "js";

  it("console log from rust", function () {
    const v = hello_js(name);
    // console.log(v);

    expect(v).toBe(`${Config.hello_message()} -> ${name}`);
  });
});
