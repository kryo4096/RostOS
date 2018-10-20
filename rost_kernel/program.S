bits 64

mov rdi, 0
lea rsi, [data]
mov rdx, 14
int 80h

spin: 
jmp spin

data:
db 'Hello, Kernel!'
