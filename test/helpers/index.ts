/**
 *
 * @param count
 * @param done
 * @param callback
 */
export function createPartialDone(
  count: number,
  done: CallableFunction,
  callback?: CallableFunction
) {
  let i = 0;
  return () => {
    if (++i === count) {
      if (callback !== undefined) {
        callback();
      }
      done();
    }
  };
}

/**
 *
 * @param count
 * @param done
 * @param callback
 */
export function* iteratePartialDone(
  count: number,
  done: CallableFunction,
  callback?: CallableFunction
) {
  let i = 1;
  while (i < count) {
    yield i;
    i++;
  }
  if (callback !== undefined) {
    callback();
  }
  done();
}
