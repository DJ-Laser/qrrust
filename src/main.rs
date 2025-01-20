#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::arch::naked_asm;

use io::strlen;
use syscalls::exit;

mod io;
mod syscalls;

#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start() {
  naked_asm!("mov rdi, rsp", "call main")
}

#[no_mangle]
pub unsafe fn main(stack_top: *const u8) {
  let argc = *(stack_top as *const u64);
  let argv = stack_top.add(8) as *const *const u8;

  use core::slice::from_raw_parts as mkslice;
  let args = mkslice(argv, argc as usize);

  for &arg in args {
    let arg = mkslice(arg, strlen(arg));
    writeln!(1, arg);
  }

  write!(1, argc as usize);
  exit(0);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}
