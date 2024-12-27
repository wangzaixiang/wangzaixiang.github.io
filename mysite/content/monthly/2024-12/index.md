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
## Zig: 本月重点花一些时间学习理解 zig。
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
3. [What is Zig got than C. Rust, and Go don't have](https://www.youtube.com/watch?v=5_oqWE9otaE&t=3910s)
## others
1. Scala Macro: 两篇关于 Scala 3 Macro 的文章，较为深入，值得收藏学习。
   - [Crafting types with Scala 3 macros - Part 1: Introduction to macros](https://inoio.de/blog/2024/07/14/scala3-macros-part1/)
   - [Crafting types with Scala 3 macros - Part 2: A Whitebox Macro](https://inoio.de/blog/2024/07/15/scala3-macros-part2/)
2. Scala 3.6.2 Released
   - Clause Interleave. 支持: `def getOrElse(k: Key)[V >: k.Value](default: V): V`, 更强的 path-dependent type.
   - Improve Syntax for Context Bounds and Givens
   - NamedTuple 很期待的一个语法糖，可以让使用 Tuple 的代码更加可读。 对 参数较多的 case class 进行 pattern match 也可以享受 NamedTuple 的便利了
3. [How We Made the Deno Language Server Ten Times Faster, +8s -> <1s](https://denoland.medium.com/how-we-made-the-deno-language-server-ten-times-faster-62358af87d11)
   - 封装（样式、行为通过 Shadow DOM）、互操作性(框架无关)、标准化
4. [Modeling in scala, part 1: modeling you domain](https://kubuszok.com/2024/modeling-in-scala-part-1/)

# MPP & OLAP
1. [Hot Module Replacement](https://bjornlu.com/blog/hot-module-replacement-is-easy#importmetahotaccept)

# Web & Visualization
1. [WC 在现代UI中的无名贡献](https://blog.devgenius.io/widget-wonders-web-components-the-unsung-heroes-of-modern-ui-052131d692be)
2. [Scoped Components](https://medium.com/dev-jam/functional-web-components-with-lit-part-2-3521a82bf339)
   - 可以为某个组件的 html`` 中使用的 webcomponent 提供一个 scope，重新定义 tag -> element 的映射关系。
   - 为组件提供示例级的 css
3. [Setting up bun in webstorm](https://medium.com/@muthuishere/setting-up-bun-as-your-javascript-runtime-in-webstorm-and-other-jetbrains-ides-ae98f9368557)
4. No Build 后续
   - [Why do we still need bundlers](https://rolldown.rs/guide/in-depth/why-bundlers)
     1. HTTP/2 并不意味可以停止关心 HTTP 请求的数量（大部份浏览器的限制是100个，每个请求的额外开销），对包含数千个模块的应用仍然需要 bundling。
     2. 深层次的 import 会导致 waterfalls.
     3. Cache:
     4. 减少网络字节数
     5. 编译优化
# Tools & Libraries