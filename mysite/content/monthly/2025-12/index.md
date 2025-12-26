+++
title = "Dec 2025"
description = "2025-112 月记"
date = 2025-12-02
draft = false
template = "blog/page.html"
+++

# Languages
## Rust
1. [ECS vs Actor Model in Rust](https://medium.com/@theopinionatedev/ecs-vs-actor-model-in-rust-the-architecture-showdown-abd1d241014e)

   | Feature                  | ECS                            | Actor Model             |
   |--------------------------|--------------------------------|-------------------------|
   | CPU Throughput           | 🔥 Extremely high              | ⚡ Medium                |
   | Memory layout            | Contiguous, cache-hot          | Scattered               |
   | Async workflows          | ❌ Hard                         | ✅ Natural               |
   | Distributed systems.     | ❌ No                           | ✅ Perfect               |
   | Scaling across machines. | ❌ Not applicable               | ✅ Built for this        |
   | Data-driven workloads.   | ✅ Best in class                | ❌ Inefficient           |
   | Isolation                | ❌ Shared memory model          | ✅ Strong isolation      |
   | Parallelism.             | Automatic via borrows          | Actor-per-thread        |
   | Ideal Use.               | Simulation, compute, pipelines | Services, orchestration |
   
   - 从性能的角度，ECS 类似于列存，适合于 OLAP 型的批量、高吞吐应用，而 Actor Model 类似于行存，更适合 OLTP 型的小交互，大并发交互式应用。
   - Bevy 等 ECS 架构可以通过借用分析来识别可并行的操作（如果两个操作在数据访问上没有借用冲突，则可以并行），而 Actor Model 则通过消息传递来实现运行时隔离。
   - 从扩展性的角度对比
     - Actor 天然匹配水平扩展。
     - ECS 高度匹配垂直扩展（单机多核）。
     - ECS 对 OCP 原则的支持更好（通过组件组合实现新功能），而 Actor Model 则更依赖于继承和多态。
     - 康威定律：ECS 可以更好的支持单实体多组件跨团队的独立开发，而 Actor Model 则更适以实体为单位的团队划分。
 
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
- Elm
  手痒，又摸了一下 elm 这个语言。
  - 纯函数式语言，类似 Haskell。
  - 没有 null 和 undefined，所有值都是不可变的。
  - 强类型系统，类型推导。重构时非常友好、安全。
  - model-view-update 架构。非常的函数式。
  - 号称：“没有运行时错误的语言”。
  - 性能不错：
    - 收益于不可变数据结构，可以使用 === 来快速比较数据变化。
    - Html.Lazy 来避免不必要的 DOM 更新。（不仅可以避免更新，甚至 view 函数都不会被执行）
    - Keyed 节点重用。
  - 激进优化
    - 纯函数 + 不可变 + 强类型，没有动态属性访问。
    - record renaming：编译时将 record 的字段名替换为数字索引，避免运行时的字符串查找。
    - static function inlining。
    - ELM 的类型系统保证了一个 Record 结构在运行期不会动态变化，匹配浏览器的 shape optimization。
    - constant folding。
  - 最小提及：25-30K，但随着应用规模的增长，Elm 的体积增增长曲线要比 JS 平缓得多。
  - [A Real-World Comparison Of Front-End Frameworks With Benchmarks 2024](https://expertbeacon.com/a-real-world-comparison-of-front-end-frameworks-with-benchmarks/)
  - 代码密度：
    - UI 描述：代码量偏多
    - 状态管理：Model -> Msg -> Update -> View，代码量偏多(每次修改都需要同步这4个地方)，但非常清晰。
    - 业务逻辑：代码量适中，但非常清晰。
    - 可读性最佳。
  - [ROC](https://www.roc-lang.org) : 类似 Elm 的纯函数式语言，基于 Rust 语言，脱离 JavaScript 生态。

# AI & Agent

# Misc
