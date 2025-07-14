# Access Path Selection in Relation Database Management System

本文是介绍数据库查询计划的鼻祖论文，基于 IBM 早起的 System R 试验性关系数据库，在现在看来，应该是关系数据库的“旧石器时代”。

## 1. Introduction

## 2. Processing of an SQL statement
4 phases:
- parsing: SQL to Query Block(AST)
- optimization: catalog lookup, validate, plan generation(access plan selection)
- code generation
- execute

## 3. Research Storage System
1. Relation: Set of Tuple
2. Page(4K): Set of Tuple( each tuple tagged with relation-id, so can contain different relation's tuple)
   - no tuple spans a page 
3. Segment: Set of Page
4. Index: B-trees
   - clustered index
5. SCAN: open, next, close
   - Segment scan
   - Index scan
   - scan filter

## 4. Costs for an access-path
`COST = Page_Fetches + W * RSI_Calls`
- Page Fetches: IO Cost
- RSI Calls: CPU Cost

## 5. Access path selection for Joins
1. 2-way join
   - selection 1: outer(probe side) and inner(build side)
   - selection 2: join method 
     - nested loops
     - merging scan: 两个表在 join column 上已排序
     - 这篇文章没有提到 hash-join，这是现在主流数据库 join 的主要方式。
2. N-way join is a recur 2-way join
   - paths: 最多 `N!` 种可能性
3. optimization
   - 只考虑：inner side 与 outer side 有 join 连接的情况。
   - 构建一颗 access path tree，对每一条path，评估其 cost
   - N: `C-outer`的基数： `N = (product of the cardinalities of all relations T of the join so far) * (product of the selectivity factors of al 1 applicable predicates)` 
   - `C-nested-loop-join(path1, path2) = C-outer(path1) + N * C-inner(path2)`
   - `C-merge(path1, path2) = C-outer(path1) + N * C-inner(path2)` -- 不太理解这个 cost 函数，merge 时，C-inner(path2) 会显著小于 nested loop。
4. 思考：对最新的关系数据库而言，优化应考虑：
   - filter 尽量下压到 scan 阶段
   - 如果不能下压到 scan 阶段，则尽量提前到 join 之前。
   - 使用 hash-join 时，尽可能选择技术小的作为 build side，技术大的作为 input-side
   - 如果左右两侧已按 join column 排序，则可选择 merge-sort join

## 6. Nested Query
1. 非相关子查询：如果子查询中未使用自由变量，则可以独立执行，而无需针对每一行展开执行。
2. 相关子查询