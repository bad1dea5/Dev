.global _start

.text
    _start:
        mov     w0, 0x0
        mov     w8, 0x5d
        svc     #0
