#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::arch::naked_asm;
use core::slice::from_raw_parts as mkslice;

use graphics::LevelView;
use io::{eprintln, println, to_cstr_slice};
use syscalls::exit;

mod graphics;
mod io;
mod level;
mod syscalls;

#[unsafe(no_mangle)]
#[naked]
pub unsafe extern "C" fn _start() {
  unsafe { naked_asm!("mov rdi, rsp", "call main") }
}

#[unsafe(no_mangle)]
pub fn main(stack_top: *const u8) {
  let mut args = unsafe {
    let argc = *(stack_top as *const u64);
    let argv = stack_top.add(8) as *const *const u8;

    let args = mkslice(argv, argc as usize);

    args.into_iter().map(|arg| to_cstr_slice(*arg))
  };

  let _argv0 = args.next();

  match args.next() {
    Some(flag @ b"-l") | Some(flag @ b"--level") => {
      let Some(level) = args.next() else {
        eprintln!("Expected a level number after ", flag);
        exit(2);
      };

      println!("Selected level: ", level);
    }
    Some(_) => {
      eprintln!("Invalid option, only -l or --level is permitted");
      exit(2);
    }
    None => (),
  }

  let view = LevelView::from(&level::LEVEL_0);
  view.print();

  exit(0);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}
