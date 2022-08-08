import cpuCount from "crate_cpu_count";

describe("neon", function () {
    const pcCpu = 6; // local pc

    it("hello", function ()  {
        const h = cpuCount.hello();
        // console.log(h);

        expect(h).toBe("hello node");
    });

    it("console log from rust", function ()  {
        const count = cpuCount.get();
        // console.log(count);

        expect(count).toBe(pcCpu);
    });
});
