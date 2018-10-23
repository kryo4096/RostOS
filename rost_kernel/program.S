bits 64

spin:
    call time

    mov rsi, time
    call print
jmp spin

print:
mov rdi, 2
int 80h
ret

time:
mov rdi, 10
int 80h
ret

data:
db 'Hello, Kernel!'
