#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::i64;

pub enum ComparisonOperator {
  LT = -1,
  EQ = 0,
  GT = 1,
}

#[napi]
pub fn add(a: f64, b: f64) -> f64 {
  a + b
}

#[napi]
pub fn subtract(a: i64, b: i64) -> i64 { a - b }

#[napi]
pub fn compare(a: i64, b: i64) -> i64 {
  if a < b {
    return ComparisonOperator::LT as i64;
  }
  if a > b {
    return ComparisonOperator::GT as i64;
  }
  ComparisonOperator::EQ as i64
}

#[napi]
pub fn decrement (value: i64) -> i64 {
  value - 1
}

#[napi]
pub fn increment(value: i64) -> i64 {
  value + 1
}

#[napi]
pub fn integer_divide(dividend: f64, divisor: f64) -> i64 {
  (dividend / divisor).trunc() as i64
}

#[napi]
pub fn modulo(dividend: f64, divisor: f64) -> f64 {
  dividend % divisor
}

#[napi]
pub fn multiply(multiplicand: i64, multiplier: i64) -> i64 {
  multiplicand * multiplier
}

#[napi]
pub fn power(base: f64, exponent: f64) -> f64 {
  // i64::pow(base, exponent)
  base.powf(exponent)
  // base ** exponent // js
}

#[napi]
pub fn to_number(input: i64) -> i64 {
  input
}

#[napi]
pub fn zero() -> i8 {
  0
}
