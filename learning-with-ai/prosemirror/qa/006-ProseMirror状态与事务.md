# ProseMirror 状态 (EditorState) 与事务 (Transaction) 详解

我们已经理解了文档的静态结构（`Schema`、`Node`、`Mark`），接下来我们关注其动态变化：**ProseMirror 如何管理编辑器的状态以及如何处理这些状态的变更。** 这将引出 `EditorState` 和 `Transaction` 这两个关键概念。

## 1. EditorState (编辑器状态)

### 什么是 EditorState？

`EditorState` 是 ProseMirror 编辑器在某一特定时间点的**完整、不可变的快照**。它包含了描述编辑器当前情况所需的所有信息。当用户进行任何操作（如打字、删除、格式化）时，都不会直接修改当前的 `EditorState`，而是会生成一个全新的 `EditorState`。

### EditorState 的构成：

一个 `EditorState` 对象通常包含以下核心部分：

*   **`doc`**: 当前编辑器的文档内容，即我们之前讨论的由 `Node` 组成的文档树。这是 `EditorState` 中最主要的部分。
*   **`selection`**: 表示用户当前在文档中的选区或光标位置。ProseMirror 提供了强大的 `Selection` API 来精确管理选区，它可以是文本选区、节点选区，甚至是更复杂的自定义选区。
*   **`storedMarks`**: 这是一个临时的标记集合，通常用于在用户输入文本时自动应用格式。例如，如果用户在加粗区域输入，`storedMarks` 可能包含 `strong` 标记，新输入的文本会自动被加粗。
*   **`schema`**: 构成 `doc` 和 `marks` 的 `Schema` 对象。每个 `EditorState` 都与其所基于的 `Schema` 绑定。
*   **`plugins` (插件状态)**: ProseMirror 的插件系统允许插件在 `EditorState` 中存储自己的数据。这意味着编辑器状态是高度可扩展的。

### 不可变性 (Immutability) 的重要性：

`EditorState` 的不可变性是 ProseMirror 设计的基石，它带来了巨大的优势：

*   **可预测性**: 每次状态变化都产生一个新对象，避免了副作用，使得编辑器的行为更加可预测。
*   **方便调试**: 可以轻松地“时间旅行”，回溯到任何一个历史状态进行调试。
*   **撤销/重做**: 自然支持强大的撤销/重做功能，因为旧状态始终可用。
*   **协作编辑**: 为协同编辑奠定了基础，因为可以基于已知的旧状态安全地应用和转换操作。

## 2. Transaction (事务)

### 什么是 Transaction？

`Transaction` 是在 ProseMirror 中**修改 `EditorState` 的唯一机制**。它是一个封装了所有变更的容器，描述了如何从一个 `EditorState` 转变到下一个 `EditorState`。

### Transaction 的构成和工作原理：

*   **不可变操作序列**: 一个 `Transaction` 描述了一系列对文档的变更。这些变更并非直接执行在 `doc` 上，而是通过生成中间的 `Transform` 对象来完成。
*   **步骤 (Steps)**: 每个 `Transaction` 包含一个或多个原子性的 `Step` 对象。每个 `Step` 都代表一个最小的、可逆的文档变更（例如：插入一段文本、删除一个范围、改变一个节点的属性）。
    *   **数学类比**: `Step` 可以被看作是**离散的、可组合的数学操作**，它们可以被反转 (`invert`) 和转换 (`map`)。
*   **元数据 (Metadata)**: `Transaction` 可以附带任意的元数据，这允许插件或外部逻辑为特定的操作添加上下文信息（例如：这个 Transaction 是由用户输入引起的，还是由粘贴操作引起的）。
*   **应用 Transaction**: 当一个 `Transaction` 被“应用”到当前的 `EditorState` 时，它会：
    1.  克隆当前的 `EditorState`。
    2.  根据 `Transaction` 中的 `Steps`，对克隆的 `doc` 进行转换，产生一个新的 `doc`。
    3.  根据 `Transaction` 中的其他信息（如选区变化），更新 `selection` 和 `storedMarks`。
    4.  处理插件对状态的更新。
    5.  最终返回一个新的 `EditorState` 对象。

### 状态转移公式回顾：

$$ S_{new} = \text{apply}(S_{old}, T) $$

*   $S_{old}$: 旧的 `EditorState`
*   $T$: `Transaction` 对象
*   `apply`: 一个纯函数，根据 `Transaction` 的内容计算并返回新的 `EditorState`。
*   $S_{new}$: 新的 `EditorState`

### 为什么是 Transaction 而不是直接修改？

Transaction 机制是 ProseMirror 实现复杂功能（如协作编辑、精确的撤销/重做）的关键：

*   **协作编辑**: 多个用户对同一文档的修改都可以通过 `Transaction` 来表示。ProseMirror 提供了 `StepMap` 和 `map` 函数来转换和合并来自不同用户的 `Transaction`，从而解决冲突。
*   **历史记录**: 每个 `Transaction` 都可以被记录下来，形成一个完整的操作历史，这使得撤销和重做变得非常可靠。
*   **一致性**: 确保了任何时候文档内容的变化都是通过一个定义良好、可控的路径进行的，保持了文档的结构和数据一致性。

### 3. 示例流程

1.  用户在编辑器中键入字符 "A"。
2.  `prosemirror-view` 捕获到键盘事件。
3.  它创建一个 `Transaction`，其中包含一个 `Step`，表示在当前光标位置插入字符 "A"。
4.  这个 `Transaction` 被 `dispatchTransaction` 方法处理。
5.  `dispatchTransaction` 调用 `EditorState.apply(currentEditorState, transaction)`。
6.  `apply` 方法基于 `currentEditorState` 和 `transaction` 计算出新的 `EditorState`。
7.  `prosemirror-view` 接收到新的 `EditorState`，并高效地更新 DOM 以反映字符 "A" 的出现。
