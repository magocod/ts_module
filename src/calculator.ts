import {
  createDinero,
  assert,
  INVALID_AMOUNT_MESSAGE,
  INVALID_SCALE_MESSAGE,
} from "@dinero.js/core";

import {
  add,
  compare,
  decrement,
  increment,
  integerDivide,
  modulo,
  multiply,
  power,
  subtract,
  toNumber,
  zero,
} from "@crates/calculator-napi";

import * as dinBigInt from "@dinero.js/calculator-bigint";

const calculatorNapi = {
  add,
  compare,
  decrement,
  increment,
  integerDivide,
  modulo,
  multiply,
  power,
  subtract,
  toNumber,
  zero,
};

export const dineroNapi = createDinero({
  calculator: calculatorNapi,
  onCreate({ amount, scale }) {
    assert(Number.isInteger(amount), INVALID_AMOUNT_MESSAGE);
    assert(Number.isInteger(scale), INVALID_SCALE_MESSAGE);
  },
});

export const dineroBigint = createDinero({
  calculator: dinBigInt.calculator,
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  onCreate({ amount, scale }) {
    // assert(Number.isInteger(amount), INVALID_AMOUNT_MESSAGE);
    // assert(Number.isInteger(scale), INVALID_SCALE_MESSAGE);
  },
});
