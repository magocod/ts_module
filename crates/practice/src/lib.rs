use wasm_bindgen::prelude::*;

use web_sys::console;

pub const HELLO_MESSAGE: &str = "Hello using web-sys";

#[wasm_bindgen]
pub struct Config;

#[wasm_bindgen]
impl Config {
    pub fn hello_message() -> String {
        String::from(HELLO_MESSAGE)
    }
}

#[wasm_bindgen]
pub fn hello_js(text: &str) -> String {
    let t = hello(text);
    console::log_1(&hello(t.as_str()).into());
    t
}

pub fn hello(text: &str) -> String {
    format!("{} -> {}", HELLO_MESSAGE, text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_test() {
        let text = "name".to_string();
        let v = hello(text.as_str());
        assert_eq!(v, format!("{} -> {}", HELLO_MESSAGE, text));
    }
}
