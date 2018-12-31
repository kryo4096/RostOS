.intel_syntax noprefix

.global _start
_start:
    call kernel_init
    call kernel_main
    int3

.global syscall_handler
syscall_handler:
    pushfq
    cli
    push r14
    push r15
    mov r15, rsp
    mov r14, 0x658000000000
    mov rsp, QWORD PTR [r14]
    push r15
    call __syscall
    pop r15
    mov rsp, r15
    pop r15
    pop r14
    popfq
    iretq

.global switch_context
switch_context:
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15
    pushfq
    mov QWORD PTR [rdi], rsp
    mov rsp, QWORD PTR [rsi]
    mov cr3, rdx
    popfq
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    ret

.global _call_signal
_call_signal:
    mov r11, rsp
    mov rsp, r9
    push r11
    call r8
    pop r11
    mov rsp, r11
    ret
