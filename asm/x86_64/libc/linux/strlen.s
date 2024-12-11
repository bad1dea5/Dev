.global strlen

.text
    #
    #    rdi =   char*     source
    #    rsi =   size_t    max
    #    rdx =   size_t*   length
    #
    strlen:
        sub     $0x20, %sp

        mov     %rdi, 0x18(%rsp)
        mov     %rsi, 0x10(%rsp)
        mov     %rdx, 0x8(%rsp)

        mov     0x10(%rsp), %rax
        mov     %rax, 0x0(%rsp)

        cmpq    $0x0, 0x18(%rsp)
        je      exit_failure

        cmpq    $0x0, 0x10(%rsp)
        jne     compare_byte

    exit_failure:
        movl    $-1, %eax
        jmp     exit

    loop_byte:
        addq    $0x1, 0x18(%rsp)
        subq    $0x1, 0x10(%rsp)

    compare_byte:
        cmpq    $0x0, 0x10(%rsp)
        je      is_max_zero

        mov     0x18(%rsp), %rax
        movzbl  (%rax), %eax
        testb   %al, %al
        jne     loop_byte

    is_max_zero:
        cmpq    $0x0, 0x10(%rsp)
        jne     count
        jmp     exit_failure

    count:
        cmpq    $0x0, 0x8(%rsp)
        je      exit_success

        mov     0x0(%rsp), %rax
        sub     0x10(%rsp), %rax

        mov     %rax, %rdx
        mov     0x8(%rsp), %rax
        mov     %rdx, (%rax)

    exit_success:
        movl    $0x0, %eax

    exit:
        add     $0x20, %sp
        ret
