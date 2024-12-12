#include <cstdio>

#include <elf.h>
#include <link.h>

void exit(int status)
{
    asm(
        "movl   %[status], %%edi \n"
        "movl   $0xe7, %%eax \n"
        "syscall"
        :: [status] "g" (status)
    );
}

extern "C" void _start()
{
    Elf64_Dyn* dynamic = reinterpret_cast<Elf64_Dyn*>(_DYNAMIC);

    std::printf("DYNAMIC address : %p\n\n", (void*)_DYNAMIC);

    while (dynamic->d_tag)
    {
        if (dynamic->d_tag == DT_DEBUG)
        {
            r_debug* debug = reinterpret_cast<r_debug*>(dynamic->d_un.d_ptr);
            link_map* map = reinterpret_cast<link_map*>(debug->r_map);

            while (map) {
                std::printf("name   : %s\n", map->l_name);
                std::printf("l_addr : %p\n", (void*)map->l_addr);
                std::printf("l_name : %p\n", (void*)map->l_name);
                std::printf("l_ld   : %p\n", (void*)map->l_ld);
                std::printf("l_next : %p\n", (void*)map->l_next);
                std::printf("l_prev : %p\n\n", (void*)map->l_prev);
                map = map->l_next;
            }
        }
        dynamic++;
    }

    exit(1);
}
