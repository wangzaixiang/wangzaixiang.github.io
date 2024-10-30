+++
title = "数据库查询优化相关技术收集 ..."
description = "收集有关 OLAP 查询优化相关的策略、算法，为后续评估向量计算体系的优化准备素材"
date = 2024-08-22T12:00:00+00:00
draft = false
template = "blog/page.html"

[taxonomies]
authors = ["wangzx"]

[extra]
toc = true
+++

## 优化策略 Summary
每一个章节都可以写若干论文
1. Expression Rewriter
2. Filter Push-down
3. Join 
   - 算法选择： Hash(parallel, grace), Sort(full, partial), 
   - 表顺序
   - filter push-down
4. Subquery as Join
   - [子查询的查询优化](https://ericfu.me/subquery-optimization/)： Executor -> Evaluator -> Executor 的执行模式是非常低效的
   - [Calcite 中新增的 Top-down 优化器](https://ericfu.me/calcite-top-down-planner)
5. Window 函数优化
   - 窗口：重用窗口、增量式窗口
   - 聚合：增量式聚合
6. group-by + 聚合函数优化
   - 分区: map-reduce
   - avg() 转换为 SUM()/COUNT()
   - 消除 grouped dataframe
7. 减枝
   - 裁剪列
   - 裁剪表（关联）
8. 索引优化
   - minmax 
   - bloom
   - bitmap
   - sorted index
9. vectorized using SIMD + GPU
10. cache, material CTE

## MySQL

1. [Optimizing SELECT Statements](https://dev.mysql.com/doc/refman/8.0/en/select-optimization.html)
   1. [WHERE Clause Optimization](https://dev.mysql.com/doc/refman/8.0/en/where-optimization.html)
      - 移除不必要的括号 (表达式优化)
      - Constant folding `(a<b AND b=c) AND a=5` -> `b>5 AND b=c AND a=5`
      - Constant condition removal. 
      - count(*) 优化
      - 在没有 group by 时， having 合并到 WHERE 中
      - where 下沉到 join 表
      - constant tables 提前读取
      - join 顺序：如果 order by / group by 字段来源于单个表，则这表优化。
   2. [Range Optimization](https://dev.mysql.com/doc/refman/8.0/en/range-optimization.html)
   3. [Window Function Optimization](https://dev.mysql.com/doc/refman/8.0/en/window-function-optimization.html)
      - 如果子查询具有窗口函数，则禁用子查询的派生表合并。子查询总是具体化的。
      - 半连接不适用于窗口函数优化，因为半连接适用于 WHERE和中的子查询JOIN ... ON，其中不能包含窗口函数。
      - 优化器按顺序处理具有相同排序要求的多个窗口，因此可以跳过第一个窗口之后的排序。
      - 优化器不会尝试合并可以在单个步骤中评估的窗口（例如，当多个 OVER子句包含相同的窗口定义时）。
        解决方法是在子句中定义窗口并在 WINDOW子句中引用窗口名称OVER。
2. [子查询、派生表、视图与CTE的优化](https://dev.mysql.com/doc/refman/8.0/en/subquery-optimization.html)
   1. subquery: scalar subquery, 1-row subquery, or N-rows subquery
   2. derived table `select ... from (subquery) as tbl` or `select * from JSON_TABLE(arg_list) as tbl`
   3. view reference
   4. Common Table Expression
   
   优化策略：
   1. `IN, = NAY, EXISTS` subquery => Semijoin / materialization / Exists-Strategy
   2. `NOT IN, <> ALL, NOT EXITS` subquery=>  materialization / Exists-Strategy
   3. derived table, view, CTE => merge into query / material  
   
   - Semi-join 优化：将子查询转化为 Semi-Join, 如果子查询使用了 range 操作，是无法进行改优化的。
   - 物化：当子查询没有引用外部表的字段时，是可以物化的
   - (a, b, c) IN (select ... ) 转化为 exists (select ... where ...) 
   - [Merging](https://dev.mysql.com/doc/refman/8.0/en/derived-table-optimization.html): 在 Mergeing 后再进行化简，避免一次重复的子查询。
   - Condition pushdown：将查询条件下压到 CTE。
   - 如果都不能走上述优化，则可能按需执行，在 select 部分，会对美航执行子查询，因而会出现性能问题。


## ClickHouse

1. [Join 算法](https://clickhouse.com/docs/en/guides/joining-tables#optimizing-join-performance)
   - hash, parallel hash
   - grace hash: right table 过大时，将 hashtable 拆分为多个，每次仅在内存中存放一个分区，其他放到磁盘保存。
   - full sorting merge: 左表、右表都在关联字段上排序
   - partial merge：右表在关联字段上排序。
   - direct: 类似于 Hash，针对于特定的存储引擎类型。
   
   {{ resize_image(path="@/blog/sql-optimize/clickhouse-joins.png", width=600, height=400, op="fit_width") }}
   
   1. [Join Types supported in ClickHouse](https://clickhouse.com/blog/clickhouse-fully-supports-joins-part1)
   2. [Hash Join, Parallel HashJoin, Grace HashJoin](https://clickhouse.com/blog/clickhouse-fully-supports-joins-hash-joins-part2)
   3. [Full Sort Merge, Partial Merge Jooin](https://clickhouse.com/blog/clickhouse-fully-supports-joins-full-sort-partial-merge-part3)
      > 从文档上看，如果右表已经是排序的，也需要做一次排序，感觉这个是多余的
   4. [Direct Join](https://clickhouse.com/blog/clickhouse-fully-supports-joins-direct-join-part4)

## Subquery 优化

1. left join
   ```sql 
   SELECT o_orderkey, (
    SELECT c_name
    FROM CUSTOMER
    WHERE c_custkey = o_custkey
   ) AS c_name FROM ORDERS;

   SELECT o_orderkey, c_name
   FROM orders left join customers on c_custkey = o_custkey
   ```
2. semi join
   ```sql
    SELECT c_custkey
    FROM CUSTOMER
    WHERE c_nationkey = 86 AND EXISTS(
        SELECT * FROM ORDERS
        WHERE o_custkey = c_custkey
   )
   
   select c_custkey from customer 
   semi join orders on o_custkey = c_custkey 
   where _nationkey = 86
   ```
3. group by
   ```sql
    SELECT c_custkey
    FROM CUSTOMER
    WHERE 1000000 < (
      SELECT SUM(o_totalprice)
      FROM ORDERS
      WHERE o_custkey = c_custkey
    );
   
    SELECT c_custkey, SUM(o_totalprice)
    From customer left join orders on o_custkey = c_custkey
    group by c_custkey
    having 1000000 < SUM(o_totalprice)
   ```
   
4. 集合比较
   ```sql
    SELECT c_name
    FROM CUSTOMER
    WHERE c_nationkey <> ALL (SELECT s_nationkey FROM SUPPLIER);
   
    select c_name from customer anti semi join supplier on c_nationkey = s_nationkey;
   
   ```