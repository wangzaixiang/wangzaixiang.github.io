+++
title = "收集箱 ..."
description = "记录一些准备整理的主题"
date = 2024-12-31 
draft = true
template = "blog/page.html"
+++

1. Lit, SolidJS's incremental rendering architecture
2. Functional Programming & Design
   - 副作用隔离
   - 结构化编程 与 结构化数据流，SSA
   - 可测试性、可证明性
   - Monads, vs Direct Style
   - ADT 建模 
   - DuckDB 等数据库执行引擎内部从 Query -> AST -> Logical Plan -> Physical Plan -> Pipeline -> Executor 的设计，也是一种典型的
     函数式风格。中间过程的数据结构，visibility 能力，对理解、调试、优化都有很大帮助。
   - 在设计数据模型的 SQL 查询引擎时，规划出 Query 的查询计划，然后再执行，也是非常函数式的风格。
3. DDD
   - 业务中台
4. 孙子兵法
   - 军争篇之迂直之计
5. Web Components
6. Design by Contract
7. REPL style programming, Notebook, playground, text protocol, visibility
   - 模块化、包管理
   - 代码即文档
   - 测试友好
   - 反模式：configure, make, cmake, ant, maven, gradle, npm, webpack, etc.
8. Visibility 可以单独讲
   - 刻意练习
   - 企业管理与指标体系
   - 软件质量
   - 代码质量
   - Visibility 工具： 日志、监控、可视化、调试工具、REPL、Notebook
9. 软件复杂性之 拆分、重组、分层、分领域、分角色、正交
   - 技术复杂性、业务复杂性
   - 时间不确定性
   - 空间不确定性
10. 最优结构的标准：内聚、耦合、抽象层次一致性
11. TDD 与 设计，由外而内的设计 vs 由内而外的设计， 自上而下 VS 自下而上
12. 敏捷、精益
    - 识别浪费
    - Pull style
    - iteration, visualization, PDCA
    - TDD 与 以迂为直
13. Dax vs MDX vs Tableau Expression Language.
    - MDX optimize 整理
14. 理解 Rust 的语法糖
    - for in 
    - `x[i]`
    - deref
    - view HIR, MIR, LLVM-IR, ASM
15. Signal
    - [TC39](https://github.com/tc39/proposal-signals)
    - [airstream](https://github.com/raquo/Airstream)
    - 我设计的 Variable Manager 设计方案
    - Dom 增量更新 的三种模式
      - vdom Diff / Lit / Solid