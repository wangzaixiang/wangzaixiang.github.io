+++
title = "Jul 2025"
description = "Jul 2025"
date = 2025-07-07
draft = false
template = "blog/page.html"
+++

# Languages
1. rust
   - [https://sharnoff.io/blog/why-rust-compiler-slow](Why is the Rust compiler so slow?)
     - ![cargo build --timings](img.png)
     - ![opt-level](img_1.png)
     - debug , LTO, optimization
     - `RUSTFLAGS="-Cllvm-args=-inline-threshold=50 -Cllvm-args=-inlinedefault-threshold=50 -Cllvm-args=-inlinehint-threshold=50"`
     - Reducing inlining with LLVM args;
     - Breaking up expensive functions in the main crate; and
     - Removing generics from dependencies to prevent needing to compile it in the main crate
     Timeline:
     -  We started at ~175s
     -  Disabling LTO (and debug symbols!) got us to 51s (-71%)
     -  Changing to opt-level = 1 on the final crate got us to 48.8s (-4%)
     -  Reducing inlining with -C llvm-args got us to 40.7s (-16%)
     -  Local changes got us to 37.7s (-7%)
     -  Changes with dependencies got us to 32.3s (-14%)
     -  Enabling -Zshare-generics got us to 29.1s (-10%)
     -  And switching away from alpine got us to 9.1s (-69%)
   - [Leaktracer: A Rust allocator to trace memory allocations](https://blog.veeso.dev/blog/en/leaktracer-a-rust-allocator-to-trace-memory-allocations/)

# MPP & OLAP

# Web & Frontend

# Tools & Libraries