---
layout: default
title: Microsoft ARM Assembler
date: 2024-11-24 13:45:03 +0000
categories: assembly windows
icon: windows
---
## {{ page.title }}
{: .d-flex .justify-content-between .flex-wrap .flex-md-nowrap .align-items-center .pt-3 .pb-2 .mb-3 .border-bottom }

The Microsoft ARM assemblers, `armasm` and `armasm64`, support several command-line options. By default, `armasm` assembles ARMv7 Thumb assembly language into the Microsoft implementation of the Common Object File Format (COFF). The `armasm64` assembler creates COFF object code for ARM64 and ARM64EC targets.

- [ARM Assembler reference](https://learn.microsoft.com/en-us/cpp/assembler/arm/arm-assembler-reference?view=msvc-170)

### Program
{: .pt-4 .border-bottom }

{% highlight armasm %}
    EXPORT WinMainCRTStartup
    AREA .text, CODE

WinMainCRTStartup PROC
    mov     x1, #0
    svc     #44
    ENDP
    END    
{% endhighlight %}

{% highlight terminal %}
armasm64 -machine arm64 program.asm -o project.obj
link program.obj /SUBSYSTEM:windows
{% endhighlight %}