+++
title = "Jul 2025"
description = "Jul 2025"
date = 2025-07-07
draft = false
template = "blog/page.html"
+++

# Languages
1. rust
   - [https://sharnoff.io/blog/why-rust-compiler-slow](Why is the Rust compiler so slow?)
     - ![cargo build --timings](img.png)
     - ![opt-level](img_1.png)
     - debug , LTO, optimization
     - `RUSTFLAGS="-Cllvm-args=-inline-threshold=50 -Cllvm-args=-inlinedefault-threshold=50 -Cllvm-args=-inlinehint-threshold=50"`
     - Reducing inlining with LLVM args;
     - Breaking up expensive functions in the main crate; and
     - Removing generics from dependencies to prevent needing to compile it in the main crate
     Timeline:
     -  We started at ~175s
     -  Disabling LTO (and debug symbols!) got us to 51s (-71%)
     -  Changing to opt-level = 1 on the final crate got us to 48.8s (-4%)
     -  Reducing inlining with -C llvm-args got us to 40.7s (-16%)
     -  Local changes got us to 37.7s (-7%)
     -  Changes with dependencies got us to 32.3s (-14%)
     -  Enabling -Zshare-generics got us to 29.1s (-10%)
     -  And switching away from alpine got us to 9.1s (-69%)
   - [Leaktracer: A Rust allocator to trace memory allocations](https://blog.veeso.dev/blog/en/leaktracer-a-rust-allocator-to-trace-memory-allocations/)
2. zig
   - [zig's new async io](https://kristoff.it/blog/zig-new-async-io/)
   - [What is Zig's “Colorblind” Async/Await?](https://kristoff.it/blog/zig-colorblind-async-await/)
3. Java
   - [小红书JDK升级带来10%整体性能提升，这份升级指南收好了！](https://mp.weixin.qq.com/s/qxDNW5Ss3r4zhPSCqdXvqw) 千年的JDK8问题
     - [G1: Parallel Full GC for G1](https://openjdk.org/jeps/307)  
     - G1: Initiating Heap Occupancy Percent
     - G1: 提前回收大对象
     总的感觉来说，对大型系统，Java的GC仍然带来了一定程度的不可控因素，所有的 GC 调优其实都是在这个不确定性上跳舞，获得或多或少的改进，
     但仍然是存在不确定性。这就是在一个低维度的挣扎。（不过，作为应用层的编程语言，不采纳 GC 的成本可能更高 ）
     - Java 的优点和缺点可能是现在的生态，在不需要那么臃肿的场景下，可能会越来越臃肿。
   - [从 Java 迁移到 Swift: 密码监控服务](https://www.swift.org/blog/swift-at-apple-migrating-the-password-monitoring-service-from-java/)
     

# MPP & OLAP

# Web & Frontend
1. [Dashboard That Works: A Step-by-Step Guide for Startups in 2025](https://uxplanet.org/dashboard-that-works-a-step-by-step-guide-for-startups-in-2025-1cec1bfe7f9c)
   
   A truly functional dashboard: 
   - GOALS, DATA, PEOPLE, LOGIC
   - analysts, designers, developers
   - work for business, not just look pretty
   - Don't try to make a dashboard for everyone
   - define use cases and users
   - identify key metrics
   - prepare data
   - sketch a rough layout
   - get user feedback
   - write a solid brief for designers
   - find the right builders
   - Launch, watch, improve
2. [fastplotlib](a plotting library built on WGPU)
   - based on [pygfx](a python-graphics library visualization framework on WGPU)
   思考：结合 Vega, Pixi, D3 等框架，思考一种描述 visualization 的最佳方式。
   - Vega：使用 JSON 的方式，缺乏类型化约束，不便于理解、使用，部份的能力过于糖化。
   - 使用多维空间的方式来理解 visualization
   - 将数据（关系数据、多维数据）映射到（可视化）多维空间。
   - 使用 react 的方式来理解 interact
   - 更好的表示语言？
     - JSON based
     - more datatype: date, interval, float, decimal, more via regexp"..." style
     - JSON constructor:   Color { r g b }
     - functional constructor: Color(r, g, b), with fixed parameter and named parameter
     - enum literal. .RED instead of Color.RED
     - reference spec: global reference, document reference, scoped reference.
3. [AntV G2](https://g2.antv.antgroup.com/manual/quick-start)
   粗览一下文档和示例，G2 的概念与 vega 时非常相似的，提供2种 API: 命令式 和 DSL 式。

# LLM & Agents
1. [LLM Powered Autonomous Agents](https://lilianweng.github.io/posts/2023-06-23-agent/)
   ![img_2.png](img_2.png)
2. AutoGPT
3. AnyTool
4. HuggingGPT
5. Understanding the planning of LLM agents: A survey
6. https://github.com/AntonOsika/gpt-engineer
7. JoyAgent
8. Gaia benchmark: a benchmark for General AI Assistants