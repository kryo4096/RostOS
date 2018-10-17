bits 64

mov rdi, 0
mov rsi, str
int 80h
ret

int3

str:
db 'Hello, World!'
db 0
