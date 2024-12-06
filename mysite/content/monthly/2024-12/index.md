+++
title = "December 2024"
description = "2024-12 monthly notes"
date = 2024-12-01
draft = false
template = "blog/page.html"

[extra]
toc = true
+++

# Languages
1. Zig Compiler Internals
    - [Tokenizer](https://mitchellh.com/zig/tokenizer): bytes -> token
      tokenizer 过程不使用 heap.
    - [Parser](https://mitchellh.com/zig/parser): token -> AST
      1. 使用 MultiArrayList 作为存储数据结构（structure of array）:减少内存占用，缓存友好。（使用 macro 可以简化 SOA 的 api）
    - [AstGen: AST => ZIR](https://mitchellh.com/zig/astgen) AST => ZIR( Zig IR)
      1. `zig ast-check -t <file>` 可以查看 ZIR
      2. zir is untyped 
    - [Zig Sema](https://mitchellh.com/zig/sema): ZIR -> AIR (Analyzed IR) The heart of zig compiler.
      1. `zig build-obj --verbose-air <file>`
      2. comptime evaluation
      3. AIR is fully typed
      4. AIR is only generated for exported or referenced functions.
      5. Value: comptime-known value.
      6. Type: comptime-known type(all types are comptime-known). a type can be a value of type `type`.
      7. TypedValue: value with type.
      
      理解 comptime, 我准备了一个实验，参考：[comptime 是如何工作的](/learning/zig/how_comptime_works.html)
2. [发现了 Zig 编译器的一个 BUG](@/blog/2024-12-04-a-zig-bug/index.md)，还比较严重的。
3. Scala Macro: 两篇关于 Scala 3 Macro 的文章，较为深入，值得收藏学习。
   - [Crafting types with Scala 3 macros - Part 1: Introduction to macros](https://inoio.de/blog/2024/07/14/scala3-macros-part1/)
   - [Crafting types with Scala 3 macros - Part 2: A Whitebox Macro](https://inoio.de/blog/2024/07/15/scala3-macros-part2/)

# MPP & OLAP

# Web & Visualization

# Tools & Libraries