import { zero } from '@crates/calculator-napi';

describe('zero', () => {
  it('returns zero', () => {
    expect(zero()).toBe(0);
  });
});
