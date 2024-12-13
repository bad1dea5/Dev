.global WinMainCRTStartup

.text
    WinMainCRTStartup:
        xor     rbp, rbp
        mov     r10, rsp

        push    rcx
        sub     sp, 0x20
        
        mov     rcx, [rsp + 0x20]
        mov     qword ptr [rip + process_environment], rcx
        
        mov     rcx, r10
        call    startup

        add     sp, 0x20
        pop     rcx

        mov     r10, 0
        mov     edx, eax
        mov     rax, 0x2c
        syscall
        ret

.bss
    .comm process_environment, 8
