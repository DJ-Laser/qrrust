use core::arch::asm;

pub unsafe fn exit(code: i32) -> ! {
  let syscall_number: u64 = 60;
  asm!(
      "syscall",
      in("rax") syscall_number,
      in("rdi") code,
      options(noreturn)
  )
}

pub unsafe fn write(fd: u32, buf: *const u8, count: usize) {
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
