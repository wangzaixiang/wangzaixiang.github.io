+++
title = "Oct 2025"
description = "2025-10 月记"
date = 2025-10-13
draft = false
template = "blog/page.html"
+++

# Languages
1. Rust
   - [Generic Asoociated Types](https://medium.com/@syntaxSavage/generic-associated-types-gats-the-rust-feature-that-finally-solves-async-trait-hell-b9b79d14a422)
     如何理解 async traits 及更为基础的 GAT？
   - [Polonius: Rust's new borrow checking](https://medium.com/@syntaxSavage/rusts-new-borrow-checker-polonius-is-coming-a890fc5ffd8b)
     - Pros: more precise, fewer false errors, ability to accept more safe code, groundwork for self-borrows.
     - Cons: more complex analysis, potentially more memory and CPU overhead, risk of regressions for big codebases
   - [Polonius Book](https://rust-lang.github.io/polonius/)
2. Zig
   - Zig 0.15.2 Released 据说又是一个破坏性的版本。 

# Mpp & OLAP
1. DataFusion 有一段时间没有跟进 DataFusion 的新特性了。
   - 48.0.0
     - performance: fewer unnecessary projections
     - performance: Accelerated string functions(ascii, character_length)
     - performance: Constant aggregate window expressions
     - grammar: order by all
   - 49.0.0
     - performance: dynamic filters and topK pushdown
     - async UDF
   - 50.0.0
     - performance: dynamic filter pushdown improvements(for hash joins)
     - performance: nested loop join optimization
     - parquet metadata caching
     - grammar: support qualify stmt
     - grammar: filter support for window functions
   

# Web & Frontend

# AI & Agent
- [graphflow: multi agent flow](https://ai.gopubby.com/graphflow-rust-native-orchestration-for-multi-agent-workflows-6143a9b767ad)
  正好最近我们也设计了一个基于 flow 的多 agent 框架，再看看这篇文章，感觉这个设计还是太粗糙了些。

# Misc
