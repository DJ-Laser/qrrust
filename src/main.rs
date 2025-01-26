#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::arch::naked_asm;
use core::slice::from_raw_parts as mkslice;

use graphics::LevelView;
use io::{
  STDIN_FILENO, eprintln, print, println, read_into_buf,
  terminal::{enable_raw_mode, get_termios, set_termios},
  to_cstr_slice,
};
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

  let prev_termios = get_termios();
  enable_raw_mode(prev_termios.clone());

  let mut input_buf: [u8; 1] = [0];
  loop {
    // Clear the terminal
    print!(b"\x1bc");
    view.print();

    read_into_buf(STDIN_FILENO, &mut input_buf);

    match input_buf[0] {
      b'\x03' => break,
      _ => (),
    }
  }

  set_termios(&prev_termios);

  exit(0);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}
