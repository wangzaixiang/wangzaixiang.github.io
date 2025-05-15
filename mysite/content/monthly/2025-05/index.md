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
3. Scala
    - Scala 3.7.0 发布。这个版本最大的特性是 [NamedTuple](https://docs.scala-lang.org/scala3/reference/other-new-features/named-tuples.html) 成为正式特性。

# MPP & OLAP

# Web & Frontend
1. [How Rolldown Works: Module Loading, Dependency Graphs, and Optimization Explained](https://www.atriiy.dev/blog/rolldown-module-loader-and-dependency-graph)
   

# Tools & Libraries