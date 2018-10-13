.intel_syntax noprefix

.global int_80
int_80:
    int 80
    ret


.global _start
_start:
    call kmain

.global test_code
test_code:
    mov rax, 409600
    call rax