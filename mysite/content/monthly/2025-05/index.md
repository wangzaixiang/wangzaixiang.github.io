+++
title = "May 2025"
description = "May 2025"
date = 2025-05-06
draft = false
template = "blog/page.html"
+++

# Languages
1. Architecture 
   - [Apple Silicon CPU Optimization Guide: 3.0](https://developer.apple.com/documentation/apple-silicon/cpu-optimization-guide)
     又发现了一本 CPU 级优化的好书，虽然这本书特别针对 Apple 系列CPU(包括 A系列和 M 系列)，但很多的内容，针对现代超标量处理器，都是具有参考意义的。
     
     结合这本书，可以更好的理解《超标量处理器设计》、《高性能超标量CPU微架构剖析于设计》等书的内容，我后续也会陆续整理这本书的学习笔记。结合 datafusion
     等项目，也可以尝试进行实践。
   - [software prefetch example 1](https://ibrahimessam.com/posts/prefetch/) 性能提升约 20%
   - [software prefetch example 2](https://lemire.me/blog/2018/04/30/is-software-prefetching-__builtin_prefetch-useful-for-performance/) 性能提升约 10%
     
     这个例子，我在 M1 上测试时，实际效果提升非常有限，大约在 1%。
2. Rust
    - [A Visual Journey Through Async Rust](https://github.com/alexpusch/rust-magic-patterns/blob/master/visual-journey-through-async-rust/Readme.md)
      通过可视化的方式来理解 async/await 的执行，包括 concurrent 与 parallel 的区别。
    - [Rust Stream API visualized and exposed](https://github.com/alexpusch/rust-magic-patterns/blob/master/rust-stream-visualized/Readme.md)
    - [使用 Stream 编写函数式的代码](https://willemvanhulle.tech/blog/func-async/)
      1. Stream ～ async iterator
    - [Two Years of Rust](https://borretti.me/article/two-years-of-rust) 很有意思的一个资深程序员（但对rust仅有2年使用经验）的思考。
      1. 【B站中文讲读】https://www.bilibili.com/video/BV1JGVXzAEXP?vd_source=eee72bea0d6227ec450743399f7c7b5b
      2. 将借用检查从一种负担转化为一种类化的检查：并将复杂的借用检查引导我们对设计进行简化，采取更加正交化的设计。
      3. linearly-types 概念：
         > 线性类型（Linearly Types）是编程语言类型系统中的一个概念，主要用于资源管理和内存安全。它要求变量在使用时必须严格遵循“恰好一次”的规则，
         > 即每个变量在作用域内必须被使用且仅被使用一次。这种机制常用于防止资源泄漏（如内存、文件句柄等），并支持对底层操作的高效控制。
      4. 操作系统的线程调度永远不够快，deepseek 可以对这个开销做很入的解释。
      5. 招聘很难吗？其实 rust 本身就帮助你进行了筛选。
      6. 滥用macro：过度使用 macro 可能会让代码的阅读和调试成为困难，这一点，我在阅读 datafusion 源代码时也有所发现，实际上，使用函数替代结合 inline
         机制来替代不必要的 macro，是我更喜欢的方式。
      
      作者对 borrow checking 有着百科全书级的理解，因为自己实现过类似的机制。这个我是深有体会的，我之前实现过一个 [easyajax](https://easyajax.sourceforge.net/),
      所以对后续的这类组件框架就有着自己独特的理解。
   
3. Scala
    - Scala 3.7.0 发布。这个版本最大的特性是 [NamedTuple](https://docs.scala-lang.org/scala3/reference/other-new-features/named-tuples.html) 成为正式特性。

# MPP & OLAP

# Web & Frontend
1. [How Rolldown Works: Module Loading, Dependency Graphs, and Optimization Explained](https://www.atriiy.dev/blog/rolldown-module-loader-and-dependency-graph)
2. [How We Built a 500% Faster Web App with WebAssembly in 2025](https://medium.com/@yewang222/how-we-built-a-500-faster-web-app-with-webassembly-in-2025-49f8f3f52995)
   
   应用类型：典型的BI展示、分析应用：实时数据处理，类似于电子表格的展示，图表展示，动态的用户界面。

   在前端进行了大量的数据加工处理类工作(非DOM操作)，以提高交互的体验：
   - 前端过滤
   - 前端排序
   - transpose（交叉表本地处理）
   - 本地的分组聚合
   - 电子表格的本地公式计算
   - JSON: 将远程的 blob 转换为 JSON 
   
   最佳搭配：JavaScript 处理 render, WASM 处理 compute
3. [Yoga](https://www.yogalayout.dev)  Meta 的急于 CSS 的 box layout engine, PixiJS 使用其来进行 scenario layout, 也可以考虑
   用于仪表盘之类的场景。
   - [Yoga-WASM](https://github.com/shuding/yoga-wasm-web) 以 WASM 方式使用
   - [Stretch](https://github.com/vislyhq/stretch) Rust 实现，支持 FlexBox layout, 目前不活跃
   - [druid: A data-first Rust-native UI design toolkit](https://github.com/linebender/druid) 
   - incremental layout.
   - containing block
4. [pixijs layout v3](https://pixijs.com/blog)
   

# Tools & Libraries