# QA: 响应式连接的粒度与底层更新机制

## 问题
Vega 的响应式连接（Signal -> Expression 依赖）是点到点的吗？
*   数量级如何？
*   粒度是以 Mark 实例为单位，还是以属性（如 `mark.update.x`）为单位？

## 回答

这是一个非常核心的性能问题。简单的回答是：**粒度在 Operator（算子）级别，通常对应一个 Mark Definition 的一个属性集，而不是每个 Mark 实例。**

### 1. 依赖图的节点是 "Operator"
Vega 的 Dataflow Graph 并不包含每一个具体的“矩形”或“点”。它包含的是**生成**这些矩形的指令节点。

*   **Signal Node**: 对应 Spec 中的一个 Signal。
*   **Scale Node**: 对应 Spec 中的一个 Scale。
*   **Mark Node**: 对应 Spec 中的一个 Mark 定义（例如整个“柱状图”层）。

### 2. 连接的粒度：属性集 (Property Set)
当你写下：
```json
"marks": [
  {
    "type": "rect",
    "encode": {
      "update": {
        "x": {"scale": "xscale", "field": "category"},
        "width": {"signal": "barWidth"}, 
        "fill": {"signal": "colorSignal"}
      }
    }
  }
]
```

Vega 会为这个 Mark 创建一个 **Encoder Operator**。
*   **输入**：`Signal(barWidth)`, `Signal(colorSignal)`, `Scale(xscale)`, `Data(table)`.
*   **输出**：也就是 Mark 的 Scenegraph 节点。

**连接数量**：
*   不是 `Num_Bars * Num_Signals`。
*   而是 `Num_Mark_Definitions * Num_Signals`。
*   **是一对多的关系**：一个 Signal 节点连接到一个 Mark Encoder 节点。

### 3. 运行时的批量计算 (Batch Processing)
当 `barWidth` 信号变化时：
1.  Signal 节点更新值。
2.  变化传播到连接的 **Encoder Operator**。
3.  Encoder Operator 收到 Pulse。
4.  **关键点**：Encoder 会**遍历**所有受影响的数据项（Tuple），并重新计算它们的属性。

```javascript
// 伪代码：Encoder Operator 的内部逻辑
function evaluate(pulse) {
  // 即使有 10000 个柱子，依赖连接只有 1 条 (Signal -> Encoder)
  var currentBarWidth = signals.barWidth; 
  
  // 批量更新
  pulse.visit(pulse.MOD, function(tuple) {
     tuple.width = currentBarWidth; // 这里循环执行 10000 次
     tuple.fill = signals.colorSignal;
  });
}
```

### 4. 总结
*   **依赖图规模**：与 Spec 的复杂度（JSON 的大小）成正比，**不**与数据量（100万行数据）成正比。这保证了图本身的遍历非常快。
*   **计算规模**：当节点被触发执行时，内部循环次数与数据量成正比。
*   **粒度**：以 **Mark Definition** 的编码块（`update` set）为单位建立依赖边。

---

# 追问 1：响应式计算的开销与优化

## 追问
如果一个 Mark Definition 作为一个响应单元：
1.  所有的数据点 datum 都需要重新计算吗？
2.  每个重新计算需要计算所有的属性（包括没变的）吗？
3.  计算结果是替换原有 Items 集合，还是就地更新？

## 回答

这触及了 Vega 运行时优化的核心。简单说：Vega 极力避免“全部重算”和“全部替换”。

### 1. 只有受影响的 Tuple 会被重算（Partial Evaluation）
Vega 并不是盲目地遍历所有数据。它利用 **Pulse** 中的 `mod`（修改列表）来过滤。

*   **场景 A：Signal 变化（全局影响）**
    如果变化源是 `Signal(barWidth)`，它影响了所有柱子的宽度。
    *   **是的**，Encoder 必须遍历**所有**现存的 Items (`pulse.visit(pulse.SOURCE, ...)`), 因为所有柱子都得变宽。
    *   **优化**：这里无法避免 $O(N)$，因为确实 N 个图形都变了。

*   **场景 B：Data 变化（局部影响）**
    如果变化源是 Data `insert` 了一个新点，或者 `modify` 修改了 1 个点的数值。
    *   Pulse 中只有 1 个 Tuple 在 `add` 或 `mod` 列表里。
    *   Encoder **只会** 为这 1 个 Tuple 运行计算逻辑。
    *   其他的 N-1 个 Items **完全不被触碰**。

### 2. 只有受影响的 Channel 会被重算（Bitmasking）
Vega 的 Encoder 内部其实更加智能。虽然我们在 Spec 里写在一个 `update` 块里，但编译器会分析依赖。

*   假设 `update` 块：
    ```json
    "update": {
      "x": {"scale": "xscale", "field": "category"}, // 依赖 xscale
      "fill": {"signal": "colorSignal"}               // 依赖 colorSignal
    }
    ```
*   **如果 `colorSignal` 变了**：
    Vega 知道只有 `fill` 属性需要重算。`x` 属性如果不依赖 `colorSignal`，理论上可以跳过。
    *(注：在 v5 的实现中，通常会重新运行该 Item 的整个 update function，但因为 update function 是编译过的 JS 代码，执行极快。真正的开销在于后续的 Scenegraph 属性赋值和 Render 阶段的脏检查)*。

### 3. 就地更新 (In-place Update)
Vega **绝对不会**因为一次更新就销毁并重建所有 Items。
*   **Item 对象是稳定的**：只要数据项还在，其对应的 JS 对象就一直存在。
*   **增量更新**：只修改变动的属性，不重建对象，极大减轻了 GC 压力。

---

# 追问 2：底层更新机制：Signal -> Item

## 追问
1.  当 signal 变化时，是直接更新 Scenegraph 中的 Item 属性吗？
2.  Datum 和 Item 是一一对应的吗？
3.  这个对应关系存储在哪里？

## 回答

### 1. 更新流程：Signal -> Tuple -> Item
最终结果确实是**直接修改了 Scenegraph Item 对象的属性**。
Encoder Operator 遍历受影响的 Tuples，根据最新的 Signal 值计算出属性值，并直接赋值给对应的 Scenegraph Item 实例。

### 2. Datum 和 Item 的对应关系
*   **一对一关系**：对于大多数 Mark（rect, symbol, text），一个 Datum 严格对应一个 Item。
*   **聚合关系**：对于 line 和 area，整个数据集或分组对应一个 Item（Item 内部包含多个点的信息）。

### 3. 对应关系存储在哪里？
这是双向链接的：
*   **Item -> Datum**：每个 Item 对象都有一个 `datum` 属性，指向其对应的数据行对象。
*   **Datum -> Item**：数据对象本身不持有 Item 引用。这个映射关系由 **Mark Operator 内部维护**（通过一个从 Tuple ID 到 Item 实例的 Map）。当 Dataflow 发出更新指令时，Operator 通过这个内部 Map 快速定位到需要修改的 Item。