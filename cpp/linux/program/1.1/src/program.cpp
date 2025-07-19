//
//
//

#include <iostream>

[[noreturn]] void exit(int status)
{
    asm(
        "movl %[status], %%edi\n"
        "movl $231, %%eax\n"
        "syscall \n"
        "hlt"
        :
        : [status] "g" (status)
        : "eax", "edi"
    );

    while(true){}
}

extern "C" void _start(int argc, char** argv)
{    
    exit(0);
}
