; Custom ELF header for the binary.
; Adapted from https://github.com/kmcallister/tiny-rust-demo

    bits 64
    org 0x00400000

ehdr:
    db  0x7f, "ELF"  ; magic
    db  2, 1         ; 64-bits, little endian
    db  1            ; ELF version 1
    db  0            ; OS = Unix

    dd 0, 0          ; Padding

    dw  2            ; e_type = executable
    dw  0x3e         ; e_machine = x86-64
    dd  1            ; e_version
    dq  entry        ; e_entry
    dq  phdr - $$    ; e_phoff
    dq  0            ; e_shoff
    dd  0            ; e_flags
    dw  ehdrsize     ; e_ehsize
    dw  phdrsize     ; e_phentsize
    dw  1            ; e_phnum
    dw  0, 0, 0      ; e_sh*

ehdrsize  equ  $ - ehdr

phdr:
    dd  1            ; p_type = loadable program segment
    dd  7            ; p_flags = rwx
    dq  0            ; p_offset
    dq  $$, $$       ; p_vaddr, p_paddr
    dq  filesize     ; p_filesz
    dq  filesize     ; p_memsz
    dq  0x1000       ; p_align

phdrsize equ  $ - phdr

incbin "payload.bin"

filesize equ $ - ehdr

; vim: ft=tasm
