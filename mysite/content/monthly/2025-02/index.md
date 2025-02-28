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
5. [LLVM指令选择](https://www.zhihu.com/question/500409301/answer/3210484073)
   
   指令选择过程以 IR 为输入，输出一系列使用无限寄存器的指令，分为如下阶段：
   - 构建初始 Selection DAG
   - 优化
   - 类型合法化
   - 优化
   - 操作合法化
   - 优化
   - 目标指令选择
   - 调度核形成。

   > 注意：您可以通过在调用llc时作为命令行参数传递-view-dag-combine1-dags、-view-legalize-dags、-view-dag-combine2-dags、
   > -view-isel-dags或-view-sched-dags来告诉LLVM在指令选择过程的各个阶段生成选择DAG的可视化表示。
   > -debug参数告诉llc生成选择DAG的文本表示。

6. [makemore](https://github.com/karpathy/makemore) 语言生成模型，transformer 的简单版本。
7. [Rust Edition 2024](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html) 又一个大版本来了。

# MPP & OLAP

# Web & Visualization
1. [The State of WASM - 2024 and 2025](https://platform.uno/blog/state-of-webassembly-2024-2025/)
   - ESM integration, `import {add} from './my-math.wasm'`

# Tools & Libraries