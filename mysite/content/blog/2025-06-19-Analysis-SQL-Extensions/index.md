+++
title = "面向分析场景的 SQL 扩展思考"
description = "datafusion 性能分析"
date = 2025-06-06
draft = false
template = "blog/page.html"

[extra]
toc = true
+++

1. group by grouping set, cube, rollup
   - 支持数据库
     - [duckdb](https://duckdb.org/docs/stable/sql/query_syntax/grouping_sets)
     - [postgres 9.5](https://www.postgresql.org/docs/current/queries-table-expressions.html#QUERIES-GROUPING-SETS)
     - [mysql 8.0.19](https://dev.mysql.com/blog-archive/improvements-to-rollup-in-mysql/)
     - datafusion: 目前并不支持
   - 使用场景：将分组、小计、合计等多个查询在同一个 SQL 中完成。
   - 性能优化：
     - 在最小粒度上进行 group aggregate
     - 在上层粒度上进行 rollup，避免重复计算。
     - 在同一个算子中完成 grouping set 的计算。
2. aggregate with filter
   - 支持数据库
     - [duckdb](https://duckdb.org/docs/stable/sql/functions/aggregates)
     - datafusion: 目前并不支持
   - 适用场景：
     - 在分组计算时，支持定义包含更多 filter 的度量计算。
3. aggregate with rollup
   ```sql
    SELECT category, product, SUM(amount) as amount, 
        SUM(amount) rollup(category) as amount_of_product,  -- 在 category 上 rollup
        SUM(amount) rollup(product) as amount_of_category,  -- 在 product 上 rollup
    FROM orders
    GROUP BY category, product
   ```
   目前还没有看到哪个数据库支持这种形式的查询。其不同于 group by grouping, 后者会返回不同的分组， aggregate with rollup 是在当前的分组上
   返回 rollup 的结果。
   
   评估：在 datafusion 中增加这个能力，并考虑在单个算子中完成计算。
4. 结合 aggregate with filter & rollup

   如果能够在 datafusion 中同时支持 aggregate with filter & rollup 的功能的话，那么以下的 OLAP 计算就会变得简单且高效：
   - with filter 类似于 MDX 的 `([Measure].[X], [DimMore].[m1])`
   - with rollup 类似于 MDX 的 `([Measure].[X], [Dim1].[AllDim1s])`
   - 组合： 实现二者的等效操作。
   - 限制：with filter & with rollup 都在 where 之后执行，其基础数据受 where 条件的限制，无法在 filter 中变更数据范围，虽然有这个限制，
     仍然可以满足很大部份的 MDX 指标计算的能力
5. More Aggregate Functions
    
   参考：https://duckdb.org/docs/stable/sql/functions/aggregates

   | function                 | duckdb | duckdb-order-by | datafusion  | description                                                                                        |
   |--------------------------|--------|-----------------|-------------|----------------------------------------------------------------------------------------------------|
   | any_value(arg)           | Y      | Y               | N           | Returns the first non-null value from arg, support order by                                        |
   | arbitrary(arg)           | Y      | Y               | first_value | Returns the first value(null or non-null) from arg, support order by                               |
   | arg_max/min(arg, val)    | Y      | Y               | N           | Finds the row with the maximum val and calculates the arg expression at that row. support order by |
   | arg_max/min(arg, val, n) | Y      | Y               | N           | return a List of N                                                                                 |
   | arg_max_null(arg, val)   | Y      | Y               | N           | ignore rows where val is null                                                                      |
   | array_agg(arg)/list      | Y      | Y               | Y           |                                                                                                    |
   | bit_and/or/xor(arg)      | Y      | N               | Y           |                                                                                                    |
   | bool_and/or(arg)         | Y      | N               | Y           |                                                                                                    |
   | count(*)/count(val)      | Y      | N               | Y           |                                                                                                    |
   | max(arg)                 |        |                 |             |                                                                                                    |
   | max(arg, n)              |        |                 | N           |                                                                                                    |
   | max_by(arg, val, n)      |        |                 | N           |                                                                                                    |
   |                          |        |                 |             |                                                                                                    |
   |                          |        |                 |             |                                                                                                    |
   |                          |        |                 |             |                                                                                                    |

6. as-of join
   ```sql 
    SELECT h.ticker, h.when, p.price * h.shares AS value
    FROM holdings h
    ASOF JOIN prices p -- 关联到 1 条 prices 记录，去 p.when 中最大的记录， inner join 丢弃找不到的 join, or left join 采用 outer join
         ON h.ticker = p.ticker
         AND h.when >= p.when;
   ```
   - 支持数据库
     - [duckdb](https://duckdb.org/docs/stable/guides/sql_features/asof_join#inner-asof-joins)
   - 适用场景
     - 时点类指标。

7. as-of-2 join 命名再考虑
   ```sql
    SELECT h.ticker, h.when, max_by(p.price) * h.shares AS value
    FROM holdings h
    ASOF2 JOIN prices p -- 关联到 1 条 prices 记录，去 p.when 中最大的记录， inner join 丢弃找不到的 join, or left join 采用 outer join
         ON h.ticker = p.ticker
         AND h.when >= p.when;
   ```
   目标：left 表的一行记录关联多行 right 记录时，以 left 表作为一个分组，将 right 的多行记录使用聚合函数进行聚合。
   适用场景：
   - 简化 group by
   - 执行优化：在 JOIN 算子中，一次性完成 聚合函数的计算，减少中间的输出行数。