#![no_std]
#![feature(ptr_offset_from)]
#![feature(lang_items)]

extern "C" {
    fn cpp_rethrow_panic(s: *const u8, n: usize) -> !;
    fn cpp_vector_push(vec: &mut CppVector, character: u32);
}

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
unsafe fn panic(info: &core::panic::PanicInfo) -> ! {
    let s = match info.payload().downcast_ref::<&str>() {
        Some(s) => s,
        None => "Sorry, no panic info for you :(",
    };

    cpp_rethrow_panic(s.as_ptr(), s.len());
}

/* Actual code */
#[repr(C)]
pub struct CppVector {
    start: *mut u32,
    finish: *mut u32,
    buffer_end: *const u32,
}

use core::{str, slice};

#[no_mangle]
pub unsafe extern "C" fn utf8_to_utf32(ptr: *const u8, len: usize, vec: &mut CppVector) -> i32 {
    let input_slice: &[u8] = slice::from_raw_parts(ptr, len);
    let s = match str::from_utf8(input_slice) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    for character in s.chars() {
        cpp_vector_push(vec, character as u32);
    }

    return 0;
}

