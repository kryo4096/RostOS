bits 64

mov rsi, data
mov rdx, 14
mov rdi, 0h
int 80h

spin:
jmp spin


data:
db 'Hello, Kernel!'
