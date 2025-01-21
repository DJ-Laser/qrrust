use core::slice::from_raw_parts as mkslice;

mod write;
pub use write::*;

pub const STDOUT_FILENO: u32 = 1;
pub const STDERR_FILENO: u32 = 1;

#[macro_export]
macro_rules! print {
    ($($arg:expr),+) => {
      write!($crate::io::STDOUT_FILENO, $($arg),+)
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:expr),+) => {
      writeln!($crate::io::STDOUT_FILENO, $($arg),+);
    };
}

#[macro_export]
macro_rules! eprint {
    ($($arg:expr),+) => {
      write!($crate::io::STDERR_FILENO, $($arg),+)
    };
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:expr),+) => {
      writeln!($crate::io::STDERR_FILENO, $($arg),+)
    };
}

/// Gets the length of a c-string as bytes (unsafe for onvious reasons *ahem* null terminatiors)
pub unsafe fn strlen(mut s: *const u8) -> usize {
  unsafe {
    let mut count = 0;
    while *s != b'\0' {
      count += 1;
      s = s.add(1);
    }
    count
  }
}

pub unsafe fn to_cstr_slice(s: *const u8) -> &'static [u8] {
  unsafe { mkslice(s, strlen(s)) }
}
