+++
title = "DuckDB 源码阅读之相关链接"
description = "收集有关 duckdb 的源代码阅读的文档，计划接下来花一定的时间来阅读 duckdb."
date = 2024-09-10
draft = false
template = "blog/page.html"

[taxonomies]
authors = ["wangzx"]
+++

最近，在评估引入一个 SQL 执行引擎，以取代分析产品对上游数据库的特性的依赖，这些差异给产品带来了显著的困难：
1. MySQL 不支持 FullJoin
2. MySQL 的窗口函数中不支持 `range expr preceding and ...`， 无法执行如下的SQL：(同期累积值)
   ```sql
    select order_date, sum(amount) as x0, sum(sum(amount)) over (
    order by order_date 
    range between 
      date_diff('day', makeDate( year(order_date)-1, 1, 1), order_date) preceding 
    and 
      date_diff('second', makeDate( year(order_date)-1, month(order_date), day(order_date) ), order_date) preceding
    ) as X1
    from orders
    group by order_date;
   ```
3. ClickHouse 对窗口函数的支持极为不标准。

考虑引入 polars 或者 duckdb, 之前会偏向于 polars（主要是个人对C++已经生疏，对Rust的感觉要良好很多），但在对比窗口函数的能力方面
还是要欠缺很多，而 duckdb 在这方面的体验要好很多，所以目前更倾向于选择 duckdb。这样，就很有必要熟悉 duckdb 的源代码。这里收集网上与
duckdb 相关的源代码阅读文档，也给自己立一个flag，在接下里的时间里，重点阅读一下 duckdb 的源代码，顺便补一补 C++ 的功课。

引入一个 SQL 执行引擎的目的：
1. 屏蔽上游数据源的特性限制。提供更多、且更一致的SQL分析能力。
2. 基于向量计算、优化的执行机制，提供相比传统 OLTP 更快速的查询能力。
3. 必要时，可以根据自己的分析需要，进行针对性的优化。例如，常见的小计、合计等功能，使用 SQL 实现时，需要执行多条SQL，
   可以开发扩展进行针对性的优化。

## DuckDB 源代码剖析 文档收集

1. 张建:
   - [DuckDB Push-Based Execution Model](https://zhuanlan.zhihu.com/p/402355976)
   - [DuckDB Query Optimizer](https://zhuanlan.zhihu.com/p/696147374)
2. Focus
    - [DuckDB：开篇](https://zhuanlan.zhihu.com/p/374627729)
    - [DuckDB: Row-Group Based Storage](https://zhuanlan.zhihu.com/p/382131436)
    - [DuckDB：接口与示例](https://zhuanlan.zhihu.com/p/376178277)
3. franzcheng
    - [DuckDB源码阅读02——公共子表达式消除](https://zhuanlan.zhihu.com/p/644095258)
    - [DuckDB源码阅读——Filter pullup](https://zhuanlan.zhihu.com/p/652394118)
    - [DuckDB源码阅读01——重复聚合函数去除](https://zhuanlan.zhihu.com/p/644039061)
4. 红星闪闪
    - [DuckDB Sort代码阅读和分析](https://zhuanlan.zhihu.com/p/628391818) 重点围绕 Sort，偏重于 Pipeline + Executor
    - [怎样把数据库排序做到全球第一](https://zhuanlan.zhihu.com/p/664312966)
    - [DuckDB的变长Sort实现](https://zhuanlan.zhihu.com/p/628507841)
    - [DuckDB的Merge实现](https://zhuanlan.zhihu.com/p/628393152)
    - 
5. Others
   - [DuckDB内存格式代码详解（一）](https://zhuanlan.zhihu.com/p/679569277)

6. 官方 
   - DuckDB internals [Slides](https://15721.courses.cs.cmu.edu/spring2023/slides/22-duckdb.pdf), 
     [Video](https://www.youtube.com/watch?v=bZOvAKGkzpQ)
   - Push-Based Execution In DuckDB [Slides](https://dsdsd.da.cwi.nl/slides/dsdsd-duckdb-push-based-execution.pdf),
     [Video](https://www.youtube.com/watch?v=1kDrPgRUuEI)
   