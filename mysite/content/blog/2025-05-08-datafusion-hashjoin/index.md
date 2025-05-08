+++
title = "datafusion hashjoin 性能分析"
description = "datafusion 中的 hashjoin 算子进行性能分析，并尝试进行优化。"
date = 2025-05-08
draft = false
template = "blog/page.html"

[extra]
toc = true
+++

最近，阅读了 datafusion 项目的源代码，我会通过这篇文章，整理代码阅读中的一些笔记、思考。本文目前是处于草稿阶段（凌乱），会持续修改、更新。

# datafusion 性能测评

# Top-down pull execution(Volcano) vs bottom-up push execution(Pipeline)

# HashJoin 性能测评
1. in-order vs out-of-order
2. small build set vs large build set

# HashJoin 改进
1. Query Plan 改进
   - left join 改写为 inner join
   - filter move to left side
2. HashJoin 算子改进
   - 使用行存替代列存，以提高缓存有好性
   - 显示的 SIMD 优化
   - 新的 hashmap 数据结构设计

# Mac 下的性能测评工具
