#![deny(clippy::all)]

use napi::bindgen_prelude::*;

pub mod class_example;

#[macro_use]
extern crate napi_derive;

#[napi]
fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
async fn minus(a: i32, b: i32) -> Result<i32> {
  Ok(a - b)
}
