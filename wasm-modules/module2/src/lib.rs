#[unsafe(no_mangle)]
pub extern "C" fn get_color() -> u32 {
    0xFFFF00FF
}

static TEXT: &[u8] = b"Hello from module 2\0";

#[unsafe(no_mangle)]
pub extern "C" fn get_text() -> *const u8 {
    TEXT.as_ptr()
}
