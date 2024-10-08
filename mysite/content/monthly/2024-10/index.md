+++
title = "October 2024"
date = 2024-10-01
draft = false
template = "blog/page.html"
+++

# Languages
1. [Rewriting Rust](https://josephg.com/blog/rewriting-rust/) 
   作者提出了自己心中理想的 rust 应该包括的几个特性：
   - 更多的 function trait. 初了 Fn, FnMut, FnOnce。这样相当于给 函数 提供更多的类型信息。
   - Move, Pin 的重新设计，消除 Pin 存在的必要性，统一 borrow 语义
   - comptime. 不同与 macro, comptime 使用相同的源语言语法，有更好的类型检查。（编译器内置一个小的解释器 ）
   这3个特性对我来说，都感觉是非常合理的，且有吸引力。其中，Pin这一块的介绍也可以帮助我们理解 Pin 存在的原因。

2. [faster Mandelbrot with SIMD](https://pythonspeed.com/articles/optimizing-with-simd/)
   Mandlebrot 这个算法如何利用 SIMD 进行优化(有循环和分支处理)。
   
   不过，这里作者的写法似乎有一些 BUG， SIMD 版本并不严格对应于 scalar 版本，有些地方的处理不一致。参考我之前的文章 
   [Mandelbrot-set CPU vs GPU comparison](@/blog/2024-08-17-mandelbrot-gpu.md)
3. 现代微处理器：90分钟速成指南！介绍了现代 CPU 的一些概念，如超标量、乱序执行、分支预测、缓存等、SIMD、SMT 等技术。
   - [中文版](https://zhuanlan.zhihu.com/p/645343994)
   - [English Version](https://www.lighterra.com/papers/modernmicroprocessors/)

# MPP & OLAP

# Web & Visualization
1. Bundlers
   - [bun.build](https://bun.sh/docs/bundler) 比 esbuild 更快。
   - [oxc-project](https://oxc-project.github.io)
   - [rolldown](https://rolldown.rs) based on oxc, vite 子项目，意在替换 esbuild 和 rollup，作为 dev/release 模式下统一的打包工具
2. [brisa](https://brisa.build) 一个号称继承了组多前端框架特性的新轮子，对 webcomponent 和 SSR 都有支持，等发布了再看看。

# Tools & Libraries