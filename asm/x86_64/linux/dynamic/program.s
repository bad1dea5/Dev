.global _start

.data
    format: .ascii "[+] %#x %s\n"

.text
    _start:
        sub     $0x20, %sp

        lea     _DYNAMIC, %rax
        movq    %rax, 0x18(%rsp)    # address of _DYNAMIC section

        jmp     loop

    inner_loop:
        movq    0x18(%rsp), %rax
        movq    (%rax), %rax        # dynamic->d_tag
        cmpq    $0x15, %rax         # DT_DEBUG
        jne     increment_loop
        
        movq    0x18(%rsp), %rax
        movq    0x08(%rax), %rax
        movq    %rax, 0x10(%rsp)    # r_debug

        movq    0x10(%rsp), %rax
        movq    0x08(%rax), %rax
        movq    %rax, 0x08(%rsp)    # link_map

        #
        jmp     compare_link

    next_link:
        movq    0x08(%rsp), %rax
        movq    0x08(%rax), %rdx    # link_map->l_name

        movq    0x08(%rsp), %rax
        movq    (%rax), %rsi        # link_map->l_addr

        movq    $format, %rdi
        call    printf

        movq    0x08(%rsp), %rax
        movq    0x18(%rax), %rax
        movq    %rax, 0x08(%rsp)    # advance link_map


    compare_link:
        cmpq    $0x0, 0x08(%rsp)
        jne     next_link

    increment_loop:
        addq    $0x10, 0x18(%rsp)   # advance section

    loop:
        movq    0x18(%rsp), %rax
        movq    (%rax), %rax
        test    %rax, %rax
        jne     inner_loop

        add     $0x20, %sp
        movl    $0x00, %edi
        movl    $0xe7, %eax
        syscall
        hlt
