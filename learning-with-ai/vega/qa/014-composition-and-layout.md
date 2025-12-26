# 组合与布局 (Composition & Layout)

在 Vega 中，构建单一图表只是第一步。真正的威力在于如何将多个图表组合成复杂的仪表盘 (Dashboard) 或分面图 (Trellis Plot)。这主要依赖于核心原语：**Group Mark (`group`)**。

## 1. Group Mark: 不仅仅是容器

`group` 是 Vega 中最特殊的标记类型。它不仅可以包含其他标记，还定义了**嵌套的作用域 (Nested Scope)**。

### A. 作用域隔离 (Scoping)
每个 Group 都可以拥有自己独立的：
*   **Data**: 可以在 Group 内部定义局部数据集。
*   **Signals**: 局部信号只在 Group 内部可见。
*   **Scales**: 每个子图可以有独立的坐标轴范围。

这使得你可以定义一个“图表模板”，然后实例化多次，每次使用不同的数据，而互不干扰。

### B. 局部坐标系 (Local Coordinate System)
Group 建立了一个新的笛卡尔坐标系。
*   Group 内部标记的 `x: 0, y: 0` 是相对于 Group 的左上角的，而不是画布的左上角。
*   移动 Group 会带着它里面的所有东西一起移动。

## 2. 分面 (Faceting) - 数据驱动的图表生成

分面（Faceting，即根据数据字段将图表拆分为多个子图）是探索性数据分析的核心。Vega 的分面机制非常底层且强大。

它不是像 ggplot2 那样简单的 `facet_wrap` 指令，而是一种 **Dataflow 的分叉机制**。

```json
{
  "type": "group",
  "from": {
    "facet": {
      "name": "faceted_data",
      "data": "table",
      "groupby": "category"
    }
  },
  "encode": { ... }, // 设置每个子图的位置/大小
  "marks": [ ... ]   // 定义子图内部的图表结构
}
```

**原理：**
1.  **Partition**: 主数据流 (`table`) 根据 `category` 字段被切分成多个子数据流。
2.  **Spawn**: 对于每一个子数据流，Vega 创建一个 Group 实例。
3.  **Subflow**: 每个 Group 实例内部运行一套独立的 Dataflow（包含 Scales 和 Marks），处理该组的数据。

## 3. 布局 (Layout) - 自动排版

如果你有 4 个子图，你怎么排列它们？手动算 `x`, `y` 坐标太痛苦了。Vega 提供了 `layout` 属性来自动管理 Group 的位置。

### Grid Layout (网格布局)
```json
"layout": {
  "columns": 2,
  "padding": 10
},
"marks": [
  { "type": "group", ... }, // 子图 1
  { "type": "group", ... }, // 子图 2
  ...
]
```
Vega 的布局引擎会自动计算每个子 Group 的边界框 (Bounds)，然后像 HTML 表格一样把它们排列整齐，自动处理对齐和间距。

## 4. 图层叠加 (Layering)

如何在同一个坐标系下同时画柱状图和折线图？
*   **共享 Scales**: 在父级 Group 定义 `xscale` 和 `yscale`。
*   **引用 Scales**: 两个子 Marks（一个 `rect`，一个 `line`）都引用父级的 Scales。
*   **绘制顺序**: `marks` 数组的顺序决定了绘制顺序（Z-index）。后定义的盖在上面。

## 5. 总结：递归的视图合成

Vega 的视图合成模型是**递归 (Recursive)** 的：
*   一个 Visualization 是一个 Group。
*   这个 Group 可以包含 Marks，也可以包含子 Groups。
*   子 Groups 又可以包含孙 Groups。

这种无限嵌套的能力，配合 Dataflow 的独立作用域，使得 Vega 可以描述极其复杂的可视化系统（例如：一个散点图矩阵，其中每个散点图内部又是一个由小型饼图组成的矩阵）。

---

# QA: Faceting 的类型与机制

## 追问
除了 `groupby` facet，Vega 还支持其他类型的 facet 吗？

## 回答

**是的，Vega 支持两种核心的 Facet 模式：**

### 1. Pre-faceted Data (预分面数据)
如果你的数据源**已经**是嵌套结构（例如 JSON 数组中套数组），你就不需要 `groupby` 了，直接迭代即可。

**数据结构示例：**
```json
[
  {"series": "A", "values": [1, 2, 3]},
  {"series": "B", "values": [4, 5, 6]}
]
```

**Spec 写法：**
```json
"from": {
  "facet": {
    "name": "subdata",
    "data": "table",
    "field": "values" // 直接指定包含子数组的字段
  }
}
```
这种模式下，Vega 不需要执行昂贵的“分组”操作，直接遍历外层数组，为每一行创建一个 Group，并将 `values` 字段的内容作为该 Group 的数据源。

### 2. Aggregate Facet (聚合分面 - 即 groupby)
这是最常用的模式，用于处理扁平数据（Flat Table）。
```json
"from": {
  "facet": {
    "name": "subdata",
    "data": "table",
    "groupby": ["category", "year"], // 可以按多个字段分组
    "aggregate": { ... } // 可选：先做聚合统计
  }
}
```
在这种模式下，Vega 运行时会先对数据流进行 Hash 或 Sort 操作，将数据切分成块 (Partitions)，然后分发给子 Group。

### 3. Cross-Product / Matrix (虽无直接 API 但可组合)
Vega 没有像 ggplot2 那样显式的 `facet_grid` (行列交叉) 原语。
但是，你可以通过**嵌套两个 Group** 来实现：
1.  外层 Group 按 `RowField` 分面。
2.  内层 Group 按 `ColField` 分面。
或者，使用 `layout` 属性配合单一的扁平 Facet，让 Vega 自动换行排列。

### 4. 总结
虽然 API 看起来只有 `facet` 这一种 transform，但通过 `field` (处理嵌套数据) 和 `groupby` (处理扁平数据) 这两个参数的组合，它涵盖了几乎所有的分面需求。

---

# QA: Layout 的作用对象

## 追问
示例中，`layout` 只是对 `marks.[type='group']` 的子图生效吗？它能布局普通的 `rect` 或 `symbol` 吗？

## 回答

这是一个非常敏锐的观察。

**答案是：是的，Vega 的 `layout` 主要是为 `group` 标记设计的。**

### 1. 为什么 Layout 针对 Groups？
Vega 的布局引擎（Grid, Flow）是基于 **Bounding Box (边界框)** 进行排列的。
*   **Group**: 作为一个容器，它很容易定义一个矩形的“盒子”（包含内部所有的轴、图例、标记）。
*   **普通 Mark (Rect, Symbol)**: 它们的位置通常是由 **Scale** (比例尺) 精确控制的（例如：`x: scale(datum.field)`）。

如果你试图用 `layout` 去排列一堆散点 (`symbol`)，这通常没有意义，因为散点的位置应该反映数据值，而不是为了“整齐好看”而被强行塞进格子里。

### 2. 特例：Legend 和 Axis 的布局
虽然我们主要用 layout 排列图表 (Groups)，但 Vega 内部也使用类似的布局逻辑来排列 **Guide (向导)** 元素：
*   **Legends**: 当你有多个图例时，Vega 会自动把它们堆叠排列。
*   **Axes**: 轴的标题和标签布局也使用了约束求解器。

### 3. 如果我想排列普通 Marks 怎么办？
如果你真的想把一堆 `rect` 排成网格（例如：日历热力图，或者 GitHub 的贡献图），你不应该使用 `layout` 属性。
你应该使用 **Scale** 和 **Transform**：
*   **方法**：计算每个数据点的 `row` 和 `col` 索引。
*   **Scale**: 定义 `x = col * cellSize`, `y = row * cellSize`。

### 总结
*   **Layout 属性**: 用于 **View Composition (视图合成)**。它解决的是“如何把多个独立的图表拼在一起”的问题。
*   **Scale & Encode**: 用于 **Visual Encoding (视觉编码)**。它解决的是“如何根据数据决定元素位置”的问题。

不要混淆这两个层级。
