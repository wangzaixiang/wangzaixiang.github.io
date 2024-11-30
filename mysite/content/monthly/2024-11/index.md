+++
title = "November 2024"
date = 2024-11-01
draft = false
template = "blog/page.html"

[extra]
toc = true
+++

# Languages
1. [Improve an algorithm performance step by step](https://blog.mapotofu.org/blogs/rabitq-bench/)
   - 使用 samply 工具收集性能数据
   - 使用 criterion 库进行性能测试
   - metrics 采集
   - 编译选项：例如 `RUSTFLAGS="-C target-feature=+popcnt"`
   - SIMD 加速，如果算法可以调整为 SIMD 优化的算法，加速效果会很明显。
   - 选择更好的库： faer 
   - IO 优化
   - const generics
2. [Using portable SIMD in stable Rust](https://pythonspeed.com/articles/simd-stable-rust/)
   > 延续：[October 2024](@/monthly/2024-10/index.md#Languages) 中 faster Mandelbrot with SIMD  一文

   本文介绍了：
   - 使用 wide 这个库，在 stable rust 中使用 SIMD 的方法。 （可以了解一下 wide 是如何通过 safe_arch 这个库来处理多平台架构的
     支持的）。
   
     按照本文的测试，wide的性能相比 portable-simd 要慢一些，但比 scalar 的版本还是有成倍的提升。
   - 使用 pulp 库，这个库是一个 high level SIMD abstraction. 也是 stable rust 的。
3. [Runtime Scripting for Rust Applications](https://www.youtube.com/watch?v=M8dpH3rO-2M) [PDF](https://dl.korz.dev/eurorust2024.pdf)
    - Python.  PyO3(Rust bindings for Python) / RustPython(a interpreter written in Rust)
    - Lua:  lightweight.(355k lua vs 25M cpython vs 37m V8) [mlua bindings to rust](https://github.com/mlua-rs/mlua)
    - JavaScript
      - V8: [rusty_v8r](https://crates.io/crates/v8) , deno_core, deno_ast, deno_runtime
      - JavaScriptCore: [rusty_jsc](https://github.com/wasmerio/rusty_jsc) [jsc.rs](https://github.com/endoli/javascriptcore.rs)
      - SpiderMonkey: [mozjs](https://github.com/servo/mozjs)
    - WASM
    - rhai: AST-interpreter, dynamic typed
    - mun:  AOT, static typed, hot reloading. LLVM based
4. a deep dive into the bun architecture
    - [part 1](https://makwritinghouse.com/bun/a-deep-dive-into-the-bun-architecture-part-1/)
      - core layer(using C/Zig): managing JS runtime, the native HTTP server, and the file system.
      - API layer(using js).
   目前没有后续的文章。

# MPP & OLAP

# Web & Visualization
1. bun 1.1.34 支持 Wasm GC，dart/kotlin/scalajs 等可以编译为 wasm 的语言都可以使用 bun 了。
2. [brisa](https://brisa.build)
   简单的浏览了一下这个项目，有一下特色：
   - 核心: 两个组件模型：Server Component, Client Component。
     - Server Component: 运行在 Server 端的 JS 组件，概念和 JSF(Java Server Faces)、[Tapestry](https://tapestry.apache.org) 很相似。
       若干年前，tapestry 曾是我的最爱。不过，brisa 是基于 JavaScript 的服务端组件，使用 JSX 的语法，似乎比 tapestry 更简单。当然，使用 JSX 
       来描述服务端的组件，在服务端处理事件，从server端更新状态到 client 端，整个的编程模型还是有些别扭的。
     
       JSF、Tapestry等服务端 UI 组件模型，随着 前端组件技术的兴起，已经逐步退出历史舞台。brisa 把这个概念重新拾起，我个人感觉意义不大。
     - Client Component: 基于 Web Component 的前端组件，brisa 的前端组件，整体与 SolidJS、Svelte 等相似。一个比较好的点是，brisa 支持 
       Web Component 的 SSR， 也就是在一个 Server Component 中，可以嵌入 Client Component，这个 Client Component 会先SSR. 相当于初始化
       过程可以在服务端完成。
       - 一个挑战：SSR 和 CSR 混合使用时，CSR 是如何将动态数据与 Declarative Shadow DOM 结合的？brisa 在这方面似乎比 Lit SSR 做的更优雅一些。
   - 约定而非配置。这使得应用变得非常简单。
3. Web Components UI Library
   - [Shoelace](https://shoelace.style)
   - [SAP UI5](https://sap.github.io/ui5-webcomponents/blog/releases/announcing-v2/)
   - [Cisco Momentum](https://github.com/momentum-design/momentum-ui/tree/master/web-components#:~:text=lit-element)
   - [Adobe](https://opensource.adobe.com/spectrum-web-components/components/underlay)
   - [Stencil Components](https://crayons.freshworks.com) Build with Stencil.
   - [Vaadin](https://vaadin.com/docs/latest/components)
   - [Sale Force](https://developer.salesforce.com/docs/component-library/overview/components)
   
   以下库目前处在未维护、更新状态
   - [Elix](https://github.com/elix/elix)
   - [Hope UI](https://hope-ui.netlify.app) 被 pigment 替代，但目前尚未发布 
   - [MWC](https://github.com/material-components/material-components-web) mwc is in maintenance mode.

# Tools & Libraries