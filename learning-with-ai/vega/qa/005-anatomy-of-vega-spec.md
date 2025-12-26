# 解剖 Vega 规范 (Anatomy of a Specification)

要理解 Vega 的模型，最好的方法是看一个“最小可行规范”。下面是一个简单的柱状图 Spec，它展示了 Vega 语法的骨架。

## 1. 最小柱状图 Spec

```json
{
  "$schema": "https://vega.github.io/schema/vega/v5.json",
  "width": 400,
  "height": 200,
  "padding": 5,

  "data": [
    {
      "name": "table",
      "values": [
        {"category": "A", "amount": 28},
        {"category": "B", "amount": 55},
        {"category": "C", "amount": 43}
      ]
    }
  ],

  "signals": [
    {
      "name": "hover",
      "value": null,
      "on": [
        {"events": "rect:mouseover", "update": "datum"},
        {"events": "rect:mouseout", "update": "null"}
      ]
    }
  ],

  "scales": [
    {
      "name": "xscale",
      "type": "band",
      "domain": {"data": "table", "field": "category"},
      "range": "width",
      "padding": 0.05
    },
    {
      "name": "yscale",
      "domain": {"data": "table", "field": "amount"},
      "range": "height",
      "nice": true
    }
  ],

  "axes": [
    {"orient": "bottom", "scale": "xscale"},
    {"orient": "left", "scale": "yscale"}
  ],

  "marks": [
    {
      "type": "rect",
      "from": {"data": "table"},
      "encode": {
        "enter": {
          "x": {"scale": "xscale", "field": "category"},
          "width": {"scale": "xscale", "band": 1},
          "y": {"scale": "yscale", "field": "amount"},
          "y2": {"scale": "yscale", "value": 0}
        },
        "update": {
          "fill": {"signal": "hover === datum ? 'red' : 'steelblue'"}
        }
      }
    }
  ]
}
```

## 2. 核心组件解析

### A. Data (数据层)
*   **作用**：定义数据源。
*   **模型**：Vega 将数据视为**命名的关系表**。
*   **引用**：后续所有组件通过 `"data": "table"` 来引用它。

### B. Signals (信号层 - 交互的核心)
*   **作用**：定义随时间或事件变化的变量。
*   **本例**：`hover` 信号捕获鼠标悬停在 `rect` 上的数据项。
*   **重要性**：这是 Vega 区别于传统 GoG 的地方，它让 Spec 具备了“状态”。

### C. Scales (比例尺层 - 翻译逻辑)
*   **作用**：将数据定义域 (`domain`) 映射到视觉值域 (`range`)。
*   **解耦**：Notice！Scale 不知道自己会被用在哪里。它只是一个数学映射函数。
*   **引用关系**：`domain` 引用了 `data`。

### D. Axes & Legends (向导层)
*   **作用**：可视化 Scale。
*   **逻辑**：Axis 必须绑定一个 Scale (`"scale": "xscale"`)。如果没有轴，Scale 依然工作，只是你看不到刻度。

### E. Marks (图形层 - 最终输出)
*   **作用**：定义要画什么几何体。
*   **`from`**: 指定数据源。每一个数据行对应一个 Mark 实例。
*   **`encode`**: **这是最关键的部分**。
    *   它定义了 Mark 的属性（x, y, fill 等）如何计算。
    *   **计算链**：`Attribute = Scale(Data.Field)`。
    *   **三位一体**：这里将 **Data**、**Scale** 和 **Signal** 编织在了一起。

## 3. 它是如何“编织”成网的？

理解 Vega 的关键在于看它的**引用链 (Reference Chain)**：

1.  **Marks** 说：我要用 `table` 的数据。
2.  **Marks** 说：我的 `x` 坐标要请 `xscale` 算一下，输入值是 `table` 里的 `category`。
3.  **xscale** 说：我的 `domain` 来自 `table` 的 `category`。
4.  **Axes** 说：我也用 `xscale` 来画底下的刻度。

**结论**：这个 JSON 不是简单的配置列表，它实际上描述了一张 **有向无环图 (DAG)**。当 `data` 改变时，Vega 会沿着这张图，自动更新受影响的 `scales` 和 `marks`。

## 4. 一个特殊的概念：Encoders 的生命周期

注意到 `marks` 里的 `enter` 和 `update` 吗？
*   **`enter`**: 当数据项第一次出现时运行（性能优化）。
*   **`update`**: 每次数据或信号更新时运行。
*   **`exit`**: 当数据项消失时运行。

这与 D3 的 `enter/update/exit` 模式高度契合，但 Vega 将其**声明式**化了。
