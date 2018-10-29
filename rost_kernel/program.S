bits 64

xor r15, r15

spin:
mov rdi, 10h
int 80h
mov rsi, rax

cmp rax, r15

jl spin

add r15, 10h
mov rdi, 2h
int 80h

jmp spin

data:
db 'Hello, Kernel!'

heap_start:
