use crate::syscalls::read as read_unsafe;

mod write;

pub(crate) use macros::*;
pub use write::*;

pub const STDIN_FILENO: u32 = 0;
pub const STDOUT_FILENO: u32 = 1;
pub const STDERR_FILENO: u32 = 2;

pub fn read_into_buf(fd: u32, buf: &mut [u8]) {
  unsafe {
    read_unsafe(fd, buf.as_mut_ptr(), buf.len());
  }
}

#[allow(unused_macros)]
mod macros {
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

  #[allow(unused_imports)]
  pub(crate) use {eprint, eprintln, print, println};
}
