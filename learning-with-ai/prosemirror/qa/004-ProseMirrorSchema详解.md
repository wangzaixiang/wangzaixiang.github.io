# ProseMirror Schema 详解

我们现在对 ProseMirror 有了宏观的认识，并且理解了它为何能超越传统富文本编辑器。接下来，我们将深入探讨 ProseMirror 的核心概念之一：**Schema（模式）**。

## 1. 什么是 Schema？

在 ProseMirror 中，`Schema` 是编辑器的“骨架”和“规则集”。它定义了文档的**合法结构**，是 ProseMirror 强大之处的基石。

简单来说，`Schema` 是一个配置对象，它定义了文档中允许出现的**节点类型（Node Types）**和**标记类型（Mark Types）**，以及它们各自的属性和嵌套规则。你可以将其理解为：

*   **文档的语法规则**: 类似于编程语言中的语法定义，Schema 规定了文档树可以如何构建。
*   **文档的类型系统**: 它为文档中的每一个“内容单元”定义了类型，确保文档在任何时候都符合预期的结构。

## 2. Schema 的目的与重要性

Schema 的存在解决了传统编辑器中 DOM 结构混乱的问题，并带来了以下核心优势：

*   **结构一致性**: 确保文档内容始终保持一个干净、可预测的树状结构，无论用户如何编辑。
*   **数据有效性**: 阻止用户创建非法的文档结构（例如，在段落中嵌套表格）。
*   **语义清晰**: 为文档中的每个部分赋予明确的语义，便于后续的数据处理、渲染和转换（例如转换为 Markdown、JSON）。
*   **可扩展性与可定制性**: 开发者可以通过定义自己的 Schema，创建满足特定业务需求的独特编辑器。

## 3. Schema 的构成要素

一个 Schema 主要由两部分定义：

### a. 节点 (Nodes)

节点是文档树的基本组成单元。Schema 定义了每个节点类型：

*   **名称 (name)**: 如 `paragraph`, `heading`, `text`, `image`, `doc` (根节点)。
*   **内容模型 (content expression)**: 定义了该节点可以包含哪些子节点，以及这些子节点的数量和顺序。例如，`paragraph: "inline*"` 表示段落可以包含零个或多个行内元素。
    *   **数学类比**: 这类似于**正则表达式**或**上下文无关文法**，用于描述子节点序列的合法模式。
*   **属性 (attributes)**: 节点可以拥有的数据，如 `heading` 的 `level` 属性，`image` 的 `src` 属性。
*   **DOM 映射 (toDOM/parseDOM)**: 如何将该节点渲染成 HTML DOM，以及如何从 HTML DOM 解析回该节点。
*   **组 (groups)**: 将节点归类，以便在内容模型中引用，例如 `block`, `inline`。

### b. 标记 (Marks)

标记是附加在行内节点（通常是文本节点）上的样式或语义信息，例如 `strong` (加粗), `em` (斜体), `link` (链接)。

*   **名称 (name)**: 如 `strong`, `em`, `link`。
*   **属性 (attributes)**: 标记可以拥有的数据，如 `link` 的 `href` 属性。
*   **排除组 (excludes)**: 定义了哪些标记不能与当前标记共存（例如，通常不能同时加粗和斜体，或者可以定义为可以）。
*   **DOM 映射 (toDOM/parseDOM)**: 如何将该标记渲染成 HTML DOM 标签，以及如何从 HTML DOM 解析回该标记。

### c. NodeSpec 属性详解

在定义每个节点时，`NodeSpec` 对象可以包含以下属性：

| 属性名       | 类型                                 | 语义说明                                                                                                                                                                             | 示例/备注                                                                                                                                                                                                                                    |
| :----------- | :----------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content`    | `string`                             | 定义该节点可以包含的子节点类型和数量。使用内容表达式语法。                                                                                                                            | `"inline*"`, `"block+"`, `"paragraph+"`, `"(paragraph | heading){1,5}"`。空字符串 `""` 表示不能有子节点。                                                                                                                               |
| `group`      | `string`                             | 将节点归类到一个或多个组中（用空格分隔），以便在其他节点的内容表达式中引用。                                                                                                         | `"block"`, `"inline"`, `"media"`, `"list_item"`。例如，`doc` 的 `content: "block+"` 意味着可以包含任何属于 `block` 组的节点。                                                                                                           |
| `inline`     | `boolean`                            | 如果为 `true`，表示这是一个行内节点。行内节点通常不能包含块级节点。                                                                                                                  | `text` 节点通常设置为 `true`。默认 `false`。                                                                                                                                                                                                 |
| `atom`       | `boolean`                            | 如果为 `true`，表示这是一个原子节点。原子节点被视为单个不可分割的单元，其内部内容不能被直接编辑。通常用于嵌入式内容（如图片、视频、数学公式）。                                             | `image`, `math_inline`, `math_display`。当 `atom` 为 `true` 时，通常会配合 `NodeView` 进行自定义渲染和交互。                                                                                                                              |
| `attrs`      | `object`                             | 定义节点的属性及其默认值。每个属性的键值对中，键是属性名，值是一个对象，其中包含 `default`（默认值）。                                                                               | `{ level: { default: 1 }, src: { default: null } }`。                                                                                                                                                                                        |
| `toDOM`      | `(node: Node) => DOMOutputSpec`      | 一个函数，接收一个 `Node` 对象，返回一个 DOMOutputSpec 数组，用于将 ProseMirror 节点渲染为 HTML DOM 元素。                                                                           | `(node) => ["h" + node.attrs.level, 0]`。`0` 表示子节点的位置。                                                                                                                                                                                |
| `parseDOM`   | `Array<ParseRule>`                   | 一个 `ParseRule` 数组，定义了如何从 HTML DOM 解析回 ProseMirror 节点。每个规则指定了匹配的 HTML 标签和如何提取属性、内容等。                                                          | `[{ tag: "p", preserveWhitespace: "full" }]`。                                                                                                                                                                                                |
| `selectable` | `boolean`                            | 如果为 `false`，则该节点不能被选中（例如，`doc` 节点通常不可选）。默认 `true`。                                                                                                      | `doc` 节点通常设置为 `false`。                                                                                                                                                                                                                |
| `draggable`  | `boolean`                            | 如果为 `false`，则该节点不能被拖拽。默认 `true`。                                                                                                                                    | 当节点不适合拖拽操作时设置为 `false`。                                                                                                                                                                                                     |
| `defining`   | `boolean`                            | 如果为 `true`，表示当选择区域延伸到该节点边界时，该节点应该“包含”选择。例如，标题节点通常是 defining 的，这样当选择包含标题的一部分时，会倾向于认为整个标题都被选中。默认 `false`。 | `heading` 节点。防止用户意外地将其拆分成多个节点。                                                                                                                                                                                             |
| `isolating`  | `boolean`                            | 如果为 `true`，表示编辑器的选择不应该跨越该节点的边界。通常用于隔离外部内容，防止其“渗入”或“渗出”编辑区。默认 `false`。                                                               | 例如，一个独立的代码块或表格，编辑其内部时，通常不希望光标或选择范围跑出去。                                                                                                                                                                 |
| `code`       | `boolean`                            | 如果为 `true`，表示该节点的内容是代码，不应进行智能引号、拼写检查等文本处理。默认 `false`。                                                                                              | `code_block` 节点。                                                                                                                                                                                                                         |
| `isBlock`    | `boolean` (派生)                     | 只读属性，表示节点是否为块级节点 (`inline: false`)。                                                                                                                                 | `paragraph`, `heading` 的 `isBlock` 为 `true`。                                                                                                                                                                                               |
| `isInline`   | `boolean` (派生)                     | 只读属性，表示节点是否为行内节点 (`inline: true`)。                                                                                                                                  | `text`, `image` 的 `isInline` 为 `true`。                                                                                                                                                                                                     |
| `isText`     | `boolean` (派生)                     | 只读属性，表示节点是否为文本节点。                                                                                                                                                   | 仅 `text` 节点为 `true`。                                                                                                                                                                                                                     |

### d. MarkSpec 属性详解

在定义每个标记时，`MarkSpec` 对象可以包含以下属性：

| 属性名       | 类型                                 | 语义说明                                                                                                                                                                             | 示例/备注                                                                                                                                                                                                                                    |
| :----------- | :----------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `attrs`      | `object`                             | 定义标记的属性及其默认值。每个属性的键值对中，键是属性名，值是一个对象，其中包含 `default`（默认值）。                                                                               | `{ href: { default: null }, title: { default: "" } }` 用于 `link` 标记。                                                                                                                                                                  |
| `inclusive`  | `boolean`                            | 如果为 `true`（默认值），表示当光标位于标记的边界时，输入文本将自动继承该标记。如果为 `false`，则光标在标记边界时输入文本不会被标记。                                                        | `strong`, `em` 通常是 `true`。 `link` 标记有时会设置为 `false`，以便在链接的末尾输入新文本时不会自动成为链接的一部分。                                                                                                                    |
| `excludes`   | `string`                             | 定义了该标记不能与哪些其他标记共存。可以是一个用空格分隔的标记名称或组名列表。如果未指定，则默认为 `_`（即排除所有其他标记），但通常会明确列出。                                       | `"link"`, `"strong em"`。例如，如果 `strong` 的 `excludes` 设置为 `"em"`，则文本不能同时是加粗和斜体。如果设置为 `""` 则允许与所有标记共存。                                                                                              |
| `group`      | `string`                             | 将标记归类到一个或多个组中（用空格分隔），以便在 `excludes` 属性中引用。                                                                                                             | `"font_style"`, `"link_marks"`。                                                                                                                                                                                                             |
| `toDOM`      | `(mark: Mark, inline?: boolean) => DOMOutputSpec` | 一个函数，接收一个 `Mark` 对象，返回一个 DOMOutputSpec 数组，用于将 ProseMirror 标记渲染为 HTML DOM 元素。                                                                           | `() => ["strong", 0]`。`0` 表示子节点的位置。                                                                                                                                                                                                 |
| `parseDOM`   | `Array<ParseRule>`                   | 一个 `ParseRule` 数组，定义了如何从 HTML DOM 解析回 ProseMirror 标记。每个规则指定了匹配的 HTML 标签和如何提取属性。                                                                 | `[{ tag: "strong" }]`, `[{ tag: "a[href]", getAttrs: dom => ({ href: dom.getAttribute("href") }) }]`。                                                                                                                                  |
| `spanning`   | `boolean`                            | 如果为 `false`，则当标记在 DOM 中被中断（例如，`<strong>foo<em>bar</em></strong>` 这样的结构，`em` 打断了 `strong`）时，ProseMirror 会尝试将 DOM 结构平铺。默认 `true`。         | 通常保持默认 `true`。仅在需要特定 DOM 结构匹配时可能修改。                                                                                                                                                                              |

## 4. Schema 示例 (概念性)

```javascript
// 这是一个概念性的 Schema 定义，实际使用时会有更详细的语法
const mySchema = new Schema({
  nodes: {
    doc: { content: "block+" }, // 文档包含一个或多个块级节点
    paragraph: {
      content: "inline*",      // 段落包含零个或多个行内节点
      group: "block",          // 属于 'block' 组
      toDOM(node) { return ["p", 0]; } // 渲染为 <p>，0 表示子节点位置
    },
    heading: {
      content: "inline*",
      group: "block",
      attrs: { level: { default: 1 } }, // 标题有 level 属性，默认为 1
      toDOM(node) { return ["h" + node.attrs.level, 0]; }
    },
    text: { inline: true } // 文本节点是行内节点，没有子节点
  },
  marks: {
    strong: {
      toDOM() { return ["strong", 0]; } // 渲染为 <strong>
    },
    em: {
      toDOM() { return ["em", 0]; }     // 渲染为 <em>
    },
    link: {
      attrs: { href: {} },             // 链接有 href 属性
      toDOM(mark) { return ["a", { href: mark.attrs.href }, 0]; }
    }
  }
});
```

## 5. 总结

Schema 是 ProseMirror 的核心。通过精心设计 Schema，我们可以为编辑器提供一个强大的、可预测的底层数据结构，这为实现复杂的编辑功能、数据存储和跨平台内容转换奠定了坚实的基础。它是 ProseMirror 将“所见即所得”转变为“所见即**结构化**数据”的关键。