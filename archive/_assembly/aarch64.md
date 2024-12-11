---
layout: default
title: AArch64
date: 2024-11-13 16:17:17 +0000
categories: aarch64
icon: creative-commons-nd
---
## {{ page.title }}
{: .d-flex .justify-content-between .flex-wrap .flex-md-nowrap .align-items-center .pt-3 .pb-2 .mb-3 .border-bottom }

It was first introduced with the Armv8-A architecture. ARMv8-A represents a fundamental change to the ARM architecture. It adds an optional 64-bit architecture, named __AArch64__, and the associated new __A64__ instruction set.

- [A64 Instruction Set Architecture](https://developer.arm.com/documentation/102374/0101)

### Registers
{: .pt-4 .border-bottom }

The architecture provides 31 general purpose registers. Each register can be used as a 64-bit `X` register, or as a 32-bit `W` register.
When a `w` register is written, the top 32 bits of the 64-bit register are zeroed.

- The zero registers, `xzr` and `wzr`, always read as 0 and ignore writes.
- You can use the stack pointer (`sp`) as the base address for loads and stores.
- The Program Counter (`pc`) is not a general-purpose register in A64.

{: .table .table-sm .small .table-borderless }
| Register(s) | Size | Description |
| --- | --- | :--- |
| `Rn` | 64 | General register (`0-30`) |
| `Xn` | 64 | General register (`0-30`) |
| `Wn` | 32 | General register (`0-30`) |
| `XZR` | 64 | Zero register |
| `WZR` | 32 | Zero register |
| `SP` | 64 | Stack register |
| `PC` | 64 | Program counter; incremented by four bytes |

There is a separate set of 32 registers (`v0-v31`) used for floating point and vector operations. These registers are 128-bit, and like the general-purpose registers, can be accessed in several ways. When the `V` form is used, the register is treated as being a vector.

{: .table .table-sm .small .table-borderless }
| Register(s) | Size | Description |
| --- | --- | :--- |
| `Vx` | 128 | |
| `Qx` | 128 | |
| `Dx` | 64 | |
| `Sx` | 32 | |
| `Hx` | 16 | |
| `Bx` | 8 | |

### Procedure Call Standard
{: .pt-4 .border-bottom }

- The first seven argments are passed into the `X` registers, with any further arguments are passed on the stack.
- In C++, `X0` is used to pass the implicit this pointer that points to the called function.

{: .table .table-sm .small .table-borderless }
| Register(s) | Significance | Description |
| --- | --- | :--- |
| `R0-R7` | | Parameter and result |
| `R8` | `XR` | Indirect result |
| `R9-R15` | | Corruptible |
| `R16` | `IP0` | Intra-procedure-call corruptible |
| `R17` | `IP1` | Intra-procedure-call corruptible |
| `R18` | `PR` | Platform |
| `R19-R23` | | Callee-saved |
| `R24-R28` | | Callee-saved |
| `R29` | `FP` | Frame |
| `R30` | `LR` | Link |

In Arch64 state, the PC is not a general purpose register and you cannot access it explicitly. The following types of instructions read it implicitly:

- Instructions that compute a PC-relative address.
- PC-relative literal loads.
- Direct branches to a PC-relative label.
- Branch and link instructions, which store it in the procedure link register.

The only types of instructions that can write to the PC are:

- Conditional and unconditional branches.
- Exception generation and exception returns.

### System Calls
{: .pt-4 .border-bottom }

There are special instructions for making such system calls. These instructions cause an exception, which allows 
controlled entry into a more privileged Exception level.

- `SVC` Supervisor call - Causes an exception targeting `EL1`.
- `HVC` Hypervisor call - Causes an exception targeting `EL2`.
- `SMC` Secure monitor call - Causes an exception targeting `EL3`.

__Linux__
{% highlight armasm %}
mov	x8, #93
svc	#0
{% endhighlight %}

__Windows__
{% highlight armasm %}
svc	#44
{% endhighlight %}

### Branch
{: .pt-4 .border-bottom }

{: .table .table-sm .small .table-borderless }
| Instruction | Description |
| --- | :--- |
| `B` | An unconditional branch to a label. |
| `B.cc` | A conditionally branch. ([see condition code suffixes](#condition-code-suffixes)) |
| `BL` | Branch with Link to a PC-relative offset, setting the register `X30` to `PC+4`. |
| `BLR` | Branch with Link to Register calls a subroutine at an address in a register, setting register `X30` to `PC+4`. |
| `BR` | Branch to Register branches unconditionally to an address in a register. |
| `BLRAA`, `BLRAAZ`, `BLRAB`, `BLRABZ` ||
| `BRAA`, `BRAAZ`, `BRAB`, `BRABZ` ||

### Condition code suffixes
{: .pt-4 .border-bottom }

Instructions that can be conditional have an optional two character condition code suffix.

{: .table .table-sm .small .table-borderless }
| Suffix | Description | Identical to |
| --- | :--- | :--- |
| `EQ` | Equal |
| `NE` | Not equal |
| `CS` | Carry set | `HS` |
| `HS` | Unsigned higher or same | `CS` |
| `CC` | Carry clear | `LO` |
| `LO` | Unsigned lower | `CC` |
| `MI` | Minus or negative result |
| `PL` | Positive or zero result |
| `VS` | Overflow |
| `VC` | No overflow |
| `HI` | Unsigned higher |
| `LS` | Unsigned lower or same |
| `GE` | Signed greater than or equal |
| `LT` | Signed less than |
| `GT` | Signed greater than |
| `LE` | Signed less than or equal |
| `AL` | Always (default) |

### For Loop
{: .pt-4 .border-bottom }

{% highlight armasm %}
    sub     sp, sp, 0x10
    str     wzr, [sp, 0xc]      // int i = 0
    b       loop_compare

loop:
    ldr     w0, [sp, 0xc]
    add     w0, w0, 0x1         // i++
    str     w0, [sp, 0xc]

loop_compare:
    ldr     w0, [sp, 0xc]
    cmp     w0, 0xff            // i < 256
    ble     loop

    add     sp, sp, 0x10
{% endhighlight %}
