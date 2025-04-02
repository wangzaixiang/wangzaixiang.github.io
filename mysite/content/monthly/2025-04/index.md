+++
title = "April 2025"
description = "April 2025"
date = 2025-04-02
draft = false
template = "blog/page.html"
+++

# Languages

# MPP & OLAP
1. Datafusion
   - [Part 1: Query Engine Architecture](https://docs.google.com/presentation/d/1D3GDVas-8y0sA4c8EOgdCvEjVND4s2E7I6zfs67Y4j8/edit#slide=id.p)
   - [Part 2: Logical Plan and  Expressions](https://docs.google.com/presentation/d/1ypylM3-w60kVDW7Q6S99AHzvlBgciTdjsAfqNP85K30)
   - [Part 3: Physical Plan and Execution](https://docs.google.com/presentation/d/1cA2WQJ2qg6tx6y4Wf8FH2WVSm9JQ5UgmBWATHdik0hg)
   思考：
   - 对比 Datafusion 与 DuckDB/Polars 的执行计划
     1. 从 Part 3, page 11 来看，datafusion 在 operator 间基于 async/await 异步执行。而 duckdb 在 pipeline 之内的 operator 是同步执行的。
        这个可能是 duckdb 的性能优势。
   - 引入一个新的 IR 思考：[another query plan IR- draft](@/blog/2025-04-02-qir-design/index.md)

# Web & Frontend

# Tools & Libraries
