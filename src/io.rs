pub const STDOUT_FILENO: u32 = 1;

mod write;

pub use write::*;

/**
 * Gets the length of a c-string as bytes
 *
 */
pub unsafe fn strlen(mut s: *const u8) -> usize {
  let mut count = 0;
  while *s != b'\0' {
    count += 1;
    s = s.add(1);
  }
  count
}
