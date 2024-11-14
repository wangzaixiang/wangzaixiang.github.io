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

# Tools & Libraries