#
#
#

.bss
    .lcomm crc64_lookup_table, 256 * 8

.text
    #
    #   rdi     source      char*
    #   rsi     length      uint64
    #
    crc64:
        sub     $0x30, %sp

        movq    %rdi, 0x28(%rsp)
        movq    %rsi, 0x20(%rsp)
        movq    $0, 0x18(%rsp)
        movq    $0, 0x10(%rsp)

        jmp     loop_compare

    next_byte:
        movq    0x18(%rsp), %rax
        shr     $56, %rax
        movq    %rax, %rcx
        movq    0x28(%rsp), %rax
        leaq    0x1(%rax), %rdx
        movq    %rdx, 0x28(%rsp)

        movzb   (%rax), %eax
        movsb   %al, %rax
        xorq    %rcx, %rax
        andl    $255, %eax
        movq    %rax, 0x8(%rsp)

        movq    0x8(%rsp), %rax
        movq    crc64_lookup_table(,%rax,8), %rax
        movq    0x18(%rsp), %rdx
        salq    $8, %rdx
        xorq    %rdx, %rax
        movq    %rax, 0x18(%rsp)
        addq    $1, 0x10(%rsp)

    loop_compare:
        movq    0x10(%rsp), %rax
        cmpq    0x20(%rsp), %rax
        jb      next_byte

        movq    0x18(%rsp), %rax

        add     $0x30, %sp
        ret
    
    #
    #
    #
    crc64_init:
        sub     $0x20, %sp

        movl    $0, 0x18(%rsp)
        jmp     init_loop_compare

    init_loop:
        movq    $0, 0x10(%rsp)

        movl    0x18(%rsp), %eax
        cltq
        salq    $56, %rax
        movq    %rax, 0x8(%rsp)

        movl    $0, 0x1c(%rsp)
        jmp     init_inner_compare

    init_inner_loop:
        movq    0x10(%rsp), %rax
        xorq    0x8(%rsp), %rax
        test    %rax, %rax
        jns     init_shift_crc

        movq    0x10(%rsp), %rax
        leaq    (%rax,%rax), %rdx
        movabs  $0x42f0e1eba9ea3693, %rax
        xorq    %rdx, %rax
        mov     %rax, 0x10(%rsp)

        jmp     init_shift_mask

    init_shift_crc:
        salq    0x10(%rsp)

    init_shift_mask:
        salq    0x8(%rsp)
        addl    $1, 0x1c(%rsp)

    init_inner_compare:
        cmpl    $7, 0x1c(%rsp)
        jle     init_inner_loop

        movl    0x18(%rsp), %eax
        cltq
        movq    0x10(%rsp), %rdx
        movq    %rdx, crc64_lookup_table(,%rax,8)

        addl    $1, 0x18(%rsp)

    init_loop_compare:
        cmpl    $255, 0x18(%rsp)
        jle     init_loop

        add     $0x20, %sp
        ret
