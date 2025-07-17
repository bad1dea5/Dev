#
#
#

.global _start

.data
    message: .asciz "%#llx\t%#llx\t%s\n"

.text
    _start:
        sub     $0x20, %sp

        leaq    _DYNAMIC, %rax
        movq    %rax, 0x18(%rsp)

        call    crc64_initialize

        jmp     is_dynamic_section_null

    main_loop:
        movq    0x18(%rsp), %rax
        movq    (%rax), %rax
        cmpq    $21, %rax
        jne     increment_dynamic_section

        movq    0x18(%rsp), %rax
        movq    0x8(%rax), %rax
        movq    %rax, 0x10(%rsp)

        movq    0x10(%rsp), %rax
        movq    0x8(%rax), %rax
        movq    %rax, 0x8(%rsp)

        jmp     is_debug_section

    inner_loop:
        #   0x8(%rsp), %rax -> l_addr
        #   0x8(%rax), %Rn  -> l_name*
        #   0x10(%rax), %Rn -> l_ld*
        #   0x18(%rax), %Rn -> l_next*
        #   0x20(%rax), %Rn -> l_prev*

        movq    0x8(%rsp), %rax
        movq    0x8(%rax), %rdi
        movq    $255, %rsi
        leaq    0x0(%rsp), %rdx
        call    strlen_s

        cmpq    $0, 0x0(%rsp)
        jz      increment_link_map

        movq    0x8(%rsp), %rax
        movq    0x8(%rax), %rdi
        movq    0x0(%rsp), %rsi
        call    crc64_generate

        movq    %rax, %rdx              # crc64
        movq    0x8(%rsp), %rax
        movq    0x8(%rax), %rcx         # name
        movq    0x8(%rsp), %rax
        movq    (%rax), %rsi            # addr
        movq    $message, %rdi
        call    printf

    increment_link_map:
        movq    0x8(%rsp), %rax
        movq    0x18(%rax), %rax
        movq    %rax, 0x8(%rsp)

    is_debug_section:
        cmpq    $0, 0x8(%rsp)
        jne     inner_loop

    increment_dynamic_section:
        addq    $0x10, 0x18(%rsp)

    is_dynamic_section_null:
        movq    0x18(%rsp), %rax
        movq    (%rax), %rax
        test    %rax, %rax
        jne     main_loop

        add     $0x20, %sp
        
        movl    $0, %edi
        movl    $231, %eax
        syscall
        hlt
