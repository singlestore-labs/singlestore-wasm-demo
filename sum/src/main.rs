fn main() {}

#[no_mangle]
pub extern "C" fn wasm_add(a: i32, b: i32) -> i32 {
    a + b
}
