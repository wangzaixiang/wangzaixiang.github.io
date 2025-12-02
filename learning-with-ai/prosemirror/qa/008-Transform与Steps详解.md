# Transform 与 Steps 详解：文档变更的微积分

如果说 `Transaction` 是包含业务逻辑（如选区更新、元数据）的高层容器，那么 `Transform` 就是专注于文档内容变更的核心引擎。这是 ProseMirror 实现协同编辑（Collab）和撤销重做（Undo/Redo）的数学基础。

## 1. Step (步骤) - 原子变更

`Step` 是 ProseMirror 中对文档修改的最小原子单位。任何复杂的文档编辑操作，最终都会被分解为一系列基本的 `Step`。

ProseMirror 内置了几种核心的 Step 类型：

*   **`ReplaceStep`**: 最通用的步骤。它将文档中指定范围 `[from, to)` 的内容替换为新的内容（`Slice`）。
    *   **插入**: `from` 等于 `to`，替换内容非空。
    *   **删除**: `from` 小于 `to`，替换内容为空。
    *   **替换**: `from` 小于 `to`，替换内容非空。
*   **`AddMarkStep`**: 在指定范围 `[from, to)` 的文本上添加一个 `Mark`。
*   **`RemoveMarkStep`**: 在指定范围 `[from, to)` 的文本上移除一个 `Mark`。
*   **`ReplaceAroundStep`**: 这是一个更高级的步骤，用于在不改变内部内容的情况下替换父节点（例如，解包一个列表项，保留其内容的段落）。

### Step 的特性：

*   **可逆 (Invertible)**: 每个 Step 都可以生成一个“逆向 Step”。应用逆向 Step 可以完全撤销原 Step 的效果。这是实现 Undo 的基础。
*   **可映射 (Mappable)**: 这是最关键的特性。如果在文档开头插入了一个字符，文档后面的所有位置索引都会发生偏移。Step 及其位置信息可以通过 `Mapping` 进行调整，以适应这些位置变化。这是协同编辑解决冲突的核心。

## 2. Transform (转换) - 步骤的构建者

`Transform` 对象是对 `Step` 序列的封装。它提供了一套更高级、更易用的 API 来生成和累积 `Step`。

开发者很少直接手动创建 `Step` 对象，而是调用 `Transform` 的方法：

*   `tr.replace(from, to, slice)`
*   `tr.delete(from, to)`
*   `tr.insert(pos, node)`
*   `tr.addMark(from, to, mark)`

当你调用这些方法时，`Transform` 会自动计算出所需的 `Step` 并将其添加到内部的步骤列表中。`Transaction` 类继承自 `Transform`，所以你在 `Transaction` 上调用的也是这些方法。

## 3. Mapping (映射) - 坐标变换系统

当文档发生变化时，旧的位置索引（Positions）在新的文档中可能不再有效或指向错误的位置。ProseMirror 必须精确地跟踪这些位置变化。

*   **问题**: 假设你在位置 5 插入了 3 个字符。原来的位置 10 现在应该变成 13。如果你有一个指向位置 10 的书签，插入后它应该指向哪里？
*   **解决方案**: ProseMirror 使用 `StepMap` 来描述每个 Step 对位置空间的影响。通过 `Mapping` 对象（包含一系列 `StepMap`），我们可以将旧位置映射（Map）到新位置。

### 位置映射公式:
$$ Pos_{new} = \text{Mapping}.map(Pos_{old}) $$

这对于保持选区（Selection）和插件装饰（Decorations）的正确位置至关重要。

## 4. 为什么这么设计？

这种基于 Step 的设计不仅仅是为了修改文档，更是为了**分布式系统**的需求：

1.  **协同编辑**: 就像 Git 里的 diff 和 merge。当 User A 和 User B 同时修改文档，ProseMirror 不会合并最终的 HTML 字符串，而是交换他们产生的 `Steps`。通过 `Rebase` (变基) 算法，将 User B 的 Steps 转换（Map）到 User A 的修改之上。
2.  **历史记录**: 撤销栈不需要存储整个文档的深拷贝。它只需要存储一串逆向的 Steps。这极大地节省了内存。

## 总结

*   **Step**: 文档修改的原子指令（Replace, AddMark...）。
*   **Transform**: 用于生成和累积 Step 的构建器。
*   **Mapping**: 处理位置偏移的坐标变换系统。
