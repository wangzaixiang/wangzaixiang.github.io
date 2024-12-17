+++
title = "October 2024"
date = 2024-10-01
draft = false
template = "blog/page.html"

[extra]
toc = true
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

4. [oxc: JavaScript Compiler 项目的性能优化记录](https://oxc.rs/docs/learn/performance.html)
   oxc 项目是使用 Rust 编写的 JavaScript 编译器，最近 Vite 项目也在基于 oxc 开发新一代的 bundle: rolldown. OXC 以性能著称，这篇文章
   介绍了作者在优化过程中采取的一系列措施，包括：
   - AST 相关
     - AST 内存分配。从每个小节点的分配、释放，Drop的成本较高，调整为为单个 AST 分配一个 arena，作为整体释放。提升 ～20%（也大为提升缓存友好性）
       这个与 zig 的内存管理风格有些相似。
     - Enum Size。使用 box 包装字段，减少 enum 占用大小。提升 ～10%。
     - Span：将位置信息从 usize 调整到 u32. 提升 ～5%。
     - String interning。 string-cache 存在并发问题，移除后，性能提升 ～30%。
     - 对 string-cache 进行并发优化
     - String inlining: 对 长度 <= 23 的字符串进行 inline, 避免使用 String。
   - Lexer
     - SIMD 
     - keyword match
   - Linter

5. [gitoxide](https://github.com/Byron/gitoxide/tree/main) 用 rust 重新的 git 轮子。 git 生态已经是一个比较大的命令行生态了，这个
   轮子的工作量并不小。
6. [x86 Intrinsics Cheat Sheet](https://db.in.tum.de/~finis/x86%20intrinsics%20cheat%20sheet%20v1.0.pdf) 这个是学习 SIMD 的好资料。
7. [x86 Intrinsics Reference](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#) Intel 官方参考。
8. 排序
   - [排序算法 SIMD版本](https://www.vldb.org/pvldb/vol8/p1274-inoue.pdf)
   - [x86-simd-sort](https://github.com/intel/x86-simd-sort)


# MPP & OLAP
1. [Why Rust is taking the data engineering world by storm](https://kerkour.com/rust-data-engineering)
   - 100G - 9T 规模的数据规模覆盖了 98% 的需求，之前需要一个大型的 MPP 集群，而现在，可以在1个服务器上完成。
   - Rust + Cargo 改变了数据库应用的开发方式，不再需要一个 monlitic 的数据库，而是可以通过组合各种 crate 来完成。
   - [很多数据库](https://datafusion.apache.org/user-guide/introduction.html#known-users)都采用 datafusion 作为计算引擎。
2. [Big Data is dead](https://motherduck.com/blog/big-data-is-dead/)
3. [Apache Doris 3.0 里程碑版本｜存算分离架构升级、湖仓一体再进化](https://www.oschina.net/news/316422/apache-doris-3-0-released)

# Web & Visualization
1. Bundlers
   - [bun.build](https://bun.sh/docs/bundler) 比 esbuild 更快。
   - [oxc-project](https://oxc-project.github.io)
   - [rolldown](https://rolldown.rs) based on oxc, vite 子项目，意在替换 esbuild 和 rollup，作为 dev/release 模式下统一的打包工具
2. [brisa](https://brisa.build) 一个号称继承了组多前端框架特性的新轮子，对 webcomponent 和 SSR 都有支持，等发布了再看看。
3. [Lit 支持 signals](https://lit.dev/blog/2024-10-08-signals/) 
   现在的组件有两种响应模式了：
   - 对 组件 自己的 props/states 的变化的响应。
   - 对 外部的 signal 的响应。
   增加后者，带来了灵活性的同时，是否会带来更多的复杂性呢？
4. 口水战： Web Components, Future or Not?
    - [Web Components Are Not the Future](https://dev.to/ryansolid/web-components-are-not-the-future-48bh)
      很遗憾，我都没有 get 到作者的点。
    - [Web Components Are the Future](https://medium.com/@treeder/web-components-are-the-future-f0f9f0022686)
    - [Web Components Are Not the Future — They’re the Present](https://www.abeautifulsite.net/posts/web-components-are-not-the-future-they-re-the-present/)
      - 组件的互操作性。 ABI 兼容性。在框架A中编写一个组件，在框架B、C、D中使用。
      - 我们不需要就如何编写组件达成一致，我们只需要在底层实现达成一致，这样就可以实现组件的互操作性。
      - Element? Component? 谁是谁的子集？组件不一定是 Element，不是 Element 的组件，不一定要作为 WebComponent, JS Module 足以。
   - 2024-11 [WebComponents Are (a Part of) the Future](https://c5r.medium.com/webcomponents-are-a-part-of-the-future-94a2b0940314)
   - [Why I don't use web components](https://dev.to/richharris/why-i-don-t-use-web-components-2cia)
      - 渐进式组件对 JS 的依赖？ tiny case
      - css in js. little case.
      - 标准、稳定性。 little case
      - polyfills.
      - slot composition. WEB 组件对 lighting DOM 缺少延迟处理。
      - props or attrs. 
      - The DOM is bad, write less code. 又一个双向绑定的争论。 svelte 有对双向绑定的支持。
      这篇文章有比较具体的技术点，不过，从我的角度上看，是一些偏好行的选择。
5. wasm
    - [Life of a Zed Extension: Rust, WIT, Wasm](https://zed.dev/blog/zed-decoded-extensions)
      1. how an extension is built
      2. using components model(IDL)
      3. the wasm runtime: wasmtime
6. [Svelte 5 发布](https://svelte.dev/blog/svelte-5-is-alive)

   Svelte 是编译期优化的响应式框架，这个版本的特色是引入了 rune 的语法特征。
   1. let count = $state(0);   // 申明 count 为一个响应式变量，这里仅仅是一个语法标识，而非函数调用。
      > 从源代码的角度来看 count 仍然是未封装的 number，但编译期为其创建了一个分装的响应式变量，只是源代码中获取的是 wrapper 的值
      > 当然，对 value 的赋值操作也被改写成为对 wrapper 的操作。
      > 通过 $state 这个 rune 标识的变量读写，都被编译器进行了改写，而非通过运行期反射方式。
      >
      > Svelte 5 为什么要做这一个从隐式（Svelte 4）到显式的调整，这样做会有什么优点？这个在官网有一个 [说明](https://svelte.dev/blog/runes):
      > - 显示说明，似乎是一种回退？
      > - 对复杂项目，可以更好的理解哪个变量是响应式的（在变量的定义时明确），对 .svelte/.js 代码保持一致。
   2. Svelte 与 solidjs 是类似的原理。后续可以做一个特性上的对比。
   


# Tools & Libraries