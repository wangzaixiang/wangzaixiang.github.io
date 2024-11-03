+++
title = "November 2024"
date = 2024-11-01
draft = false
template = "blog/page.html"

[extra]
toc = true
+++

# Languages
1. [Improve an algorithm performance step by step](https://blog.mapotofu.org/blogs/rabitq-bench/)
   - 使用 samply 工具收集性能数据
   - 使用 criterion 库进行性能测试
   - metrics 采集
   - 编译选项：例如 `RUSTFLAGS="-C target-feature=+popcnt"`
   - SIMD 加速，如果算法可以调整为 SIMD 优化的算法，加速效果会很明显。
   - 选择更好的库： faer 
   - IO 优化
   - const generics
2.

# MPP & OLAP

# Web & Visualization
1. bun 1.1.34 支持 Wasm GC，dart/kotlin/scalajs 等可以编译为 wasm 的语言都可以使用 bun 了。

# Tools & Libraries