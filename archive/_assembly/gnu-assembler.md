---
layout: default
title: GNU Assembler
date: 2024-11-13 16:51:23 +0000
categories: assembly linux
icon: linux
---
## {{ page.title }}
{: .d-flex .justify-content-between .flex-wrap .flex-md-nowrap .align-items-center .pt-3 .pb-2 .mb-3 .border-bottom }

The GNU Assembler, commonly known as `gas` or `as`, is the assembler developed by the GNU Project. 
It is the default back-end of GCC, and is part of the GNU Binutils package. 
The default syntax for `as` is AT&amp;T, but it can be used with Intel syntax by using the `.intel_syntax` directive.

- [assembler](https://sourceware.org/binutils/docs-2.38/as/)
- [linker](https://sourceware.org/binutils/docs/ld/)

### Program
{: .pt-4 .border-bottom }

{% highlight nasm %}
.global _start

.text
    _start:
        xor     %rbp, %rbp
        sub     $0x20, %sp

        add     $0x20, %sp

        xor     %edi, %edi
        mov     $231, %ax
        syscall
        hlt
{% endhighlight %}

{% highlight terminal %}
as -c program.s -o program.o
ld program.o -o program
{% endhighlight %}

### BSS Segment
{: .pt-4 .border-bottom }

The BSS segment contains uninitialized static data, both variables and constants,
i.e. global variables and local static variables that are initialized to zero or do
not have explicit initialization in source code.

{% highlight nasm %}
.bss
    .comm foo, 8
    .lcomm bar, 8

movl    $0xbad1dea5, foo
movq    $foo, -0x8(%rsp)

mov     $0xd15ab1ed, bar
mov     $bar, %rax
{% endhighlight %}

### DATA Segment
{: .pt-4 .border-bottom }

The data segment contains initialized static variables, that is, global variables and static local variables. 
The data segment is read/write, since the values of variables can be altered at run time.

{% highlight nasm %}
.data
    foo: .ascii "BAD1DEA5"
    bar: .octa 0xbad1dea5

mov     $foo, %rdi
mov     $bar, %esi
{% endhighlight %}

### Macro
{: .pt-4 .border-bottom }

{% highlight nasm %}
.macro foo
    nop
.endm

.macro bar, from=0, to:req
    nop
    .if \to-\from
        bar "(\from+1)",\to
    .endif
.endm

foo
bar     ,5
{% endhighlight %}
