#[unsafe(no_mangle)]
pub extern "C" fn get_color() -> u32 {
    0x9400D3FF
}

static TEXT: &[u8] = b"Hello from module 3\0";

#[unsafe(no_mangle)]
pub extern "C" fn get_text() -> *const u8 {
    TEXT.as_ptr()
}

