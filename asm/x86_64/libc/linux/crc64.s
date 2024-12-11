.global crc64
.global crc64_init

.bss
    .lcomm crc64_lookup_table, 256 * 8

.text
    crc64_init:
        sub     $0x20, %sp

        movl    $0x0, 0x0(%rsp)                     # i = 0
        jmp     crc64_init_loop_compare

    crc64_init_loop:
        movq    $0x0, 0x8(%rsp)                     # crc = 0
        mov     0x0(%rsp), %eax
        cltq
        salq    $0x38, %rax                         # i << 56
        mov     %rax, 0x10(%rsp)                    # mask = (i << 56)

        movl    $0x0, 0x4(%rsp)                     # j = 0
        jmp     crc64_init_inner_compare

    crc64_init_inner_loop:
        mov     0x8(%rsp), %rax                     # crc
        xor     0x10(%rsp), %rax                    # mask
        test    %rax, %rax
        jns     crc64_init_shift_crc

        mov     0x8(%rsp), %rax
        lea     (%rax,%rax), %rdx
        movabs  $0x42f0e1eba9ea3693, %rax
        xor     %rdx, %rax
        mov     %rax, 0x8(%rsp)
        jmp     crc64_init_shift_mask

    crc64_init_shift_crc:
        salq    0x8(%rsp)                           # crc <<= 1

    crc64_init_shift_mask:
        salq    0x10(%rsp)                          # mask <<= 1
        addl    $0x1, 0x4(%rsp)                     # j++

    crc64_init_inner_compare:
        cmpl    $0x7, 0x4(%rsp)                     # j == 7
        jle     crc64_init_inner_loop

        mov     0x0(%rsp), %eax
        cltq

        mov     0x8(%rsp), %rdx
        mov     %rdx, crc64_lookup_table(,%rax,8)   # crc64_table[i] = crc

        addl    $0x1, 0x0(%rsp)                     # i++

    crc64_init_loop_compare:
        cmpl    $0xff, 0x0(%rsp)                    # i == 255
        jle     crc64_init_loop

        add     $0x20, %sp
        ret

    #
    #    rdi =   char*     source
    #    rsi =   size_t    length
    #
    crc64:
        sub     $0x30, %sp

        mov     %rdi, 0x0(%rsp)
        mov     %rsi, 0x8(%rsp)
        movq    $0x0, 0x10(%rsp)
        movq    $0x0, 0x18(%rsp)
        jmp     crc64_loop_compare

    crc64_byte_loop:
        # byte = ((r >> 56) ^ (*src++)) & 0xff

        mov     0x10(%rsp), %rax
        shr     $0x38, %rax
        mov     %rax, %rcx
        mov     0x0(%rsp), %rax
        lea     0x1(%rax), %rdx
        mov     %rdx, 0x0(%rsp)
        movzb   (%rax), %eax
        movsb   %al, %rax
        xor     %rcx, %rax
        andl    $0xff, %eax
        mov     %rax, 0x20(%rsp)

        # r = crc64_table[byte] ^ (r << 8)

        mov     0x20(%rsp), %rax
        mov     crc64_lookup_table(,%rax,8), %rax
        mov     0x10(%rsp), %rdx
        salq    $0x8, %rdx
        xor     %rdx, %rax
        mov     %rax, 0x10(%rsp)
        addq    $0x1, 0x18(%rsp)

    crc64_loop_compare:
        mov     0x18(%rsp), %rax
        cmp     0x8(%rsp), %rax
        jb      crc64_byte_loop

        mov     0x10(%rsp), %rax
        add     $0x30, %sp
        ret
