# ProseMirror 概述

ProseMirror 是一个用于在 Web 上构建富文本编辑器的工具包。与传统的 WYSIWYG（所见即所得）编辑器不同，它优先考虑数据的结构化、一致性和可编程性。

## 1. 核心架构 (Core Modules)

ProseMirror 的设计遵循模块化原则，主要由以下四个核心模块组成：

- **`prosemirror-model`**: 定义编辑器的文档模型（数据结构）。
- **`prosemirror-state`**: 定义编辑器的整个状态（文档、选区、插件状态等）。
- **`prosemirror-view`**: 负责将状态渲染为 UI，并处理用户交互。
- **`prosemirror-transform`**: 定义对文档的修改操作（Steps），是事务（Transaction）的基础。

## 2. 数据模型 (Data Model) - "状态空间"

从数学角度看，ProseMirror 的核心是一个**不可变（Immutable）**的状态机。

### 文档树 (Document Tree)
文档不仅仅是 HTML 字符串，而是一棵由 `Node` 对象组成的树。
*   **Node**: 编辑器中的基本单元（段落、标题、文本节点等）。
*   **Fragment**: `Node` 的子节点列表。
*   **Schema**: 定义了这棵树的“语法规则”。它约束了哪些节点可以包含哪些子节点。

### 状态 (State)
整个编辑器的状态 $S$ 不仅仅包含文档 $D$，还包含选区 $Sel$ 和插件状态 $P$。
$$ S = \{ D, Sel, P \} $$

## 3. 状态流转 (State Transitions) - "计算过程"

ProseMirror 的运行机制可以抽象为一个状态转移函数。

$$ S_{new} = \text{apply}(S_{old}, T) $$

其中 $T$ 是 **Transaction (事务)**。

*   **Transaction**: 代表一次原子性的变更。它包含了一系列的 **Steps**（步骤）。
*   **Immutability**: $S_{old}$ 不会被修改，而是生成一个新的 $S_{new}$。这使得实现“撤销/重做”（Undo/Redo）和协同编辑变得非常自然（只需回溯或合并 Transaction）。

### 流程图

```mermaid
graph LR
    UserAction(用户操作) --> |触发| Dispatch
    Dispatch --> |创建| Transaction
    Transaction --> |应用 apply| OldState
    OldState --> |生成| NewState
    NewState --> |更新| View
    View --> |渲染| DOM
```

## 4. 视图 (View) - "输出投影"

`prosemirror-view` 组件不仅是渲染器，也是输入控制器。
*   **渲染**: 将当前的 $S$ 映射（Project）到浏览器的 DOM 树。
*   **交互**: 监听 DOM 事件，将其转化为 Transaction，然后分发（Dispatch）出去。

## 总结

ProseMirror 本质上是一个**基于 State-Action 模型的功能性响应式系统**。你定义数据结构（Schema）和转换规则（Plugins/Commands），框架负责维护状态的一致性和视图的同步。
