import {
    createDinero,
    assert,
    INVALID_AMOUNT_MESSAGE,
    INVALID_SCALE_MESSAGE,
} from '@dinero.js/core';

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
} from '@crates/calculator-napi';

export const calculator = {
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
    calculator,
    onCreate({ amount, scale }) {
        assert(Number.isInteger(amount), INVALID_AMOUNT_MESSAGE);
        assert(Number.isInteger(scale), INVALID_SCALE_MESSAGE);
    },
});
