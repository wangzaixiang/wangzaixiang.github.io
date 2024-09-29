# subquery optimization

subquery 的优化包括：
1. 将 行级的 subquery 转换为 batch的 JOIN 操作

表达能力：
- 与 MDX 的 Top-Down + Pull 模型匹配，可以支持复杂的度量计算。
  - 类似于 DAX 的 Calculate 函数，修改查询上下文。