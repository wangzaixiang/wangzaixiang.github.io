# 事件流与信号系统 (Event Streams & Signals)

在 Vega 的设计哲学中，交互的核心是 **Event Streams (事件流)** 和 **Signals (信号)**。这一节将深入探讨它们是如何协同工作的。

## 1. 信号 (Signals): 状态的容器

Signal 本质上是 Reactive Graph 中的一个**可变变量**。

### 定义
```json
{
  "name": "activePoints",
  "value": [],
  "on": [
    {
      "events": "rect:mouseover", 
      "update": "datum" 
    },
    {
      "events": "rect:mouseout", 
      "update": "null" 
    }
  ]
}
```

### 属性解析
*   `value`: 初始值。
*   `on`: 更新规则列表。这是命令式逻辑进入声明式系统的入口。
    *   `events`: 触发源。
    *   `update`: 更新逻辑（Expression）。

## 2. 事件流 (Event Streams): 交互的源头

Vega 的事件流系统非常强大，它不仅仅是简单的 `click` 监听。

### A. 事件选择器 (Event Selectors)
类似于 CSS 选择器，Vega 允许你通过字符串精确描述你关心的事件。

语法结构：`[Source:]Type[Filter][Throttle/Debounce]`

*   **View-level events**: `"mousedown"`, `"mousemove"`
*   **Mark-level events**: `"rect:click"`, `"@myMarkName:mouseover"`
*   **Keyboard events**: `"keydown[key='Shift']"`

### B. 事件合成与过滤 (Composition & Filter)
你可以在事件流定义中直接过滤不需要的事件。

```json
"events": "mousemove![event.shiftKey]" // 只监听没按 Shift 时的移动
"events": {"source": "window", "type": "resize"}
```

### C. 节流与防抖 (Throttle & Debounce)
对于 `mousemove` 或 `wheel` 这样高频触发的事件，Vega 内置了性能优化修饰符。

```json
"events": "mousemove{100}" // Throttle: 每 100ms 触发一次
"events": "input{500}"     // Debounce: 停下来 500ms 后才触发
```

## 3. 信号更新机制 (Signal Update Mechanism)

当一个 Event 发生时，Vega 运行时会执行以下步骤：

1.  **捕获**: 浏览器捕获 DOM 事件。
2.  **解析**: Vega Event Handler 解析事件选择器，确定哪个 Signal 关注这个事件。
3.  **求值**: 运行 Signal 定义中的 `update` 表达式。此时 `event` 和 `datum` 变量是可用的。
4.  **推送**: Signal 的值更新后，它会产生一个 **Pulse**，沿着 Dataflow Graph 向下传播（到 Scale, 到 Mark）。

## 4. 高级模式：Derived Signals (派生信号)

信号可以依赖其他信号，形成依赖链。

```json
// 信号 A
{"name": "width", "value": 500}

// 信号 B: 永远是宽度的一半
{"name": "midPoint", "update": "width / 2"}
```
当 `width` 改变时，`midPoint` 会自动重新计算。这就是响应式编程的真谛。

## 5. 绑定 (Binding): 双向数据流

Signal 还可以绑定到 HTML 输入控件 (Range, Checkbox, Select)。

```json
{
  "name": "opacity",
  "value": 0.5,
  "bind": {"input": "range", "min": 0, "max": 1, "step": 0.01}
}
```
*   **HTML -> Vega**: 拖动滑块 -> 更新 Signal -> 重绘图表。
*   **Vega -> HTML**: 内部逻辑更新 Signal -> 滑块位置自动更新。
这是构建可视化 Dashboard 的神器。