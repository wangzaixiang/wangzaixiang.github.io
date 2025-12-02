# Schema 实例与 Content 语法详解

## 1. Schema 对应的 Document 实例

ProseMirror 的文档在运行时是一个 Node 树对象，但在序列化（例如存储到数据库）时，通常表现为 JSON 格式。

基于我们在 `004` 中定义的 `mySchema`，一个合法的文档 JSON 结构如下：

```json
{
  "type": "doc",
  "content": [
    {
      "type": "heading",
      "attrs": { "level": 1 },
      "content": [
        {
          "type": "text",
          "text": "Hello ProseMirror"
        }
      ]
    },
    {
      "type": "paragraph",
      "content": [
        {
          "type": "text",
          "text": "这是一个 "
        },
        {
          "type": "text",
          "text": "加粗",
          "marks": [
            { "type": "strong" }
          ]
        },
        {
          "type": "text",
          "text": " 的文本示例，以及一个 "
        },
        {
          "type": "text",
          "text": "链接",
          "marks": [
            { "type": "link", "attrs": { "href": "https://prosemirror.net" } },
            { "type": "em" }
          ]
        },
        {
          "type": "text",
          "text": "。"
        }
      ]
    }
  ]
}
```

### 观察要点：
*   **根节点**: 必须是 `doc`。
*   **类型定义**: 除了根节点 `doc` 是 ProseMirror 强制要求的以外，示例中出现的 `heading`, `paragraph`, `text`, `strong`, `link`, `em` 等都是由开发者在 Schema 中自定义的节点或标记类型。ProseMirror 提供的是定义这些类型的机制，而非预设了大量内置类型。
*   **嵌套结构**: `doc` 包含 `content` 数组，数组中是 `block` 类型的节点（如 `heading`, `paragraph`）。
*   **Mark 的位置**: 注意 `marks` 是附着在 `text` 节点上的属性，而不是包裹文本的父节点。这验证了我们之前说的“Mark 是扁平的”。
*   **Attrs**: `heading` 的 `level` 和 `link` 的 `href` 都存储在 `attrs` 对象中。

## 2. Content 表达式 (Content Expressions) 详解

Schema 中的 `content` 属性定义了子节点的**匹配规则**。它的语法非常强大，本质上就是作用于节点类型的**正则表达式**或 **BNF (巴科斯范式)** 变体。

### 基础语法

| 符号 | 含义 | 示例 | 解释 |
| :--- | :--- | :--- | :--- |
| **Node Name** | 具体的节点类型名称 | `"paragraph"` | 必须是一个 paragraph 节点 |
| **Group Name** | 节点组名称 | `"block"` | 任何属于 block 组的节点 |
| **Space** | 序列 (Sequence) | `"heading paragraph"` | 先是一个标题，紧接着是一个段落 |
| **`|`** | 选择 (Choice) | `"paragraph | heading"` | 一个段落 **或者** 一个标题 |

### 量词 (Quantifiers) - 与正则一致

| 符号 | 含义 | 示例 | 解释 |
| :--- | :--- | :--- | :--- |
| **`*`** | 0 次或多次 | `"inline*"` | 空内容，或者多个行内节点 |
| **`+`** | 1 次或多次 | `"block+""` | 至少包含一个块级节点 |
| **`?`** | 0 次或 1 次 | `"image?"` | 可选的图片 |
| **`{n, m}`** | 次数范围 | `"paragraph{2, 5}"` | 2 到 5 个段落 |

### 复杂示例解析

1.  **`doc: "block+"`**
    *   文档必须包含至少一个块级元素。空文档是不允许的（除非定义为 `block*`）。

2.  **`paragraph: "inline*"`**
    *   段落可以包含任意数量的行内元素（文本、图片等），也可以是空的。

3.  **`list_item: "paragraph block*"`**
    *   列表项必须以一个段落开头，后面可以跟任意数量的其他块级元素（例如嵌套的列表或代码块）。这意味着列表项不能为空。

4.  **`figure: "image caption?"`**
    *   一个 `figure` 节点必须包含一张 `image`，后面可以**可选地**跟一个 `caption` 节点。

5.  **`(paragraph | heading)+`**
    *   这是一个更复杂的序列，表示由段落或标题组成的列表，且至少有一个。

### 表达能力总结

这套语法赋予了 Schema 极强的表达能力，类似于 **上下文无关文法 (Context-Free Grammar)** 的产生式规则。它不仅能定义简单的“包含关系”，还能定义严格的**顺序**和**数量**约束。

这确保了：
*   **结构正确性**: 比如，你可以强制要求 `doc` 必须以 `heading` 开头（`content: "heading block*"`）。
*   **业务逻辑约束**: 比如，限制一个特定的卡片组件只能包含一张图片和一段描述。

## 3. 常见疑问：`inline` vs `inline*`

**问：如果定义 `heading.content: "inline"`，是否意味着标题下只能放单个 Text 节点，不能部分加粗、部分斜体了？**

**答：是的，理解非常准确。**

这是一个非常关键的细节。在 ProseMirror 中，一段包含混合样式（如“**Hello** *World*”）的文本，在底层数据结构上并非一个单一的 Text 节点，而是**多个相邻的 Text 节点**组成的序列。

例如，“**Hello** *World*” 的节点结构其实是：
1.  `TextNode("Hello", marks=["strong"])`
2.  `TextNode(" ", marks=[])`
3.  `TextNode("World", marks=["em"])`

这实际上是 **3 个** 不同的节点。

*   **`content: "inline"`**: 规定该节点只能包含**确切的一个**行内节点。因此，它只能容纳纯文本（且中间不能有样式变化）或单个原子节点（如一个 inline image）。一旦文本中出现样式切换，就会被拆分为多个节点，从而违反 "inline" 的数量限制。
*   **`content: "inline*"`** (或 `"inline+"`): 规定该节点可以包含**任意数量**的行内节点。这才是支持富文本（混合样式）的正确写法。

**问：`inline` 是关键字吗？我们不能定义一个叫做 `inline` 的 node 吧？**

**答：`inline` 不是编程语言层面的保留字，但它在 ProseMirror 中是一个约定俗成的特殊“组名”（Group Name）。**

1.  **它不是关键字**：在 ProseMirror 的解析器中，并没有硬性规定 `inline` 不能作为节点名。从技术上讲，你确实可以在 schema 中定义一个名为 `inline` 的节点（例如 `nodes: { inline: { ... } }`）。
2.  **它是约定俗成的“组名”**：`inline` 本身是一个**组名（Group Name）**。在 ProseMirror 中，要让一个节点（如 `text` 节点、`image` 节点等）能够被 `content` 表达式中的 `"inline*"` 匹配，该节点必须在其 `NodeSpec` 中通过 `group: "inline"` 明确地加入到这个组中。`inline: true` 属性仅仅表示该节点是行内节点，并不会自动将其加入 `inline` 组。

    **补充说明**：在 `004` 文档的示例中，`text: { inline: true }` 的定义是概念性的，省略了 `group: "inline"`。但在实际应用中，为了让 `text` 节点能被 `"inline*"` 这样的内容表达式匹配，通常需要显式声明 `group: "inline"`（如 `text: { inline: true, group: "inline" }`）。许多基于 `prosemirror-schema-basic` 的默认 schema 会为 `text` 节点自动或隐式地加上此组。
3.  **极度不推荐重名**：如果你定义了一个名为 `inline` 的节点，那么当你在 content 表达式中写 `"inline"` 时，ProseMirror 会优先匹配节点类型还是组名可能会产生混淆（通常节点名优先级更高）。这会导致你无法方便地引用“所有行内元素”这个组，从而破坏了 Schema 的清晰度和扩展性。

**最佳实践**：
*   保留 `inline` 作为组名，用于代表 `text`, `image`, `hard_break` 等所有行内元素。
*   不要将具体的节点命名为 `inline`。

**问：文档中的 Text 节点可以包括换行符吗？**

**答：通常情况下，ProseMirror 的 `text` 节点**不应**包含表示段落或块级分割的换行符 (`\n`)。**

ProseMirror 的文档结构是基于节点的树形结构，严格区分块级节点（如 `paragraph`, `heading`）和行内节点（如 `text`, `image`）。

*   **块级分割**：在 ProseMirror 中，不同的块级内容（例如两个段落）是通过不同的块级节点来表示的，而不是通过在一个 `text` 节点中插入换行符。
*   **行内换行**：如果你需要在**同一个块级节点内**实现视觉上的换行（例如在一个段落内部强制换行），通常会使用一个特定的**行内节点**，例如 `hard_break`（硬换行）节点。这个 `hard_break` 节点本身是一个独立的行内节点，而不是 `text` 节点的一部分。

**总结**：`text` 节点只包含纯粹的文本内容，不包含任何结构性的标记，包括表示换行的 `\n`。`\n` 在 ProseMirror 的内部模型中是用于区分和组织块级内容的结构化分隔符，而非 `text` 节点的内容。

## 4. 进阶应用：数学公式 (LaTeX)

**问：类似 Latex 的数学公式语言，可以使用 prosemirror 来表达吗？**

**答：完全可以。**

这也是 ProseMirror 相比于传统富文本编辑器的强项之一。对于数学公式，通常的做法不是将其作为普通文本处理，而是定义为一个**原子节点 (Atom Node)**。

### 实现思路

1.  **数据模型 (Schema)**: 定义一个名为 `math_inline` (行内公式) 或 `math_display` (块级公式) 的节点。
2.  **内容存储**: 使用 `attrs` 属性来存储 LaTeX 源码（例如 `E = mc^2`）。
3.  **原子性**: 设置 `atom: true`。这意味着光标无法进入节点内部去“编辑”它的子节点（因为它根本没有子节点），光标只能跨过它，或者选中整个公式。
4.  **渲染 (View)**: 在编辑视图中，通常使用 `NodeView` 结合渲染库（如 **KaTeX** 或 **MathJax**）将 LaTeX 源码渲染为漂亮的数学公式。当用户双击或点击公式时，再弹出一个输入框让用户修改 LaTeX 源码。

### Schema 定义示例

```javascript
const mathNodes = {
  // 行内公式：$E=mc^2$
  math_inline: {
    group: "inline",      // 属于行内组，可以像文本一样流动
    inline: true,         // 定义为行内节点
    atom: true,           // 关键：作为一个整体单元
    attrs: {
      latex: { default: "" } // 存储 Latex 源码
    },
    parseDOM: [{
      tag: "span.math-inline", // 解析规则
      getAttrs: dom => ({ latex: dom.getAttribute("data-latex") })
    }],
    toDOM: node => ["span", { "class": "math-inline", "data-latex": node.attrs.latex }, node.attrs.latex]
    // 注意：toDOM 只是简单的序列化，实时预览通常需要 NodeView
  },

  // 块级公式：$$...$$
  math_display: {
    group: "block",       // 属于块级组
    atom: true,
    attrs: {
      latex: { default: "" }
    },
    parseDOM: [{
      tag: "div.math-display",
      getAttrs: dom => ({ latex: dom.getAttribute("data-latex") })
    }],
    toDOM: node => ["div", { "class": "math-display", "data-latex": node.attrs.latex }, node.attrs.latex]
  }
}
```

### 为什么这样设计？

*   **结构化**: 公式被视为文档结构树中的一个“叶子节点”，而不是一串杂乱的字符。
*   **互操作性**: 这种结构非常容易转换为 Markdown 的 `$` 语法或 LaTeX 原始文件。
*   **编辑体验**: 用户不会不小心删掉公式的一半字符导致渲染崩坏。你要么删除整个公式，要么通过专门的 UI 修改公式内容。
