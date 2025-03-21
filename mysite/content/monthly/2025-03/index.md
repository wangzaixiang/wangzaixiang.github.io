+++
title = "March 2025"
description = "March 2025"
date = 2025-03-03
draft = false
template = "blog/page.html"
+++

# Languages
1. [Rust 2024 年度报告 之一： Rust 2024 Edition 变更特性详解](https://mp.weixin.qq.com/s/sCRHoM-JITyp0L4kgWH1Jw)
   - 哲学
       - Safety & Performance
       - 生态系统 与 社区协作的开放治理
       - Zero Cost Abstractions & 现代语言特性的兼容
       - 系统编程 与 应用落地
   - 2024 Edition (需要花些时间逐一理解一下)
     - if-let chains/ let chains
     - 表达式临时值的scope
     - match 绑定变量
     - ! 类型的回退类型
     - gen 生成器
     - async trait fn
     - async closure
     - use<'a>
2. zig 0.14 期待发布时在此补充
3. [Const Fn in Rust](https://felixwrt.dev/posts/const-fn/)
   这篇文章并没有介绍 const fn 是如何在 compile time 时evaluate 的，只是介绍了 const fn 的基本用法。
   - [compile time evaluation](https://doc.rust-lang.org/reference/const_eval.html)
4. 学习 register allocation
   - [A Generalized Algorithm for Graph-Coloring Register Allocation](https://c9x.me/compile/bib/pcc-rega.pdf)
   - [CS143 编译原理笔记 6 - Register Allocation](https://zhuanlan.zhihu.com/p/640647465) 中文笔记
     - Liveness Analysis
     - Register interference graph
     - Graph coloring
   - [Linear Scan Register Allocation](https://c9x.me/compile/bib/linearscan.pdf)

# MPP & OLAP
1. [DuckDB -- ART索引](https://zhuanlan.zhihu.com/p/645064049)
   trie 树的变种，面向内存（索引数据全部加载到内存）
   1. Node4 ( `key[4], child[4]` ):  4 + 4 * 8 = 36 bytes
   2. Node16 ( `key[16], child[16]` ) 16 + 16 * 8 = 144 bytes
   3. Node48 ( `key[256], child[48]` ) 256 + 48 * 8 = 400 bytes
   4. Node256 ( `child[256]` ) 256 * 8 = 2048 bytes
   5. Leaf
   
   优化：Node: prefix + key + index, Leaf: prefix + value。
2. [Duck DB -- ART](https://duckdb.org/2022/07/27/art-storage.html)
   DuckDB 早期 ART 索引是不存储的，启动时会重建。目前的版本是持久化的。
   使用 ART 来存储 PK（每一行都需要在 Leaf 中）:
   1. 是否会占用大量内存？lazy load node 使得无需将全部索引加载到内存中。
   2. 如何进行存储？方便 lazy load
   ![img.png](art-post-order.png)
   ![img.png](art-storage.png)
   使用 post-order 可以使得一个一个 Block (256K) 作为一个整体进行读取（存储时可选压缩）。当 block 未加载时，只需整体加载该 block 到内存中。
   ![img.png](swizzlable-pointer.png)
   
   问题：
   1. post order 维持了 node 的整体有序性，一个 block 中的节点是基本连续、有序的。
   2. 插入、修改操作涉及到对 block 的拆分、合并。
   3. 索引的存储放大比率如何？比如说对 1G 的数据行， ART 索引需要多少空间？

# Web & Visualization

# Tools & Libraries