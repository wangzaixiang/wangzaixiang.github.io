+++
title = "Mar 2026"
description = "2026-03 monthly"
date = 2026-03-01
draft = false
template = "blog/page.html"
+++

# Languages
## Rust
 
## Scala

# Mpp & OLAP
     
# Web & Frontend
- [Building a Pure-Rust Charting Library](https://autognosi.medium.com/scalable-crates-with-lod-in-wasm-interactive-svg-charts-in-pure-rust-for-dashboards-f6f5103dba38)
  - [lodviz-rs](https://github.com/automataIA/lodviz-rs)
  - 基于 Leptos 的 react, 纯 rust, 通过 wasm 编译成 web 前端，提供交互式的 svg 图表库，适合 dashboard 场景。
  - lodviz_core: 无UI依赖的核心库，提供数据处理、LOD计算等功能。
  - lodviz_components
  - 11种图表类型，支持交互式功能如缩放、工具提示、图例等。
  - LOD: 通过采样技术，对大型的数据集进行采样，减少渲染开销。通过在 rust 中实现这些算法，相比 JS 提高性能。
    - LTTB: Largest Triangle Three Buckets，适合大规模数据的线图降采样算法。
    - M4: 可视化层面的再聚合（first/last/min/max)
    - Gaussian KDE: 基于高斯核密度估计的降采样算法，适合散点图等。
  - 图形语法：类似于 vega-lite。
  
  我年前也实现了一个 viz 引擎，对 d3, vega, vega-lite 等图形语法相对深入的进行学习和对比，最好还是自己设计了 viz-spec 语法（也是借鉴 vega-lite）。看这篇文章后，感受更深刻一些。

# AI & Agent

# Misc
