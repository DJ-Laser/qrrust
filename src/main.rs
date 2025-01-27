#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::arch::naked_asm;

use io::{STDIN_FILENO, print, println, read_into_buf};
use level::Movement;
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
pub fn main() {
  let (mut print_level, mut move_player) = level::level_4();

  let mut input_buf: [u8; 1] = [0];
  loop {
    // Clear the terminal
    print!(b"\x1bc");
    print_level();

    read_into_buf(STDIN_FILENO, &mut input_buf);

    let movement = match input_buf[0] {
      b'\x03' => break,
      b'w' => Movement::Up,
      b'a' => Movement::Left,
      b's' => Movement::Down,
      b'd' => Movement::Right,
      b'r' => {
        (print_level, move_player) = level::level_4();
        continue;
      }
      _ => continue,
    };

    // Returns true on clear
    if move_player(&movement) {
      print!(b"\x1bc");
      print_level();
      println!("Congrats! You beat the last level!");

      break;
    }
  }

  exit(0);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}
