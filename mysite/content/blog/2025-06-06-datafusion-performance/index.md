+++
title = "datafusion 性能分析"
description = "datafusion 性能分析"
date = 2025-06-06
draft = false
template = "blog/page.html"

[extra]
toc = true
+++

本文记录我对 datafusion 的一些性能测试的数据、分析和思考。

从2023年起，我开始关注在公司的 BI 分析引擎中，引入 OLAP 引擎的底层计算能力，当时就开始评估了几个计算引擎，包括：
- [polars](https://pola.rs)
- [datafusion](https://datafusion.apache.org)
- [duckdb](https://duckdb.org)

关注的重点主要是 计算能力 和 计算性能，后面选择了 duckdb 作为我们的计算引擎，主要的优势：
- 性能：duckdb 在大部份测试场景都具有领先或相当的性能（可参考我当时的一个测试：[duckdb 测评](/inside-duckdb/preface.html)）
- 表达能力：duckdb 对窗口函数的支持程度基本上是最完备的（而我测试的其他包括 mysql, clickhouse, datafusion 都有不同程度的支持不充分），pola.rs
  的窗口计算逻辑完全不同于 SQL 定义的窗口函数（而且我无法理解起底层逻辑是什么）。当然，我们后续在使用 duckdb 时，也发现了 duckdb 提供了很多有价值的
  SQL 扩展，例如 group by grouping set 对分组小计等统计场景非常有帮助。
- 工具完善度：duckdb 的 工具，包括 CLI、python API 都设计得非常友好。

25年，在阅读了[2024 Practice: Apache Arrow DataFusion A Fast, Embeddable, Modular Analytic Query Engine](https://docs.google.com/presentation/d/1gqcxSNLGVwaqN0_yJtCbNm19-w5pqPuktII5_EDA6_k)
一文后，尤其是文中提到的性能对比后，我也在本地重跑了一次 datafusion-duckdb-benchmark 的对比，再花了些时间阅读了一下 datafusion 的源代码，
有了一些不同的认知：
- Rust 源代码的可读性相比 C++ 有了更好的提升（可能因为我喜欢 Rust 超过 C++），我也把阅读 hash-join, window function等算子整理了文章，在这个系列中进行了发表。
- datafusion 的性能虽然整体上仍然不如 duckdb，但这个差距主要是工程上的成熟度，而非架构上的差异。

# datafusion-duckdb-benchmark

1. clickbench
2. tpch
3. h2o

# datafusion 文章系列
1. [push vs pull](@/blog/2025-04-08-duck-push-vs-datafusion-pull/index.md)
2. [datafusion hashjoin executor](@/blog/2025-05-08-datafusion-hashjoin/index.md)
3. [datafusion window function executor](@/blog/2025-05-26-datafusion-window-function/index.md)
4. [datafusion performance](@/blog/2025-06-06-datafusion-performance/index.md)

