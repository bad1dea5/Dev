---
layout: default
title: Microsoft Macro Assembler
date: 2024-11-19 17:21:43 +0000
categories: assembly windows
icon: windows
---
## {{ page.title }}
{: .d-flex .justify-content-between .flex-wrap .flex-md-nowrap .align-items-center .pt-3 .pb-2 .mb-3 .border-bottom }

The Microsoft Macro Assembler (MASM) is an x86 assembler that uses the Intel syntax for MS-DOS and Microsoft Windows. 
Beginning with MASM 8.0, there are two versions of the assembler: One for 16-bit & 32-bit assembly sources, 
and another (ML64) for 64-bit sources only.

- [ML64 command-line Reference](https://learn.microsoft.com/en-us/cpp/assembler/masm/ml-and-ml64-command-line-reference?view=msvc-170)
- [Linker Options](https://learn.microsoft.com/en-us/cpp/build/reference/linker-options?view=msvc-170)
- [Directives Reference](https://learn.microsoft.com/en-us/cpp/assembler/masm/directives-reference?view=msvc-170)

### Program
{: .pt-4 .border-bottom }

{% highlight nasm %}
public WinMainCRTStartup

_text segment
    WinMainCRTStartup:
        push    rcx
        sub     rsp, 20h

        add     rsp, 20h
        pop     rcx
        xor     edx, edx
        mov     eax, 2ch
        syscall
        ret
_text ends

end
{% endhighlight %}

{% highlight terminal %}
ml64 /Fo program.obj -c program.asm
link /SUBSYSTEM:windows program.obj
{% endhighlight %}

### BSS Segment
{: .pt-4 .border-bottom }

The BSS segment contains uninitialized static data, both variables and constants,
i.e. global variables and local static variables that are initialized to zero or do
not have explicit initialization in source code.

{% highlight nasm %}
_bss segment
    foo dword 0
    bar qword 0
_bss ends
{% endhighlight %}

### DATA Segment
{: .pt-4 .border-bottom }

The data segment contains initialized static variables, that is, global variables and static local variables. 
The data segment is read/write, since the values of variables can be altered at run time.

{% highlight nasm %}
_data segment
_data ends
{% endhighlight %}
