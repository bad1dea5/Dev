public WinMainCRTStartup

_text segment
    WinMainCRTStartup:
        xor     edx, edx
        mov     eax, 2ch
        syscall
        ret
_text ends

end