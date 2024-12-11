.global _start

.text
    _start:
        movl    $0x0, %edi
        movl    $0xe7, %eax
        syscall
        hlt
