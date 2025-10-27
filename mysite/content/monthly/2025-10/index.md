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
   - [We Deleted Tokio From Our Payment System and Cut Cloud Costs by $127,000](https://medium.com/@the_atomic_architect/we-deleted-tokio-from-our-payment-system-and-cut-cloud-costs-by-127-000-b745a86f973b)
     Medium 上的标题党也是越来越多了，这篇文章尝试得出了一个不算太错误的结论，但却做出了完全错误的归因。
     - 这个业务场景技术要求其实很低，QPS < 1000，根本用不上什么高大上的技术，使用 Thread 完全是满足要求的。
     - 但将成本原因归结于 async 却是头痛开错了药方，虽然杀鸡用了牛刀，但杀不死鸡并不是牛刀的问题，而是作者在 async 框架中使用了 sync 的 postgres 库，而又只配置了较小的线程，
       导致大部份线程实际上处在 Blocking 状态，导致 RTT 大幅提升。
     这种错误的归因，其危害不亚于选择笨重的框架。
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
