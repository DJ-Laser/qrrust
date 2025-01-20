#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::arch::naked_asm;
use core::slice::from_raw_parts as mkslice;

use io::to_cstr_slice;
use syscalls::exit;

mod io;
mod syscalls;

#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start() {
  naked_asm!("mov rdi, rsp", "call main")
}

#[no_mangle]
pub fn main(stack_top: *const u8) {
  let args = unsafe {
    let argc = *(stack_top as *const u64);
    let argv = stack_top.add(8) as *const *const u8;

    let args = mkslice(argv, argc as usize);

    args.into_iter().map(|arg| to_cstr_slice(*arg))
  };

  for arg in args.clone() {
    writeln!(1, arg);
  }

  write!(1, args.len());
  exit(0);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}
