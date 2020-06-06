#![no_std]
#![feature(lang_items)]

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  let (file, line) = match info.location() {
    Some(loc) => (loc.file(), loc.line()),
    None      => ("UNKNOWN FILE", u32::MAX),
  };

  unsafe {
    cpp_rethrow_panic(file.as_ptr(), file.len(), line);
  }
}

/* Imports */
use core::{slice, str};

extern "C" {
  fn cpp_rethrow_panic(s: *const u8, n: usize, line: u32) -> !;
  fn cpp_vector_push_u32(vec: &mut CppVecU32, character: u32);
}

#[repr(C)]
pub struct CppVecU32 {
  start: *mut u32,
  finish: *mut u32,
  buffer_end: *const u32,
}

/* Exports */
#[no_mangle]
pub extern "C" fn utf8_to_utf32(ptr: *const u8, len: usize, vec: &mut CppVecU32) -> bool {
  let input_slice: &[u8] = unsafe { slice::from_raw_parts(ptr, len) };
  let s = match str::from_utf8(input_slice) {
    Ok(x) => x,
    Err(_) => return false,
  };

  for character in s.chars() {
    unsafe { cpp_vector_push_u32(vec, character as u32) };
  }

  return true;
}

