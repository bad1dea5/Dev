extern "C" void _start()
{
    asm(
        "movl   $0x0, %edi \n"
        "movl   $0xe7, %eax \n"
        "syscall"
    );
}
