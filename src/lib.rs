#[no_mangle]
pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

extern {
    fn appendNumberToBody(number: u32);
}

#[no_mangle]
pub fn run() {
    unsafe {
        appendNumberToBody(42);
    }
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
