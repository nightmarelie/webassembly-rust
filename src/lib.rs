extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[no_mangle]
pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[wasm_bindgen(raw_module = "../domUtils.js")]
extern {
    fn appendStringToBody(s: &str);
}

#[wasm_bindgen]
#[no_mangle]
pub fn run() {
    appendStringToBody("Hello World");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
