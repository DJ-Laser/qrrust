use crate::syscalls::write as write_unsafe;

pub enum Writable<'a> {
  Bytes(&'a [u8]),
  Number(usize),
}

impl<'a> From<usize> for Writable<'a> {
  fn from(v: usize) -> Self {
    Writable::Number(v)
  }
}

impl<'a> From<&'a str> for Writable<'a> {
  fn from(s: &'a str) -> Self {
    Writable::Bytes(s.as_bytes())
  }
}

impl<'a> From<&'a [u8]> for Writable<'a> {
  fn from(v: &'a [u8]) -> Self {
    Writable::Bytes(v)
  }
}

impl<'a, const N: usize> From<&'a [u8; N]> for Writable<'a> {
  fn from(v: &'a [u8; N]) -> Self {
    Writable::Bytes(v.as_ref())
  }
}

fn write_num(fd: u32, n: usize) {
  if n > 9 {
    write_num(fd, n / 10);
  }
  let c = b'0' + (n % 10) as u8;
  write_bytes(fd, &[c]);
}

fn write_bytes(fd: u32, b: &[u8]) {
  unsafe {
    write_unsafe(fd, b.as_ptr(), b.len());
  }
}

#[doc(hidden)]
pub fn write_args(fd: u32, args: &[Writable]) {
  for arg in args {
    match arg {
      Writable::Bytes(b) => write_bytes(fd, b),
      Writable::Number(n) => write_num(fd, *n),
    }
  }
}

macro_rules! write {
    ($fd:expr, $($arg:expr),+) => {
      $crate::io::write_args($fd, &[
            $($arg.into()),+
        ])
    };
}

macro_rules! writeln {
    ($fd:expr, $($arg:expr),+) => {
      $crate::io::write!($fd, $($arg),+,b"\n")
    };
}

pub(crate) use {write, writeln};
