use wasm_bindgen_test::*;

use practice::{hello_js, Config, HELLO_MESSAGE};

#[test]
fn test_add_one() {
    assert_eq!(2 + 2, 4);
}

#[wasm_bindgen_test]
fn hello_js_test() {
    let t = "string_from_js".to_string();
    let v = hello_js(t.as_str());

    assert_eq!(v, format!("{} -> {}", HELLO_MESSAGE, t));
}

#[wasm_bindgen_test]
fn get_constants_from_js() {
    let v = Config::hello_message();

    assert_eq!(v, HELLO_MESSAGE);
}
