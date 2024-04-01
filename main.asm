format elf64

section ".text" executable

public _start

_start:
    push 1
    push 2

    pop rax
    pop rbx
    add rax, rbx

    push rax

    mov rax, 60 ;; SYS_exit
    mov rdi, 0

    syscall

    ret

section ".data" writeable
section ".bss" writeable
