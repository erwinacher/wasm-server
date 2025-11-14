#[unsafe(no_mangle)]
pub extern "C" fn get_color() -> u32 {
    0xFF0000FF
}

static TEXT: &[u8] = b"Hello from module 1\0";

#[unsafe(no_mangle)]
pub extern "C" fn get_text() -> *const u8 {
    TEXT.as_ptr()
}
