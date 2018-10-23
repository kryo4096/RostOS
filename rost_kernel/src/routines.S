.intel_syntax noprefix

.global int_80
int_80:
    int 80
    ret


.global _start
_start:
    call kstart
    int3

.global proc_start
proc_start:
    call rdi
    mov rsp, rsi
    ret