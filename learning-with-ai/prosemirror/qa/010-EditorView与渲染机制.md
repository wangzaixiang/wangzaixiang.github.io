# EditorView 与渲染机制：连接数据与 DOM 的桥梁

我们已经学完了 ProseMirror 的数据模型、状态管理和扩展系统。最后一步也是最重要的一步，就是了解 **EditorView（编辑器视图）**。这不仅是渲染层，更是整个 ProseMirror 系统的“控制器”。

## 1. EditorView 的核心职责

`EditorView` 类属于 `prosemirror-view` 模块。它的主要职责是双向的：
1.  **State -> DOM**: 将当前的 `EditorState` 渲染成浏览器中的 DOM 结构。
2.  **DOM Event -> Transaction**: 监听用户的 DOM 操作（键盘、鼠标、粘贴等），将其转换为 `Transaction` 并分发。

## 2. 初始化 EditorView

创建一个编辑器非常简单，只需要一个 DOM 挂载点和一个初始的 `EditorState`。

```javascript
import { EditorState } from "prosemirror-state";
import { EditorView } from "prosemirror-view";
import { schema } from "./schema"; // 假设这是我们自定义的 Schema

// 1. 创建初始状态
const state = EditorState.create({
  schema,
  plugins: [/* ... keymap, history 等插件 ... */]
});

// 2. 创建视图
const view = new EditorView(document.querySelector("#editor"), {
  state,
  // 可选：自定义 dispatchTransaction
  dispatchTransaction(transaction) {
    const newState = view.state.apply(transaction);
    view.updateState(newState);
  }
});
```

## 3. 渲染循环 (The Update Loop)

ProseMirror 的更新机制非常高效且类似于 React 的单向数据流：

1.  **Action**: 用户操作（如按键）触发事件。
2.  **Dispatch**: 事件处理程序（通常是 Plugin）生成一个 `Transaction` 并调用 `dispatchTransaction`。
3.  **Apply**: `view.state.apply(transaction)` 计算出新的 `EditorState`。
4.  **Update**: `view.updateState(newState)` 被调用。
5.  **Diff & Patch**: `EditorView` 比较新旧两个 State 的 `doc`。它会智能地计算出最小的 DOM 变更集，只更新发生变动的部分，而不是重绘整个编辑器。这保证了极高的性能。

## 4. Props (视图属性)

`EditorView` 接受一系列配置选项，称为 **Props**。这些 Props 可以直接在初始化时传入，也可以通过插件注入。

*   **`handleKeyDown`, `handleKeyPress`, `handleTextInput`**: 拦截低级 DOM 事件。
*   **`handleDOMEvents`**: 处理任意 DOM 事件。
*   **`nodeViews`**: 这是一个非常强大的特性。它允许你为特定的节点类型（如 `image`, `code_block`）自定义渲染逻辑。
    *   你可以使用 React, Vue 或原生 JS 组件来渲染节点。
    *   这使得在这个“富文本”编辑器中嵌入复杂的交互式组件（如视频播放器、图表、嵌入式推文）成为可能。
*   **`decorations`**: 这是一个函数，根据当前 State 返回一组装饰（Decorations）。装饰可以改变视图的渲染方式而不改变实际文档数据（例如：高亮搜索结果、显示协同编辑者的光标）。

## 5. NodeView (自定义节点视图)

这是 ProseMirror 最具杀伤力的特性之一。默认情况下，ProseMirror 根据 Schema 中的 `toDOM` 规则渲染节点。但如果你需要：

*   在编辑器中显示不可编辑的 UI（如图片下方的“下载”按钮）。
*   拥有复杂的交互逻辑（如一个可点击展开的代码块）。
*   阻止光标进入某些区域。

你就需要使用 **NodeView**。

```javascript
const nodeViews = {
  image(node, view, getPos) {
    const dom = document.createElement("div");
    const img = document.createElement("img");
    img.src = node.attrs.src;
    dom.appendChild(img);
    // 添加额外的 UI
    const btn = document.createElement("button");
    btn.innerText = "Delete";
    btn.onclick = () => {
      // 获取当前节点位置并删除
      const pos = getPos();
      view.dispatch(view.state.tr.delete(pos, pos + 1));
    };
    dom.appendChild(btn);
    
    return { dom };
  }
};
```

## 总结

`EditorView` 是 ProseMirror 的“外壳”。它负责：
*   高效地将 State 映射到 DOM。
*   捕获用户输入并驱动 State 更新循环。
*   通过 `nodeViews` 和 `decorations` 提供无限的 UI 定制能力。
