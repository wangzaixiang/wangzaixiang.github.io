# QA: Scenegraph 与 Mark 的关系

## 问题
Vega 中的 Scenegraph (场景图) 是否可以理解为：每个 Mark Definition 对应一个 Scenegraph？

## 回答
不完全准确。更精确的描述是：**Scenegraph 是一个包含所有可视元素的单一树状结构，而 Mark Definition 定义了这个树中的不同层级或节点集合。**

### 1. Scenegraph 的结构
Vega 的 Scenegraph 是一个**树 (Tree)**，类似于 DOM 树。
*   **Root**: 根节点（通常是一个 Group）。
*   **Nodes**: 树中的节点可以是：
    *   **Group**: 容器，可以包含子节点（Marks 或其他 Groups）。
    *   **Mark (Set)**: 一组同类型的几何元素（如“所有的柱子”）。
    *   **Item**: 单个几何元素（如“第一根柱子”）。

### 2. Mark Definition 与 Scenegraph 的映射
当你在 JSON Spec 中定义一个 Mark 时：

```json
// Spec
"marks": [
  {
    "type": "rect",
    "name": "bar_chart_layer",
    "from": {"data": "table"},
    "encode": { ... }
  }
]
```

它在 Scenegraph 中映射为一个 **Mark 对象**（可以理解为一个数组/集合容器）。
*   这个 Mark 对象包含了 N 个 **Item 对象**（如果 `table` 有 N 行数据）。
*   每个 Item 对象对应屏幕上的一个矩形。

### 3. 树状层级关系
如果你的 Spec 包含 Groups（嵌套）：

```json
// Spec
"marks": [
  {
    "type": "group", // 父级 Group
    "marks": [
      { "type": "rect", ... }, // 子级 Mark A
      { "type": "text", ... }  // 子级 Mark B
    ]
  }
]
```

对应的 Scenegraph 结构如下：
```text
Root (Scene)
└── Group Item (父容器)
    ├── Mark (类型: rect)
    │   ├── Item 1 (柱子 1)
    │   ├── Item 2 (柱子 2)
    │   └── ...
    └── Mark (类型: text)
        ├── Item 1 (标签 1)
        └── ...
```

### 4. 为什么这么设计？
这种**Mark Set (集合) -> Item (个体)** 的双层结构是为了性能：
1.  **Shared State**: 同一个 Mark Set 下的所有 Items 共享相同的 Scale、Transform 和部分属性配置。
2.  **Rendering**: 渲染器（Canvas/WebGL）可以对同一个 Mark Set 进行批量绘制优化（Draw Call Batching）。

### 总结
*   **One Scenegraph**: 整个可视化只有一个 Scenegraph。
*   **Mark Definition -> Mark Node**: JSON 中的一个 `mark` 定义，对应 Scenegraph 中的一个 **Mark 节点**（它是一个 Items 的容器）。
*   **Datum -> Item Node**: 数据源中的每一行，对应 Scenegraph 中的一个 **Item 节点**（具体的绘图单元）。
