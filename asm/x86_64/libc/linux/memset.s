.global memset

.text
    #
    # arguments
    #   rdi     void*   dest
    #   esi     int     ch
    #   rdx     size_t  count
    #
    memset:
        sub     $0x20, %sp

        movq    %rdi, 0x0(%rsp)
        movl    %esi, 0x8(%rsp)
        movq    %rdx, 0x10(%rsp)
        movq    0x0(%rsp), %rax
        movq    %rax, 0x18(%rsp)
        jmp     loop_check

    loop:
        movl    0x8(%rsp), %eax
        movl    %eax, %ecx
        movq    0x18(%rsp), %rax
        leaq    0x1(%rax), %rdx
        movq    %rdx, 0x18(%rsp)
        movb    %cl, (%rax)

    loop_check:
        movq    0x10(%rsp), %rax
        leaq    -0x1(%rax), %rdx
        movq    %rdx, 0x10(%rsp)
        testq   %rax, %rax
        setne   %al
        testb   %al, %al
        jne     loop
        movq    0x0(%rsp), %rax

        add     $0x20, %sp
        ret
