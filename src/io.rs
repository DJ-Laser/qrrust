use core::slice::from_raw_parts as mkslice;

mod write;
pub use write::*;

pub const STDOUT_FILENO: u32 = 1;
pub const STDERR_FILENO: u32 = 1;

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

macro_rules! print {
  ($($arg:expr),+) => {
    $crate::io::write!($crate::io::STDOUT_FILENO, $($arg),+)
  };
}

macro_rules! println {
  ($($arg:expr),+) => {
    $crate::io::writeln!($crate::io::STDOUT_FILENO, $($arg),+);
  };
}

macro_rules! eprint {
  ($($arg:expr),+) => {
    $crate::io::write!($crate::io::STDERR_FILENO, $($arg),+)
  };
}

macro_rules! eprintln {
  ($($arg:expr),+) => {
    $crate::io::writeln!($crate::io::STDERR_FILENO, $($arg),+)
  };
}

pub(crate) use {eprint, eprintln, print, println};
