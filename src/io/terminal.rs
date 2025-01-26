use rustix::{fd::BorrowedFd, termios::Termios};

use super::STDOUT_FILENO;

fn get_stdin_fd() -> BorrowedFd<'static> {
  unsafe { BorrowedFd::borrow_raw(STDOUT_FILENO as i32) }
}

pub fn get_termios() -> Termios {
  rustix::termios::tcgetattr(get_stdin_fd()).unwrap()
}

pub fn set_termios(termios: &Termios) {
  rustix::termios::tcsetattr(
    get_stdin_fd(),
    rustix::termios::OptionalActions::Now,
    termios,
  )
  .unwrap();
}

pub fn enable_raw_mode(mut termios: Termios) {
  termios.make_raw();
  set_termios(&termios);
}
