---
layout: ddd
prev: applyfp.html
up: learn-scala.html
---

# 如何在工程中践行 FP 六原则。

## 六原则
1. 无副作用纯函数
2. 不可变数据
3. 使用表达式而非语句
4. 简单化流程
5. 简单数据流
6. 隔离副作用

## 代码检查工具

基于 scalafix， 支持如下规则：

1. [ExplicitResultTypes](https://scalacenter.github.io/scalafix/docs/rules/ExplicitResultTypes.html) 为每一个 public 方法显示申明类型
2. [DisableSyntax](https://scalacenter.github.io/scalafix/docs/rules/DisableSyntax.html)
    1. noVars = true  不容许使用 var 
    2. noNulls = true 不容许使用 null
    3. noReturns = true 不容许使用显示 return
    4. noAsInstanceOf = true 不容许使用强制类型转换。在SOA中基本用不到
    5. noXml = true 未来不兼容
    6. noFinalize = true
    7. noFinalVal = true
3. 新增规则：不容许使用 scala.collection.mutable 下的包 和 java.util 下的集合包。
4. 新增规则：代码复杂度度量，单个方法的复杂度不超过300。（复杂度算法另文介绍）

新增规则的源代码会稍后放到 github 上。