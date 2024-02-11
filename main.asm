format elf64

section ".text" executable

public _start

_start:
    mov rax, 60 ;; SYS_exit
    mov rdi, 0

    syscall

    ret

section ".data" writeable
section ".bss" writeable
