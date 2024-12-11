---
layout: default
title: Rust no_std application
date: 2024-11-12 22:00:00 +0000
categories: rust
---
## {{ page.title }}
{: .d-flex .justify-content-between .flex-wrap .flex-md-nowrap .align-items-center .pt-3 .pb-2 .mb-3 .border-bottom }

###### {{ page.date }}
{: .d-flex .justify-content-between .flex-wrap .flex-md-nowrap .align-items-center .pb-3 .mb-4}

`#![no_std]` is a crate-level attribute that indicates that the crate will 
link to the core-crate instead of the std-crate. The libcore crate in turn
is a platform-agnostic subset of the std crate which makes no assumptions
about the system the program will run on. As such, it provides APIs for 
language primitives like floats, strings and slices, as well as APIs that
expose processor features like atomic operations and SIMD instructions.

- [Documentation](https://www.rust-lang.org/learn)

### Project
{: .pt-4 .border-bottom }

{% highlight terminal %}
> cargo new shared --lib --vcs none
> cargo new program --bin --vcs none

> touch Cargo.toml .cargo/config.toml
> touch shared/src/_start.s
{% endhighlight %}

{% highlight toml %}
# Cargo.toml

[workspace]
members = ["shared", "program"]
resolver = "2"

[profile.dev]
panic = "abort"
debug = true
incremental = true
lto = false
opt-level = 0

[profile.release]
panic = "abort"
debug = false
incremental = false
lto = true
opt-level = 3
codegen-units = 1
strip = "symbols"
{% endhighlight %}

{% highlight toml %}
# .config/config.toml

[target.'cfg(target_family="unix")']
rustflags=[
    "-Cdefault-linker-libraries=off",
    "-Clink-arg=-nostdlib"
]

[target.'cfg(target_family="windows")']
rustflags=[
    "-Clink-arg=/NODEFAULTLIB",
    "-Clink-arg=/SUBSYSTEM:WINDOWS",
    "-Clink-arg=/ENTRY:_start"
]
{% endhighlight %}

{% highlight nasm %}
; shared/src/_start.s

.global _start

.text
    _start:
        xor     rbp, rbp
        call    start
        mov     edi, eax
        mov     eax, 0xe7
        syscall
        hlt
{% endhighlight %}

### Shared
{: .pt-4 .border-bottom }

{% highlight rust %}
# shared/lib.rs

#![no_std]

use core::arch::global_asm;
use core::panic::PanicInfo;

#[cfg(all(target_arch="x86_64", target_os="linux"))]
global_asm!(include_str!("_start.s"));

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn start() -> i32 {
    0
}

pub fn foo() -> i32 {
    256
}
{% endhighlight %}

### Program
{: .pt-4 .border-bottom }

{% highlight toml %}
# program/Cargo.toml

[dependencies]
shared = { path = "../shared" }
{% endhighlight %}

{% highlight rust %}
# program/main.rs

#![no_std]
#![no_main]

use shared;

#[no_mangle]
extern fn main() -> i32 {
    shared::foo()
}
{% endhighlight %}

### Build
{: .pt-4 .border-bottom }

{% highlight terminal %}
> cargo build

Compiling shared v0.1.0 (/media/dev/project/shared)
Compiling program v0.1.0 (/media/dev/project/program)

Finished dev [unoptimized + debuginfo] target(s) in 0.11s
{% endhighlight %}
