.global _start

.text
    _start:
        xor     %rbp, %rbp
        sub     $32, %sp

        lea     32(%rsp), %rdi
        call    startup

        add     $32, %sp
        
        mov     %eax, %edi
        mov     $231, %eax
        syscall
        hlt

        call    _exit
