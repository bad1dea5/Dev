---
layout: default
title: "GDB: The GNU Debugger"
date: 2024-12-04 20:17:13 +0000
categories: linux gdb debugger
---
## {{ page.title }}
{: .d-flex .justify-content-between .flex-wrap .flex-md-nowrap .align-items-center .pt-3 .pb-2 .mb-3 .border-bottom }

###### {{ page.date }}
{: .d-flex .justify-content-between .flex-wrap .flex-md-nowrap .align-items-center .pb-3 .mb-4}

The GNU Debugger (GDB) is a portable debugger that runs on many Unix-like systems and works for many programming languages, including Ada, Assembly, C, C++, D, Fortran, Haskell, Go, Objective-C, OpenCL C, Modula-2, Pascal, Rust,[2] and partially others.

- [GDB User Manual](https://sourceware.org/gdb/current/onlinedocs/gdb.html/)

{% highlight terminal %}
gdb ./bin/program

(gdb) set args foo bar

(gdb) b _start                  # breakpoint label
(gdb) b 20                      # breakpoint line number

(gdb) r                         # run
(gdb) c                         # continue
(gdb) s                         # step

(gdb) info registers            # show registers
(gdb) info f                    # show frame
{% endhighlight %}

### Examining Memory
{: .pt-4 .border-bottom }

You can use the command x (for “examine”) to examine memory in any of several formats, independently of your program’s data types.

{% highlight terminal %}
x/nfu addr
x addr
x
{% endhighlight %}

`n`, `f`, and `u` are all optional parameters that specify how much memory to display and how to format it; `addr` is an expression giving the address where you want to start displaying memory. If you use defaults for `nfu`, you need not type the slash ‘/’. Several commands set convenient defaults for addr.

{: .table .table-sm .small .table-borderless .mt-4 }
| `n` | Repeat count | The repeat count is a decimal integer; the default is 1. |
| `f` | Display format | The display format is one of the formats used by [print](https://sourceware.org/gdb/current/onlinedocs/gdb.html/Output-Formats.html#Output-Formats). |
| `u` | Unit size | The unit size is any of `b` bytes `h` short `w` word `g` qword. |

{% highlight terminal %}
(gdb) x 0x7fffffffdef0
0x7fffffffdef0:	0x0000000000000003

(gdb) x/8x 0x7fffffffdef0
0x7fffffffdef0:	0x0000000000000003 0x00007fffffffe24c
0x7fffffffdf00:	0x00007fffffffe284 0x00007fffffffe288
0x7fffffffdf10:	0x0000000000000000 0x00007fffffffe28c
0x7fffffffdf20:	0x00007fffffffe29c 0x00007fffffffe2f2

(gdb) x/1xb 0x7fffffffdef0
0x7fffffffdef0:	0x03
{% endhighlight %}