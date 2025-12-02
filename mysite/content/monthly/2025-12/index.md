+++
title = "Dec 2025"
description = "2025-112 月记"
date = 2025-12-02
draft = false
template = "blog/page.html"
+++

# Languages
## Rust
 
## Scala

# Mpp & OLAP
1. [data storage format](https://dipankar-tnt.medium.com/apache-parquet-vs-newer-file-formats-btrblocks-fastlanes-lance-vortex-cdf02130182c)
   - apache parquet: 
     - columnar layout, row group and pages, encoding and compression, statistics and filters, interoperability.
     - decode bottlenecks, 
     - Random access inefficiency
     - Memory pressure, Row Groups typically in 128M
     - lack of SIMD/GPU awareness
   - [BtrBlocks](https://github.com/maxi-k/btrblocks)
     - 不牺牲压缩率的前提下，极致优化解压和扫描速度。解压专为 SIMD 优化，压缩率极高（通过智能选择最佳编码组合）。
     - 压缩性能并不比 Parquet 更耗时，通常写得更快。
     - 智能选择 + 简单算法 > 固定选择 + 复杂算法。
   - [FastLanes](https://github.com/cwida/FastLanes): Next-Gen Big Data File Format
     - 极致压缩率 + 解压缩性能。
     - 数据并行编码：FOR/RLE/DICT 可以高效在 SIMD/GPU 上运行
     - 表达式编码
     - 多列压缩：识别不同列之间的相关性。
     - 细粒度访问和部份解压。
   - [Lance](https://lancedb.com/docs/storage): 核心目标：处理 AI 应用中对多模态、快速更新和高效随机访问的需求。
     - 高性能随机访问
       - parquet 重压缩，变长编码，以数据块为单元。（RowGroup -> Column Chunk -> Page）
       - Lance 将数据铺的更有序，可直接计算偏移量。（更小的 Fragments，每个Frag有独立的元数据句和 Min/Max 信息，Frags有全局索引）
       - 编码方式：Lance：固定宽度 + 偏移量表。
       - 索引机制：Parquet 没有行号索引。Lance 可以通过 索引 -> RowId -> 偏移量 快速定位。
       - I/O: parquet 基于大块读取，Lance 倾向随机访问。基于 SSD 和 io_uring 技术，一次性发出多个小读请求。
     - 原生多模态数据支持
     - 集成向量搜索
     - 零拷贝 Schema 演进：高效的添加或修改列。
     - 版本控制
   - [Vortex](https://research.google/pubs/vortex-a-stream-oriented-storage-engine-for-big-data-analytics/)
     - [web site](https://vortex.dev)
     - [github](https://github.com/vortex-data/vortex) 
     - 旨在成为 Apache Parquet 的继任者
     - Arrow 原生：zero copy
     - 级间压缩：类似于 BtrBlock。
     - 极致扩展性：容许用户自定义编码，布局策略和压缩算法。
     

# Web & Frontend

# AI & Agent

# Misc
