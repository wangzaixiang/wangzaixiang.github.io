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