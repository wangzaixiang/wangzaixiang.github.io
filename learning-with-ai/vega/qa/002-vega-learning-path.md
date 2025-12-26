# Vega 深度学习路线图：原理与模型篇

这份学习路线图旨在帮助你从**系统设计**和**内部原理**的角度理解 Vega，而不仅仅是将其作为绘图工具。我们将深入探讨它是如何将声明式规范转换为可执行的响应式数据流的。

## 第一阶段：语言设计与语法 (Language Design & Grammar)
**目标**：理解 Vega 作为一种“语言”的设计词汇、语法结构及其背后的抽象思想。

1.  **解剖 Vega 规范 (Anatomy of a Specification)**
    *   **顶层构建块**：`data` (数据), `transforms` (变换), `scales` (比例尺), `projections` (投影), `axes` (坐标轴), `legends` (图例), `marks` (标记), `signals` (信号)。
    *   **设计决策**：为什么选择 JSON？为什么将 `scales` 和 `axes` 分离？(对比 D3 的紧耦合 vs Vega 的解耦)。
2.  **图形语法的映射 (Mapping the Grammar of Graphics)**
    *   回顾 Wilkinson 的理论：Data -> Trans -> Scale -> Coord -> Element -> Guide。
    *   Vega 的具体实现：如何用 `marks` 对应 Element，用 `scales` + `guides` 对应 Scale + Guide。
    *   **关键差异**：Vega 引入了 `Signal`，这是传统静态图形语法中不存在的概念，它将“交互”提升为语法的核心组成部分。
3.  **声明式逻辑与表达式 (Declarative Logic)**
    *   **Expressions**：Vega 规范中的字符串表达式 (e.g., `"datum.x * 2"`) 是如何赋予 JSON 逻辑能力的？
    *   **Predicates (谓词)**：用于条件判断的抽象。
4.  **组合与布局 (Composition & Layout)**
    *   `Group` 标记的作用：不仅是图形分组，更是**作用域 (Scope)** 的边界（数据、信号的作用域）。
    *   Layouts (GridLayout, TreeLayout)：如何在语法层面描述复杂的空间排列。

## 第二阶段：核心设计哲学 (Core Design Philosophy)
**目标**：理解 Vega 为什么要这样设计，以及它在可视化领域的定位。

1.  **图形语法 (Grammar of Graphics) 的演变**
    *   从 Leland Wilkinson 的《The Grammar of Graphics》到 ggplot2。
    *   Vega 的突破：引入**交互图形语法 (Interaction Grammar)** 和 **响应式 (Reactivity)**。
2.  **核心论文阅读**
    *   **必读**：[Reactive Vega: A Streaming Dataflow Architecture for Declarative Visualization](https://idl.cs.washington.edu/papers/reactive-vega/) (2014)
    * 本地链接：[2015-ReactiveVega-InfoVis.pdf](../2015-ReactiveVega-InfoVis.pdf)
    *   重点理解：
        *   为什么要将可视化状态建模为数据流 (Dataflow)？
        *   声明式规范 (JSON) 如何映射到运行时模型？

## 第三阶段：响应式数据流架构 (Reactive Dataflow Architecture)
**目标**：理解 Vega 运行时的“心脏”——Dataflow Engine。

1.  **数据流图 (Dataflow Graph)**
    *   **Operator (算子)**: 计算的基本单元 (Transform, Scale, Encode)。
    *   **Topology (拓扑)**: 依赖关系如何决定计算顺序。
2.  **核心原语**
    *   **Signals (信号)**: 它是如何驱动交互的？理解 Signal 作为一个“随时间变化的变量”在图中的作用。
    *   **Pulse (脉冲)**: 数据更新的传播机制。理解 `add`, `rem`, `mod` 如何通过 Tuple ID 追踪数据变化，实现增量更新 (Incremental Updates)。
3.  **调度 (Scheduler)**
    *   Propagation Loop (传播循环)：变化是如何从 Source 节点传导到 Sink 节点的。
    *   如何避免不必要的重计算 (Lazy evaluation / Pruning)。

## 第四阶段：场景图与渲染 (Scenegraph & Rendering)
**目标**：理解抽象的数据如何变成屏幕上的像素。

1.  **Scenegraph (场景图) 模型**
    *   它是一个树状结构，包含 Group, Mark, Item。
    *   理解 Vega 的 Scenegraph 与 DOM 树或 Canvas 绘图指令的区别。
2.  **渲染管线**
    *   `Dataflow` -> `Scenegraph` -> `Renderer`。
    *   Renderers: Canvas vs SVG (以及 WebGL 的扩展)。
    *   Bounds Calculation (边界计算): 这是一个复杂但关键的步骤，用于布局和脏矩形检测。

## 第五阶段：交互与事件系统 (Interaction & Event System)
**目标**：理解用户的鼠标点击是如何转化为数据流的变化的。

1.  **Event Streams (事件流)**
    *   DOM 事件如何被捕获并转化为 Vega 的 Event Stream。
    *   Event Selectors (事件选择器) 的解析逻辑。
2.  **Signal Updates**
    *   事件处理函数 (Handlers) 如何更新 Signal 的值。
    *   Signal 的更新如何触发 Dataflow 的新一轮 Pulse。

## 第六阶段：高级机制与扩展 (Advanced Mechanisms)
**目标**：探索 Vega 的边界和扩展能力。

1.  **Expression Language (表达式语言)**
    *   Vega 内部的小型解释器，用于计算 Signal 表达式。
2.  **Transforms (变换)**
    *   深入研究几个核心 Transform 的实现 (如 Aggregate, Stack, Geo)。
    *   如何编写自定义 Transform。
3.  **View API**
    *   宿主环境 (Host Application) 如何通过 View API 与 Vega 实例通信 (Signal 注入, 数据监听)。

## 学习资源推荐
*   **Vega 源码库**: `vega/vega` (monorepo)，重点关注 `vega-dataflow`, `vega-scenegraph`, `vega-runtime`.
*   **Observable Notebooks**: 搜索 Mike Bostock 或 UW Interactive Data Lab 发布的关于 Vega 内部机制的讲解。
