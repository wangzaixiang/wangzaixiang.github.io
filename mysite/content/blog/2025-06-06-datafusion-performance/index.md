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

本文主要针对 datafusion-duckdb-benchmark 这个项目的测试结果，对二者的差异进行分析，并尝试给出改进、优化 datafusion 的一些建议。
更新的 [datafusion-duckdb-benchmark](https://github.com/wangzaixiang/datafusion-duckdb-benchmark) 项目:
1. 升级到最新的版本 datafusion 47.0.0 和 duckdb 1.3.0
2. 配合本文对部份脚本进行了更新。

# datafusion-duckdb-benchmark

1. clickbench
2. tpch
3. h2o

## clickbench 单线程测试
![comparison.clickbench.png](comparison.clickbench.png)

```sql
-- duckdb sql
 with 
   cb_datafusion as (from  read_csv('results/2025-06-17-df-48.0.0-duckdb-1.3.0-m1max/clickbench_datafusion.csv', header=false, columns= { 'query': 'int', 'core': 'int', 'seq': 'int', 'tm': 'double' } )),
   cb_duckdb as (from  read_csv('results/2025-06-17-df-48.0.0-duckdb-1.3.0-m1max/clickbench_duckdb.csv', header=false, columns= { 'query': 'int', 'core': 'int', 'seq': 'int', 'tm': 'double' } )),
   duckdb as (select query, min(tm) as tm from cb_duckdb group by query order by query), 
   datafusion as (select query, min(tm) as tm from cb_datafusion group by query order by query)
  select duckdb.query, round(duckdb.tm,3) as "duckdb time", round(datafusion.tm,3) as "datafusion time", round((datafusion.tm - duckdb.tm) / (duckdb.tm) * 100, 2) as "diff%" 
  from duckdb left join datafusion on duckdb.query = datafusion.query
  order by 1;
```

| query | duckdb time | datafusion time |   diff% |
|------:|------------:|----------------:|--------:|
|     1 |        0.03 |           0.033 |   12.23 |
|     2 |       0.109 |           0.071 |  -35.08 |
|     3 |       0.285 |           0.321 |   12.51 |
|     4 |       0.394 |           0.287 |  -27.19 |
|     5 |       2.411 |           2.219 |   -7.96 |
|     6 |       2.931 |           3.111 |    6.15 |
|     7 |       1.633 |           0.063 |  -96.15 |
|     8 |       0.117 |           0.075 |  -36.21 |
|     9 |        2.95 |           2.629 |  -10.87 |
|    10 |        4.04 |           3.929 |   -2.75 |
|    11 |       0.651 |           1.008 |    55.0 |
|    12 |       0.757 |           1.133 |   49.66 |
|    13 |       2.781 |           2.917 |    4.91 |
|    14 |       4.062 |           3.907 |   -3.81 |
|    15 |       2.968 |           2.763 |   -6.92 |
|    16 |       2.673 |           2.596 |   -2.86 |
|    17 |       5.652 |           5.042 |  -10.79 |
|    18 |        5.57 |           5.024 |    -9.8 |
|    19 |      10.476 |           8.318 |   -20.6 |
|    20 |       0.095 |           0.241 |  151.94 |
|    21 |       7.064 |           6.455 |   -8.62 |
|    22 |       5.578 |           7.659 |   37.31 |
|    23 |       10.85 |          14.823 |   36.62 |
|    24 |       7.969 |            39.1 |  390.67 |
|    25 |       2.454 |           2.314 |   -5.69 |
|    26 |       1.708 |           2.048 |   19.93 |
|    27 |       2.251 |           2.613 |   16.09 |
|    28 |       5.094 |          10.745 |  110.92 |
|    29 |      29.147 |          28.087 |   -3.64 |
|    30 |        0.24 |           2.405 |   904.0 |
|    31 |       3.086 |           2.896 |   -6.15 |
|    32 |       3.544 |           2.911 |  -17.86 |
|    33 |      10.346 |           8.892 |  -14.05 |
|    34 |       9.928 |          11.879 |   19.65 |
|    35 |      10.395 |          11.869 |   14.19 |
|    36 |       2.916 |           3.819 |   30.97 |
|    37 |       0.125 |           0.152 |   22.28 |
|    38 |       0.101 |           0.093 |   -7.43 |
|    39 |       0.066 |           0.112 |   69.53 |
|    40 |       0.222 |           0.292 |   31.95 |
|    41 |       0.052 |           0.033 |  -36.48 |
|    42 |       0.047 |           0.031 |  -33.35 |
|    43 |       0.062 |           0.039 |  -36.65 |



在43个场景中，有23个场景 datafusion 更快，有20个场景，datafusion 更慢, 其中差异比较显著的是：


| query | duckdb time | datafusion time |  diff% | analyze                  |
|------:|------------:|----------------:|-------:|--------------------------|
|     1 |        0.03 |           0.033 |  12.23 |                          |
|     3 |       0.285 |           0.321 |  12.51 |                          |
|     6 |       2.931 |           3.111 |   6.15 |                          |
|    11 |       0.651 |           1.008 |   55.0 |                          |
|    12 |       0.757 |           1.133 |  49.66 |                          |
|    13 |       2.781 |           2.917 |   4.91 |                          |
|    20 |       0.095 |           0.241 | 151.94 | TODO                     |
|    22 |       5.578 |           7.659 |  37.31 |                          |
|    23 |       10.85 |          14.823 |  36.62 | TODO                     |
|    24 |       7.969 |            39.1 | 390.67 | duckdb 生成了更好的 TOP-N 执行计划 |
|    26 |       1.708 |           2.048 |  19.93 |                          |
|    27 |       2.251 |           2.613 |  16.09 |                          |
|    28 |       5.094 |          10.745 | 110.92 | TODO                     |
|    30 |        0.24 |           2.405 |  904.0 | duckdb 有更好的公共表达式优化       |
|    34 |       9.928 |          11.879 |  19.65 |                          |
|    35 |      10.395 |          11.869 |  14.19 |                          |
|    36 |       2.916 |           3.819 |  30.97 |                          |
|    37 |       0.125 |           0.152 |  22.28 |                          |
|    39 |       0.066 |           0.112 |  69.53 |                          |
|    40 |       0.222 |           0.292 |  31.95 |                          |

1. query 23
   - duckdb: TABLE_SCAN 11.49s, output: 7128 rows (98%)
   - datafusion:
2. query 24: `SELECT * FROM hits WHERE URL LIKE '%google%' ORDER BY EventTime LIMIT 10`
   duckdb 生成了更好的查询计划：
   ```sql
    select a.* 
    from hits a 
    join
        (select file_index, file_row_number from hits where URL LIKE '%google%' ORDER BY EventTime LIMIT 10) b 
    on a.file_index = b.file_index and a.file_row_number = b.file_row_number
   ```
   这个优化，极大的减少了 parquet 的扫描开销。
3. query 30
   duckdb 对公共表达式优化进行了更好的优化，将 `SUM(col + n)` 替换为 `SUM(col) + n * COUNT(col)`, 这样对 Query 30 中多达 90 个 `SUM(col + n)`
   的字段，最后优化到只有2个分组计算： `SUM(col)` 和 `COUNT(col)`。
   优化： 参考 duckdb 的表达式优化策略。

# tpch benchmark
## tpch sf=10
![comparison.tpch.png](comparison.tpch.png)
这个测试在 M1-MAX 64G 上运行。

| query | duckdb time | datafusion time |  diff% |
|------:|------------:|----------------:|-------:|
|     1 |       2.624 |           4.939 |  88.23 |
|     2 |        0.37 |            0.84 | 126.78 |
|     3 |       1.584 |           1.634 |   3.17 |
|     4 |       1.117 |           0.897 | -19.63 |
|     5 |       1.753 |           2.412 |  37.64 |
|     6 |       0.742 |           0.822 |  10.74 |
|     7 |       1.659 |           4.817 | 190.37 |
|     8 |       2.413 |           2.835 |  17.47 |
|     9 |       3.651 |           4.605 |  26.11 |
|    10 |       2.503 |           2.587 |   3.35 |
|    11 |       0.347 |           0.949 |  173.3 |
|    12 |       0.918 |           1.556 |  69.45 |
|    13 |       2.859 |           2.503 | -12.44 |
|    14 |        1.03 |           1.194 |  15.95 |
|    15 |       0.907 |           2.331 | 156.89 |
|    16 |       0.337 |           0.524 |  55.21 |
|    17 |       1.679 |           5.537 | 229.75 |
|    18 |       2.779 |           8.695 | 212.84 |
|    19 |         1.5 |           2.584 |  72.31 |
|    20 |       1.347 |           2.007 |   49.0 |
|    21 |        4.58 |           7.678 |  67.63 |
|    22 |       0.498 |           0.765 |  53.83 |
在 TPCH 的测试中，datafusion 整体落后于 duckdb，除 4/13 要更快一些之外，其余的 20 个查询都更慢，差距比较大的是 2/7/11/15/17/18 号查询。

后续将逐一进行性能对比分析。

# datafusion 文章系列
1. [push vs pull](@/blog/2025-04-08-duck-push-vs-datafusion-pull/index.md)
2. [datafusion hashjoin executor](@/blog/2025-05-08-datafusion-hashjoin/index.md)
3. [datafusion window function executor](@/blog/2025-05-26-datafusion-window-function/index.md)
4. [datafusion performance](@/blog/2025-06-06-datafusion-performance/index.md)

