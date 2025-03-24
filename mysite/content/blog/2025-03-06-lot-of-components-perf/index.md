+++
title = "大量组件树下的性能对比"
description = "大量组件树下的性能对比"
date = 2025-03-06
draft = false
template = "blog/page.html"
+++

这个测试是在创建大量组件树的情况下，对比几个前端框架的性能，为在高性能敏感的场景下选择合适的框架提供参考。

## 场景说明：
1. App 包括 1 个 PerfComp20 组件
2. 1个 PerfComp20 组件包括 26 个 PerfComp11 组件
3. 1个 PerfComp11 组件包括 1 个 PerfComp10 组件
4. 1个 PerfComp10 组件包括 200 个 PerfComp01 组件
5. 1个 PerfComp01 组件包括 5 个 PerfComp00 组件
6. 整个页面包括：
   - 26 * 200 * 5 = 26000 个 PerfComp00 组件
   - 26 * 200 = 5200 个 PerfComp01 组件
   - 26 = 26 个 PerfComp10 组件
   - 26 = 26 个 PerfComp11 组件
   - 1 = 1 个 PerfComp20 组件
   - 1 = 1 个 App 组件
   - 总共 31254 个组件
   
## 测试结果：

| Part      | Svelte   | Vue3     | Lit      |
|-----------|----------|----------|----------|
| Scripting | 134.28ms | 332.95ms | 427.68ms |
| Styling   | 17.39ms  | 15.3ms   | 59.71ms  |
| Layout    |          | 51.69ms  | 124.53ms |
| Paint     | 2.10ms   | 8.6ms    | 34ms     |

| Part      | Lit0  | Lit1  | Lit 2 |
|-----------|-------|-------|-------|
| Scripting | 468ms | 418ms | 406ms |
| Styling   | 104ms | 60ms  | 29ms  |
| Layout    | 111ms | 124ms | 47ms  |
| Paint     | 37ms  | 37ms  | 22ms  |

Lit 0: all using shadow dom
Lit 1: PerfComp00 using no shadow dom
Lit 2: PerfComp00(simple text node) + PerfComp01 using no shadow dom

## 初步结论
1. 基于 Lit 的 Web Component 用于 micro component 时，导致页面中大量的 shadow dom 和 大量的DOM时
   对性能的影响较大，尤其是对 styling 和 layout 的影响的耗时影响较大。
2. 组件初始化性能最佳的是 Svelte，其次是 Vue3，最差的是 Lit。Svelte 的主要优势在于其编译时的静态优化。
3. 本次测试较为变态，在一般应用场景中如不涉及到如此巨大的组件树（建议一般控制在2000以下时），一般无需担心初始化的性能问题。

## 参考：
[Git Repo](https://github.com/wangzaixiang/lot-of-elements-compare)

