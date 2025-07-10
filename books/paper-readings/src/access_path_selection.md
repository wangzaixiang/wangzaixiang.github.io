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
   - selection 2: join method (现代的数据库还有很多的方式，比如 hash-join 就是最常用的方法)
     - nested loops
     - merging scan
2. N-way join is a recur 2-way join
   - paths: 最多 `N!` 种可能性
3. optimization
   - 