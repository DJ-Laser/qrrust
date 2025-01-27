#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::arch::naked_asm;
use core::slice::from_raw_parts as mkslice;

use io::{
  STDIN_FILENO, eprintln, print, println, read_into_buf,
  terminal::{enable_raw_mode, get_termios, set_termios},
  to_cstr_slice,
};
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

const LEVELS: &'static [fn() -> (fn(), for<'a> fn(&'a Movement) -> bool)] = const {
  use level::*;
  &[level_1, level_2, level_3, level_4]
};

#[unsafe(no_mangle)]
pub fn main(stack_top: *const u8) {
  let mut args = unsafe {
    let argc = *(stack_top as *const u64);
    let argv = stack_top.add(8) as *const *const u8;

    let args = mkslice(argv, argc as usize);

    args.into_iter().map(|arg| to_cstr_slice(*arg))
  };

  let _argv0 = args.next();
  let mut level: usize = 0;

  match args.next() {
    Some(flag @ b"-l") | Some(flag @ b"--level") => {
      let Some(selected_level) = args.next() else {
        eprintln!("Expected a level number after ", flag);
        exit(2);
      };

      level = match selected_level {
        b"1" => 0,
        b"2" => 1,
        b"3" => 2,
        b"4" => 3,
        _ => {
          eprintln!("No such level, valid levels are 1 to ");
          exit(2);
        }
      }
    }
    Some(_) => {
      eprintln!("Invalid option, only -l or --level is permitted");
      exit(2);
    }
    None => (),
  }

  let prev_termios = get_termios();
  enable_raw_mode(prev_termios.clone());

  let (mut print_level, mut move_player) = LEVELS[level]();

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
        (print_level, move_player) = LEVELS[level]();
        continue;
      }
      _ => continue,
    };

    // Returns true on clear
    if move_player(&movement) {
      if level == LEVELS.len() - 1 {
        print!(b"\x1bc");
        print_level();
        println!("Congrats! You beat the last level!");

        break;
      }

      level += 1;

      (print_level, move_player) = LEVELS[level]();
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
