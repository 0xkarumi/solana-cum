#[inline]
pub fn sol_log(ptr: *const u8, size: usize) {
    unsafe {
        core::mem::transmute::<_, extern "C" fn(*const u8, usize) -> u64>(0x207559bdusize)(
            ptr, size,
        );
    }
}

#[inline]
pub fn sol_log_slice(slice: &[u8]) {
    sol_log(slice.as_ptr(), slice.len())
}

#[inline]
pub fn sol_log_str(line: &str) {
    sol_log(line.as_ptr(), line.len())
}
