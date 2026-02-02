# OLAP 引擎技术对比分析

欢迎来到 OLAP 引擎技术对比分析学习笔记。

## 项目目标

本项目旨在深入理解和对比主流 OLAP（在线分析处理）引擎的技术实现，重点关注：

1. **架构设计**：执行模型、优化器、存储管理等核心架构
2. **性能特点**：向量化、SIMD、并行化等性能优化技术

## 研究对象

我们将对比分析以下六款 OLAP 引擎：

### 嵌入式/进程内引擎

- **[DuckDB](./engines/duckdb.md)** - 嵌入式分析型数据库，被称为"分析型的 SQLite"
- **[DataFusion](./engines/datafusion.md)** - Apache Arrow 生态的 Rust 查询引擎
- **[Polars](./engines/polars.md)** - 高性能 DataFrame 库，基于 Arrow 和 Rust

### 分布式/执行引擎

- **[Spark SQL](./engines/spark-sql.md)** - Apache Spark 的 SQL 执行引擎
- **[Trino](./engines/trino.md)** - 分布式 SQL 查询引擎（前身为 PrestoSQL）
- **[Velox](./engines/velox.md)** - Meta 开源的统一执行引擎

## 学习方法

本项目采用问答式学习方法：

1. **提出问题**：针对特定技术点提出问题
2. **记录问答**：将问答内容记录到 `qa/` 目录
3. **整理总结**：定期将问答整理到本 mdBook 中
4. **数学建模**：从数学角度理解算法和数据结构
5. **可视化**：使用图表和架构图辅助理解

## 如何使用

- 左侧导航栏可以浏览各个章节
- 每个引擎有独立的章节介绍
- 架构和性能部分提供横向对比
- 综合对比章节提供总结性分析

让我们开始探索 OLAP 引擎的技术世界！
