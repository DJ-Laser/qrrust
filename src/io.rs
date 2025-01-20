use core::slice::from_raw_parts as mkslice;

pub const STDOUT_FILENO: u32 = 1;

mod write;

pub use write::*;

/// Gets the length of a c-string as bytes (unsafe for onvious reasons *ahem* null terminatiors)
pub unsafe fn strlen(mut s: *const u8) -> usize {
  let mut count = 0;
  while *s != b'\0' {
    count += 1;
    s = s.add(1);
  }
  count
}

pub unsafe fn to_cstr_slice(s: *const u8) -> &'static [u8] {
  mkslice(s, strlen(s))
}
