.intel_syntax noprefix

.global int_80
int_80:
    int 80
    ret


.global _start
_start:
    call kstart
    int3

.global _call
_call:
    call rdi
    ret