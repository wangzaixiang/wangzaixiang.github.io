+++
title = "收集箱 ..."
description = "记录一些准备整理的主题"
date = 2025-12-31 
draft = true
template = "blog/page.html"

[extra]
toc = true
+++

1. Functional Programming & Design
   - 副作用隔离
   - 结构化编程 与 结构化数据流，SSA
   - 可测试性、可证明性
   - Monads, vs Direct Style
   - ADT 建模 
     - [ ] 对 Reference 建模。在一个复杂的 ADT 中，如何建模对其他实体的引用，并发边应用进行处理？ 
   - DuckDB 等数据库执行引擎内部从 Query -> AST -> Logical Plan -> Physical Plan -> Pipeline -> Executor 的设计，也是一种典型的
     函数式风格。中间过程的数据结构，visibility 能力，对理解、调试、优化都有很大帮助。
   - 在设计数据模型的 SQL 查询引擎时，规划出 Query 的查询计划，然后再执行，也是非常函数式的风格。
2. DDD
   - 业务中台
3. 孙子兵法
   - 军争篇之迂直之计
4. Web Components
5. REPL style programming, Notebook, playground, text protocol, visibility
   - 模块化、包管理
   - 代码即文档
   - 测试友好
   - 反模式：configure, make, cmake, ant, maven, gradle, npm, webpack, etc.
6. Visibility 可以单独讲
   - 刻意练习
   - 企业管理与指标体系
   - 软件质量
   - 代码质量
   - Visibility 工具： 日志、监控、可视化、调试工具、REPL、Notebook
7. 理解 Rust 的语法糖
   - for in 
   - `x[i]`
   - deref
8. Rust intrinsic
9. 产品质量思考
   - 产品定义的质量
   - 软件设计（定义、概念、API）
   - 软件设计（实现：拆分、架构）
   - 软件实现（编码）
   - 软件测试：可测试性、测试成本。
10. meta programming
   - dapeng SOA
   - macro: scala, rust, zig
   - java: reflection, JDBC metadata.
   - ADT / JSON Schema.
   - code as data, data as code