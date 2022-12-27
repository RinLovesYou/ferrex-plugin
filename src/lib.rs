#[no_mangle]
extern "C" fn init() {
    let string = "Hello from wasm!";
    unsafe {
        log_str(string.as_ptr() as usize as i32, string.len() as i32);
    }
}

#[link(wasm_import_module = "")]
extern "C" {
    fn log_str(ptr: i32, len: i32);
}