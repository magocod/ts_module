import os from "os";

import cpuCount from "crate_cpu_count";

describe("neon", function () {
  // const pcCpu = 6; // local pc

  const bookValue = {
    author: "B",
    title: "A",
    year: 2009,
  };

  it("hello", function () {
    const h = cpuCount.hello();
    // console.log(h);

    expect(h).toBe("hello node");
  });

  it("neon cpu count", function () {
    const count = cpuCount.get();
    // console.log(count);
    const nodeCount = os.cpus().length;
    // console.log(nodeCount)

    // expect(count).toBe(pcCpu);
    expect(count).toBe(nodeCount);
  });

  it("neon object", function () {
    expect(cpuCount.book).toEqual(bookValue);
  });
});
