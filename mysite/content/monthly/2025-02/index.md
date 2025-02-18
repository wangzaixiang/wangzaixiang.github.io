+++
title = "February 2025"
description = "2025-02 monthly digest"
date = 2025-02-06
draft = false
template = "blog/page.html"
+++

# Languages
1. [rust-kperf](https://github.com/El-Naizin/rust-kperf) MacOS 下的未公开的API 的逆向工程。
2. [ARM SVE2 架构基础](https://developer.arm.com/documentation/102340/0100/SVE2-architecture-fundamentals) 
   SVE 指令集的设计确实是一个创新，可以支持任意的向量长度而无需修改代码。相比 SSE/AVX/AVX2/AVX512 等指令集，SVE 要优美得多。
3. [What I like and unlike for Zig](https://strongly-typed-thoughts.net/blog/zig-2025)
   - Likes
      - arbitrary width integers and packed structs
      - generic types are functions at the type level
      - error union types
      - c interop
      - the build system
   - Unlikes
      - error handling, than Rust's Result
      - shadowing is forbidden
      - compile time duck typing: anytype
      - no typeclasses/traits
      - comptime is probably not interesting as it looks
      - non encapsulion
      - memory safety is highly underestimated and fallacious
      - lazy compilation
      - no destructors
      - no (unicode) strings.
4. [QBE](https://c9x.me/compile/) QBE is a compiler backend that aims to provide 70% of the performance of industrial optimizing compilers in 
   10% of the code. 

   - like LLVM IR, but simpler.
   - ~10k lines of c code. (vs 1M lines of LLVM), 代码量少，Pure C，是一个很好的学习编译期后端的参考。
   - support for amd64 (linux and osx), arm64, and riscv64.

# MPP & OLAP

# Web & Visualization
1. [The State of WASM - 2024 and 2025](https://platform.uno/blog/state-of-webassembly-2024-2025/)
   - ESM integration, `import {add} from './my-math.wasm'`

# Tools & Libraries