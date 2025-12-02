# ProseMirror 的装饰 (Decoration) 概念

在 ProseMirror 中，**装饰 (Decoration)** 是一种强大的机制，允许你向编辑器视图添加视觉效果或自定义 UI 元素，而**不修改底层文档的内容和结构**。

## 什么是 Decoration？

想象一下你有一个文档，其中的文字可以被加粗、斜体。这些是文档内容的一部分（通过 Mark 实现）。但如果你想：

*   高亮显示搜索结果。
*   在特定的词下方显示波浪线（如拼写检查）。
*   在某个节点旁边插入一个自定义按钮（如图片上传按钮）。
*   临时显示一个协同编辑者的光标。

这些效果都是视觉上的，它们不应该改变文档的 JSON 结构，也不应该被复制粘贴到纯文本中。这时，就需要使用 Decoration。

## 为什么需要 Decoration？

核心原因在于：将**视图层效果**与**文档模型**解耦。

1.  **纯视图表示**: Decoration 纯粹是视图层的东西。它不影响文档的 `Node` 或 `Mark` 结构。这意味着，即使装饰被移除，文档的语义内容也保持不变。
2.  **动态与临时性**: 装饰可以非常灵活地添加、移除或修改，而无需触及底层的 `EditorState` 或 `Transaction` 逻辑（除了管理 Decoration 的插件状态）。它们常常用于表示临时的、非持久化的 UI 元素。
3.  **性能**: 对 Decoration 的修改通常比对文档内容进行修改的开销小。
4.  **丰富用户体验**: 允许插件开发者创建各种复杂的、交互式的编辑器功能，如：
    *   协同编辑者的光标和选区。
    *   语法高亮。
    *   内容建议 / 自动补全 UI。
    *   嵌入式小部件。

## Decoration 的类型

ProseMirror 提供了三种基本的 Decoration 类型：

### 1. 行内装饰 (Inline Decoration)

*   **作用**: 在文档中的一个文本范围上应用 CSS 样式或属性。
*   **示例**:
    *   搜索结果高亮 (背景色)。
    *   拼写错误下划线。
    *   语法高亮。
*   **构造函数**: `Decoration.inline(from: number, to: number, attrs: {[key: string]: string}, spec?: DecorationSpec)`
    *   `from`, `to`: 装饰应用的起始和结束位置 (相对于文档)。
    *   `attrs`: 一个对象，键是 HTML 属性名，值是属性值。ProseMirror 会将这些属性添加到被装饰文本的 DOM 元素上。例如 `{ class: "my-highlight", style: "background: yellow;" }`。

### 2. 节点装饰 (Node Decoration)

*   **作用**: 在整个节点（如图片、段落）上应用 CSS 样式或属性。
*   **示例**:
    *   选中一个图片节点时，在其周围显示蓝色边框。
    *   给某个代码块添加行号。
*   **构造函数**: `Decoration.node(from: number, to: number, attrs: {[key: string]: string}, spec?: DecorationSpec)`
    *   `from`, `to`: 节点在文档中的起始和结束位置。
    *   `attrs`: 同 inline Decoration。ProseMirror 会将这些属性添加到该节点的 DOM 元素上。

### 3. 小部件装饰 (Widget Decoration)

*   **作用**: 在文档的特定位置插入任意的 DOM 节点。
*   **示例**:
    *   在图片节点旁边插入一个“编辑”按钮。
    *   在段落末尾插入一个拖拽手柄。
    *   协同编辑中显示的光标（一个垂直的 DOM 元素）。
*   **构造函数**: `Decoration.widget(pos: number, toDOM: (view: EditorView, getPos: () => number) => Node, spec?: DecorationSpec)`
    *   `pos`: 插入小部件的位置。
    *   `toDOM`: 一个函数，返回要插入的 DOM 节点。
    *   `spec`: 包含 `side` 属性，用于指定小部件是在 `pos` 位置的左侧 (`-1`) 还是右侧 (`1`) 插入。默认 `0` 意味着在两者之间，并随着内容移动。

## Decoration 在 DOM 层如何呈现？

### 装饰 DOM 与文档 DOM 的区分

1. **行内 / 节点装饰不会生成“新节点”**  
   `Decoration.inline` 和 `Decoration.node` 会在渲染阶段由 `computeOuterDeco` 计算出一层或多层包裹 DOM，并将属性（class、style 等）同步到这些包裹上。ProseMirror 会在这些 DOM 上打上 `pmIsDeco` 标记，解析时只把它当成包裹壳而不是一棵新的文档节点树，因此 Decoration 不会污染 `doc`。

2. **Widget 装饰有显式标记**  
   `WidgetViewDesc` 会把 `toDOM` 生成的节点设置成 `contentEditable="false"`，并强制加上 `ProseMirror-widget` class。如果 `toDOM` 不是元素节点，还会自动包一层 `<span>`。这样 DOM 层可以清楚区分 widget 与文档内容；再次把 DOM 解析成文档时，`parseRule()` 直接 `ignore: true`，因此 widget 不会被写回 `doc`。

3. **解析阶段的“跳过”逻辑**  
   无论是包裹 DOM 还是 widget，在解析文本节点时都会一路向上跳过持有 `pmIsDeco` 的元素；widget 自身还有 `domAtom = true`，因此 `dom.pmViewDesc` 能告知编辑器“我只是装饰”。这套约定保证了 Decoration 仅存在于视图层。

### DOM Selection 为什么不会落到 Decoration 上？

1. **widget 通过 contentEditable=false 挡住原生选区**  
   浏览器原生 selection 无法进入 `contentEditable="false"` 的节点。再配合 `WidgetViewDesc.ignoreForSelection`（默认 `false`，只有 `relaxedSide` 时才允许停在另一侧），ProseMirror 可以完全控制光标落在 widget 左右哪一边。

2. **DOM 遍历也会主动绕过 widget/装饰**  
   `scanFor` 等 selection 同步逻辑在向前/向后遍历 DOM 时，只要遇到 `contentEditable="false"` 的元素就会停止，将光标位置映射回文档坐标；如果该元素的 `ViewDesc.ignoreForSelection` 为真（即 relaxedSide），则跳到相邻位置。这样 selection 中永远只包含真正的文档节点。

3. **复制/解析时被忽略**  
   widget 的 `parseRule` 设置为 `ignore: true`，并可通过 `spec.stopEvent`、`spec.ignoreSelection` 控制事件冒泡与 selection。即便用户用鼠标圈选，剪贴板和 `EditorState` 的 selection 也不会带上 widget 自己，只会记录它所在的文档位置。

## DecorationSet

所有 Decoration 都被组织在一个 `DecorationSet` 对象中。
`DecorationSet` 是不可变的，它提供了一系列方法来管理 Decoration，例如：

*   `DecorationSet.create(doc: Node, decorations: Decoration[])`: 创建一个新的 DecorationSet。
*   `set.add(doc: Node, newDecorations: Decoration[])`: 返回一个新的 DecorationSet，添加了新的装饰。
*   `set.remove(decorations: Decoration[])`: 返回一个新的 DecorationSet，移除了指定的装饰。
*   `set.map(mapping: Mappable, doc: Node)`: **非常重要**。当文档内容发生变化时，这个方法会根据 `Transaction` 的 `mapping` 对象，自动调整 Decoration 的位置，确保它们仍然指向正确的文本或节点。

## 如何使用 Decoration？

Decoration 主要通过 `prosemirror-view` 中的 `EditorView` 的 `props` 属性来注入：

```javascript
import { EditorView } from "prosemirror-view";
import { Plugin } from "prosemirror-state";
import { DecorationSet, Decoration } from "prosemirror-view";

const myDecorationPlugin = new Plugin({
  state: {
    init() { return DecorationSet.empty },
    apply(tr, set, oldState, newState) {
      // 这里的逻辑会根据交易的变化来更新 DecorationSet
      // 例如，如果有一个标记为 "add-highlight" 的 meta，就添加一个高亮
      // 如果文档内容变化，确保调用 set.map(tr.mapping, tr.doc) 来更新位置
      if (tr.docChanged) {
        set = set.map(tr.mapping, tr.doc);
      }
      const meta = tr.getMeta("my-highlight-meta");
      if (meta && meta.add) {
          const newDeco = Decoration.inline(meta.from, meta.to, { class: "my-highlight" });
          return set.add(tr.doc, [newDeco]);
      }
      return set;
    }
  },
  props: {
    // 这个函数会在每次 EditorState 更新时被调用，返回当前的 DecorationSet
    decorations(state) {
      return myDecorationPlugin.getState(state);
    }
  }
});
```

在这个例子中，`myDecorationPlugin` 的 `state` 字段负责维护一个 `DecorationSet`。当文档或插件自身的状态发生变化时，`apply` 方法会返回一个新的 `DecorationSet`。然后，`props.decorations` 会从插件状态中取出这个 `DecorationSet`，并交给 `EditorView` 进行渲染。

## 总结

Decoration 是 ProseMirror 中一个强大的工具，用于在不修改文档模型的情况下，为编辑器视图添加各种视觉效果和自定义 UI。理解其类型和 `DecorationSet` 的 `map` 方法是实现复杂编辑器功能（如搜索高亮、拼写检查、协同光标等）的关键。
