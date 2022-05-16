import { createPartialDone, iteratePartialDone } from "./helpers";

import EventEmitter from "events";

describe("helper_done", () => {
  describe("createPartialDone", function () {
    it("verify that 3 events occur", (done) => {
      let count = 0;
      const eventEmitter = new EventEmitter();
      const partialDone = createPartialDone(3, done, () => {
        expect(count).toBe(1 + 2 + 4);
      });

      eventEmitter.on("event", (value: number) => {
        count += value;
        partialDone();
      });

      eventEmitter.emit("event", 1);
      eventEmitter.emit("event", 2);
      eventEmitter.emit("event", 4);
    });
  });

  describe("iteratePartialDone", function () {
    it("verify that 3 events occur", (done) => {
      let count = 0;
      const eventEmitter = new EventEmitter();
      const partialDone = iteratePartialDone(3, done, () => {
        expect(count).toBe(1 + 2 + 4);
      });

      eventEmitter.on("event", (value: number) => {
        count += value;
        partialDone.next();
      });

      eventEmitter.emit("event", 1);
      eventEmitter.emit("event", 2);
      eventEmitter.emit("event", 4);
    });
  });
});
