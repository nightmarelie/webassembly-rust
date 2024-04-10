extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[no_mangle]
pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[wasm_bindgen(module = "domUtils")]
extern {
    fn appendNumberToBody(number: u32);
    fn alert(x: u32);
    fn appendStringToBody(s: &str);
}

#[wasm_bindgen]
pub fn run() {
    appendNumberToBody(42);
    alert(42);
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
