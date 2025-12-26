# QA: Vega 数据的增量更新与响应式机制

## 问题
如何理解数据变更带来的响应式变化？能否提供一个示例，演示在初始化 JSON 后，如何对 Data 进行增量操作（增、删、改），以及这些操作是否真的是增量的？

## 回答

Vega 的运行时（View API）提供了精确控制数据流的方法。**是的，这些操作都是完全增量（Incremental）的**。Vega 不会重新解析整个数据集，而是追踪具体变动的 Tuple（元组/数据行）。

### 1. 核心概念：Pulse (脉冲) 与 Tuple ID
在 Vega 内部，每一行数据（对象）被摄入时，都会被分配一个唯一的内部 ID (`_id`)。
当数据发生变化时，Vega 会生成一个 **Pulse** 对象，其中包含三个列表：
*   `add`: 新增的元组列表。
*   `rem`: 移除的元组列表。
*   `mod`: 被修改的元组列表。

Dataflow Engine 只会处理这三个列表中的数据，从而实现 $O(\Delta N)$ 的更新效率，而不是 $O(N)$。

### 2. 实战示例：通过 View API 操作数据

假设我们要操作名为 `"table"` 的数据源。

#### A. 初始化 View
```javascript
const view = new vega.View(vega.parse(spec), {
  renderer:  'canvas', 
  container: '#view',
  hover:     true
});
await view.runAsync(); // 首次渲染
```

#### B. 增加一个数据点 (Add)
使用 `insert` 方法。
```javascript
view.change('table', vega.changeset().insert([
  {"category": "D", "amount": 99}
])).run();
```
*   **响应行为**：
    1.  新的 Tuple 被创建。
    2.  `xscale` 和 `yscale` 的 `domain` 可能会根据新值自动扩展（如果未固定）。
    3.  一个新的 `rect` Mark 实例被创建，执行 `enter` -> `update`。
    4.  其他的 Marks **不会**重绘（除非 Scale 发生了变化影响了全局布局）。

#### C. 删除一个数据点 (Remove)
使用 `remove` 方法。你需要提供一个**匹配函数 (Predicate)**。
```javascript
view.change('table', vega.changeset().remove(
  datum => datum.category === 'A' // 删除 category 为 'A' 的行
)).run();
```
*   **响应行为**：
    1.  找到匹配的 Tuple。
    2.  对应的 `rect` Mark 执行 `exit` 编码集（例如淡出动画）。
    3.  Mark 被从场景图中移除。

#### D. 更新某个数据点 (Modify)
使用 `modify` 方法。这比删除再插入更高效，因为它可以保持对象引用的稳定性（对于动画过渡至关重要）。

**场景 1：直接根据条件修改值**
```javascript
// 将 category 为 'B' 的 amount 改为 100
view.change('table', vega.changeset().modify(
  datum => datum.category === 'B', // 匹配条件
  "amount",                        // 要修改的字段
  100                              // 新值
)).run();
```

**场景 2：使用函数更新值**
```javascript
// 将所有 amount 小于 50 的项翻倍
view.change('table', vega.changeset().modify(
  datum => datum.amount < 50,
  "amount",
  datum => datum.amount * 2
)).run();
```

*   **响应行为**：
    1.  被修改的 Tuple 被标记为 `mod`。
    2.  对应的 Mark 实例执行 `update` 编码集。
    3.  **注意**：`enter` 不会执行。

### 3. "Insert" vs "Add" 的区别？
在 Vega View API 中，通常使用 `insert` 来添加数据。没有专门的 `add` 术语，但在底层 Pulse 语义中称为 `add`。从用户角度看，`insert` 就是追加。

### 4. 为什么这很重要？
想象你在做一个**实时股票大屏**，每秒钟有 1000 个新 tick 推送进来，而屏幕上已经有 100 万个历史点。
*   **非增量**：每秒重绘 100 万 + 1000 个点 -> 浏览器卡死。
*   **Vega 增量**：每秒只处理 1000 个 `insert`。
    *   1000 个新 Mark 执行 `enter`。
    *   Scale 根据新值微调 Domain。
    *   旧的 100 万个点**完全不动**（只要 Scale 没发生剧烈变化导致它们的位置需要重算）。

这就是 Vega **Reactive Dataflow** 的威力所在。
