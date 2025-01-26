use core::arch::asm;

/// Should be safe?
pub fn exit(code: i32) -> ! {
  unsafe {
    let syscall_number: u64 = 60;
    asm!(
        "syscall",
        in("rax") syscall_number,
        in("rdi") code,
        options(noreturn)
    )
  }
}

/// prefer io::write! unless you need to do something EXTREMELY silly
pub unsafe fn write(fd: u32, buf: *const u8, count: usize) {
  unsafe {
    let syscall_number: u64 = 1;
    asm!(
        "syscall",
          inout("rax") syscall_number => _,
          in("rdi") fd,
          in("rsi") buf,
          in("rdx") count,
          lateout("rcx") _, lateout("r11") _,
          options(nostack)
    );
  }
}

pub unsafe fn read(fd: u32, buf: *mut u8, count: usize) {
  unsafe {
    let syscall_number: u64 = 0;
    asm!(
        "syscall",
          inout("rax") syscall_number => _,
          in("rdi") fd,
          in("rsi") buf,
          in("rdx") count,
          lateout("rcx") _, lateout("r11") _,
          options(nostack)
    );
  }
}
