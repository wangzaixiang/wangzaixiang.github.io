# Reactive Vega: 核心设计哲学 (Core Design Philosophy)

理解 Vega 实现机制的最佳途径是阅读其核心论文 [Reactive Vega (2014)](https://idl.cs.washington.edu/papers/reactive-vega/)。该论文解决了可视化领域的一个重大难题：**如何用声明式语言优雅地表达复杂的交互？**

## 1. 核心矛盾：声明式 vs. 交互

*   **声明式语法 (如 GoG/ggplot2)**：擅长描述“是什么”（What it is），例如“X轴代表时间，颜色代表类别”。但它对“点击后变大”这种动态行为描述乏力。
*   **命令式编程 (如 D3/JS)**：擅长描述交互细节（How it changes），但代码逻辑零散，难以维护和移植。

**Vega 的答案是：将交互逻辑建模为数据流 (Dataflow)。**

## 2. 三位一体：一切皆为流 (Everything is a Stream)

在 Reactive Vega 中，三种看似完全不同的东西被统一到了同一个模型下：

1.  **Data (数据流)**：从 CSV/JSON 摄入的数据行。
2.  **Signals (信号流)**：随时间变化的变量（由表达式或事件驱动）。
3.  **Events (事件流)**：来自浏览器的原始输入（click, mousemove）。

**哲学意义**：交互不再是“修改图形的脚本”，而是“流入系统并驱动图表更新的数据”。

## 3. 反应式数据流图 (Reactive Dataflow Graph)

Vega 内部维护了一张 **有向无环图 (Directed Acyclic Graph, DAG)**。每个节点都是一个 **Operator (算子)**。

### A. 拓扑排序 (Topological Sort)
这是保证可视化正确性的关键。
*   **问题**：如果 Signal A 依赖 Signal B，而 Scale 依赖 Signal A 和 B。如果不按顺序更新，可能会出现“一半是旧数据，一半是新数据”的闪烁现象（GLITCH）。
*   **解决**：Vega 每次更新前都会对图进行拓扑排序，确保所有上游依赖都计算完毕后，才计算下游。

### B. 脉冲传播 (Pulse Propagation)
当某个源头（Data 或 Signal）发生变化时，它会向外发送一个 **Pulse**。
*   Pulse 就像是图中的一封信。
*   它不仅告诉下游“我有变化”，还精确描述了变化的内容（`add`, `rem`, `mod`）。
*   **按需计算 (Lazy Evaluation)**：如果一个节点发现自己的输入 Pulse 没有实质性变化，它会拒绝执行并停止传播，从而节省性能。

## 4. 动态图重写 (Dynamic Graph Rewriting)

这是 Vega 论文中最深奥但也最强大的部分。
*   **场景**：当你在做分面图 (Faceting) 时，数据的每一组都需要一套独立的 Scale 和 Mark。
*   **挑战**：在编译 Spec 时，我们不知道会有多少组数据。
*   **解决**：Vega 允许在运行时动态地向 Dataflow 图中添加或删除子图（Sub-flows）。这就像是一个可以自我生长的程序结构。

## 5. 总结

Reactive Vega 的哲学可以概括为：**用数据流图的稳定性，去对抗交互逻辑的复杂性。**

通过将所有的依赖关系显式化，Vega 实现了：
1.  **自动的一致性**：你只需改变 Signal，剩下的（Scale 重算、Mark 重绘）全部由 DAG 自动推导。
2.  **极高的性能**：通过增量更新和依赖剪枝，它能以 60fps 处理复杂的交互。

---

# QA: 追问与深度探讨
*(此处预留给后续追问)*
