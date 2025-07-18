+++
title = "面向分析场景的 SQL 扩展思考"
description = "datafusion 性能分析"
date = 2025-06-06
draft = false
template = "blog/page.html"

[extra]
toc = true
+++

# 在一个计算流中输出多个结果
目前的 Query 都只能输出单个结果集，如果能在一个计算图中同时输出多个结果集，在一些 BI 场景会有性能上的优化。一个典型的案例就是：在查询第一页数据的同时，
返回总行数。目前的做法是拆分为2个独立的查询：Q1 查询第一页数据，Q2 查询总行数。如果结合到一个计算图中，则可以共享很多算子的计算，从而减少重复计算消耗。

# group by grouping set, cube, rollup
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
# aggregate with filter
   - 支持数据库
     - [duckdb](https://duckdb.org/docs/stable/sql/functions/aggregates)
     - datafusion: 目前并不支持
   - 适用场景：
     - 在分组计算时，支持定义包含更多 filter 的度量计算。
# aggregate with rollup
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
# 结合 aggregate with filter & rollup

   如果能够在 datafusion 中同时支持 aggregate with filter & rollup 的功能的话，那么以下的 OLAP 计算就会变得简单且高效：
   - with filter 类似于 MDX 的 `([Measure].[X], [DimMore].[m1])`
   - with rollup 类似于 MDX 的 `([Measure].[X], [Dim1].[AllDim1s])`
   - 组合： 实现二者的等效操作。
   - 限制：with filter & with rollup 都在 where 之后执行，其基础数据受 where 条件的限制，无法在 filter 中变更数据范围，虽然有这个限制，
     仍然可以满足很大部份的 MDX 指标计算的能力 
# More Aggregate Functions
    
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

# as-of join
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

# 高效执行 scalar subquery
   使用 scalar subquery (可以出现在 SelectItem, Where 部份)，来描述计算度量，具有较强的表达能力，可以覆盖上述的 aggregate with rollup 的能力，
   以及更为复杂的一些计算场景，包括：
   - 时间快速计算，如 同期值、前期值、年（季、月）累计、同期累计值
   - 组内占比
   
   如果 SQL 执行引擎能够高效的执行 scalar subquery, 那么对 OLAP 分析来说，就很有价值了。
   - [SQL 子查询的优化](https://ericfu.me/subquery-optimization/)
   - 文献：[Unnesting Arbitrary Queries](https://ericfu.me/subquery-optimization/)
   - [4.5 duck-关联子查询(binder->plan->optimize)](https://zhuanlan.zhihu.com/p/21395872495)
   
   - TPCH Query2
     ```sql
     select	s_acctbal,	s_name,	n_name,	p_partkey,	p_mfgr,	s_address,	s_phone,	s_comment	
     from	part,	supplier,	partsupp,	nation,	region	
     where	p_partkey = ps_partkey	and s_suppkey = ps_suppkey	and p_size = 15	and p_type like '%BRASS'	
        and s_nationkey = n_nationkey	and n_regionkey = r_regionkey	and r_name = 'EUROPE'	
        and ps_supplycost = (	
            select	min(ps_supplycost)	from	partsupp,	supplier,	nation,	region	
            where	p_partkey = ps_partkey	-- p_partkey is from outer part
                and s_suppkey = ps_suppkey	and s_nationkey = n_nationkey	
                and n_regionkey = r_regionkey	and r_name = 'EUROPE'	
       )	
     order by	s_acctbal desc,	n_name,	s_name,	p_partkey;
     ```
     对这个子查询, duckdb 和 datafusion 都具有良好的优化能力。
   - datafusion 48.0.0 暂不支持如下的查询（duckdb支持，但其查询计划不是很好阅读）：
     ```sql
        select 
            o1.o_custkey,
            o1.o_orderdate, 
            sum(o1.o_totalprice),
            (
                select sum(o2.o_totalprice)
                from orders o2
                where 	o2.o_custkey = o1.o_custkey 
                    and o2.o_orderdate >= date_trunc('year', o1.o_orderdate) 
                    and o2.o_orderdate <= o1.o_orderdate
            ) as "年累计"
        from orders o1 
        where 
            o1.o_orderdate <= date'1992-02-10' 
            and o1.o_custkey in (3689999, 7800163)
        group by 1, 2;
     ```     
     错误：`This feature is not implemented: Physical plan does not support logical expression ScalarSubquery(<subquery>)`
   - [ ] idea: 新增一种 reduce join, outer : inner = 1: N， 将多行 inner 的数据进行聚合后，作为 inner 的输出。
     在 join 算子中完成聚合操作，减少 join 的 output。
   - [ ] idea：在 reduce join 中，将部份 window function 的执行优化策略引入，减少 working set 以及聚合计算量。

