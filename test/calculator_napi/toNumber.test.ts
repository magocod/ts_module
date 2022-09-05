import { toNumber } from '@crates/calculator-napi';

describe('toNumber', () => {
  it('returns the input', () => {
    expect(toNumber(2)).toBe(2);
  });
});
