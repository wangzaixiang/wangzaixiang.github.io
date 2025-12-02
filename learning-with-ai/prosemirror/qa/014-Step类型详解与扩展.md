# Step 类型详解与扩展

## 1. Step 详细清单与分类

### 核心 Step 类型（4种）

#### **内容替换类**
- **`ReplaceStep`** - 最通用的步骤类型
  - 作用：替换文档中指定范围 `[from, to)` 的内容为新内容（`Slice`）
  - 三种用法：
    - 插入：`from === to`，替换内容非空
    - 删除：`from < to`，替换内容为空
    - 替换：`from < to`，替换内容非空
  - 是最基础、最强大的 Step，理论上可以表达大部分文档变更

#### **标记（Mark）操作类**
- **`AddMarkStep`** - 添加格式标记
  - 作用：在指定范围 `[from, to)` 的文本上添加 `Mark`
  - 应用场景：加粗、斜体、添加链接、高亮等

- **`RemoveMarkStep`** - 移除格式标记
  - 作用：在指定范围 `[from, to)` 的文本上移除 `Mark`
  - 应用场景：取消加粗、移除链接等

#### **结构变换类**
- **`ReplaceAroundStep`** - 高级结构替换
  - 作用：在保持内部内容不变的情况下替换父节点
  - 应用场景：
    - 解包列表项（保留段落内容）
    - 改变节点包装结构而不影响子内容
    - 节点类型转换（如 `<blockquote>` 转 `<div>`）

### Step 的两大核心特性

1. **可逆性（Invertible）**
   - 每个 Step 都有 `invert()` 方法生成逆向 Step
   - 应用逆向 Step 可完全撤销原操作
   - 这是 Undo/Redo 的实现基础

2. **可映射性（Mappable）**
   - 通过 `StepMap` 和 `Mapping` 系统跟踪位置变化
   - 当文档修改后，旧位置可以映射到新位置
   - 这是协同编辑冲突解决的核心机制

---

## 2. Step 可以扩展！

虽然基础文档中没有提到，但 **ProseMirror 完全支持自定义 Step**。

### 如何创建自定义 Step

创建自定义 Step 需要：

1. **继承 `Step` 类**
2. **实现必需的方法**：
   - `apply(doc)` - 应用 Step 到文档，返回 `StepResult`
   - `invert(doc)` - 生成逆向 Step
   - `map(mapping)` - 根据位置映射调整 Step
   - `getMap()` - 返回此 Step 的 `StepMap`
   - `toJSON()` - 序列化为 JSON
   - `fromJSON(schema, json)` - 从 JSON 反序列化

3. **注册 JSON 序列化标识符**（用于协同编辑）：
   ```javascript
   Step.jsonID("myCustomStep", MyCustomStep)
   ```

### 简化示例

```javascript
import { Step, StepResult } from 'prosemirror-transform'

class SetAllNodeAttrStep extends Step {
  constructor(nodeName, attrChange) {
    super()
    this.nodeName = nodeName
    this.attrChange = attrChange
  }

  apply(doc) {
    // 遍历文档，修改所有匹配节点的属性
    let modified = doc.copy(doc.content)
    // ... 实现逻辑
    return StepResult.ok(modified)
  }

  invert(doc) {
    // 返回逆向操作
    return new SetAllNodeAttrStep(this.nodeName, reverseChange)
  }

  map(mapping) {
    // 通常结构性变更不需要映射
    return this
  }

  getMap() {
    return StepMap.empty
  }

  toJSON() {
    return {
      stepType: 'setAllNodeAttr',
      nodeName: this.nodeName,
      change: this.attrChange
    }
  }

  static fromJSON(schema, json) {
    return new SetAllNodeAttrStep(json.nodeName, json.change)
  }
}

// 注册自定义 Step
Step.jsonID("setAllNodeAttr", SetAllNodeAttrStep)
```

### 实际应用场景

根据社区讨论，开发者创建自定义 Step 的常见场景：

- **批量属性修改**：一次性修改所有特定类型节点的属性
- **评论系统**：创建/删除/移动评论范围
- **自定义协同操作**：实现特殊的协同编辑行为
- **复杂结构变换**：超出 `ReplaceAroundStep` 能力的结构操作

### 注意事项

⚠️ **创建自定义 Step 需要非常小心**：
- 必须确保 `invert()` 能完全撤销操作
- `map()` 必须正确处理位置映射
- 如果需要协同编辑支持，必须正确实现 JSON 序列化
- 大多数情况下，组合使用内置的 4 种 Step 就足够了

---

## 3. Step 的作用域：只针对 Document

### 重要设计原则

**Step 只操作 `document`**，对 `selection`、`storedMarks`、`plugins` 状态的修改**不通过 Step**，而是通过 **Transaction 的其他机制**。

### 3.1 Selection 的更新：通过 `setSelection()` 方法

**不是 Step，而是 Transaction 的方法**：

```javascript
// 不存在 "SetSelectionStep"
// 而是直接调用：
tr.setSelection(newSelection)
```

**自动映射机制**：
- 当 Transaction 包含修改文档的 Steps 时，旧的 selection 会**自动**通过每个 Step 的 `StepMap` 进行映射
- 例如：光标在位置 10，你在位置 5 插入了 3 个字符，光标会自动映射到位置 13
- 但你可以通过 `setSelection()` **显式覆盖**这个自动映射

**示例**：
```javascript
let tr = state.tr
  .insert(5, schema.text("hello"))  // Step 修改 doc，selection 自动映射
  .setSelection(TextSelection.create(tr.doc, 20))  // 显式设置 selection
```

**追踪标志**：
- Transaction 有一个 `selectionSet` 属性，标记 selection 是否被显式设置
- 这对于插件判断是否需要保留用户的 selection 很重要

### 3.2 StoredMarks 的更新：通过 `setStoredMarks()` / `ensureMarks()`

**同样不是 Step**：

```javascript
tr.setStoredMarks([schema.marks.strong.create()])
// 或
tr.ensureMarks(schema.marks.em.create())
```

**自动清除机制**：
- 当文档内容改变或 selection 改变时，`storedMarks` 会**自动清除**
- 除非你显式调用 `setStoredMarks()` 来保留或设置它们

**追踪标志**：
- Transaction 有一个 `storedMarksSet` 属性，标记 storedMarks 是否被显式设置

### 3.3 Plugin State 的更新：通过 Plugin 的 `apply()` 方法

**完全不同的机制**：

Plugin state 通过插件定义的 `apply` 方法来响应 Transaction：

```javascript
new Plugin({
  state: {
    init() { return { count: 0 } },
    apply(tr, pluginState) {
      // 根据 Transaction 计算新的 plugin state
      if (tr.docChanged) {
        return { count: pluginState.count + 1 }
      }
      return pluginState  // 必须返回不可变的新对象或旧对象
    }
  }
})
```

**为什么不用 Step？**
- Plugin state 是插件私有的，不需要协同编辑支持
- Plugin state 的变化不需要 undo/redo（通常）
- Plugin state 通过重新应用 Transaction 自动重建

**特殊情况**：
某些插件（如评论系统）可能需要协同编辑支持，这时可以创建**自定义 Step** 来修改 plugin state，但这是**高级用例**。

### 3.4 设计理由

**为什么 Step 只针对 document？**

| 特性 | Document | Selection | StoredMarks | Plugin State |
|------|----------|-----------|-------------|--------------|
| **需要协同编辑** | ✅ 必需 | ❌ 本地状态 | ❌ 本地状态 | ❌ 通常不需要 |
| **需要 Undo/Redo** | ✅ 必需 | ⚠️ 自动处理 | ⚠️ 自动处理 | ❌ 通常不需要 |
| **可逆性** | ✅ 必需 | ❌ 不需要 | ❌ 不需要 | ❌ 不需要 |
| **位置映射** | ✅ 作为源头 | ⚠️ 被动映射 | N/A | N/A |

**核心思想**：
- **Step** = 协同编辑的基本单位，必须可逆、可映射、可序列化
- **Document** 是多用户共享的，需要冲突解决
- **Selection/StoredMarks** 是每个用户私有的，只需要跟随 document 变化自动调整
- **Plugin State** 是插件私有的，通过 `apply` 方法响应 Transaction 即可

### 3.5 EditorState 组件更新对比表

| 状态组件 | 更新方式 | 是否通过 Step | 自动行为 |
|----------|----------|---------------|----------|
| **doc** | Steps (`ReplaceStep`, `AddMarkStep`, etc.) | ✅ | - |
| **selection** | `tr.setSelection()` | ❌ | 自动通过 Steps 映射 |
| **storedMarks** | `tr.setStoredMarks()` / `ensureMarks()` | ❌ | 文档变化后自动清除 |
| **plugin state** | Plugin 的 `apply(tr, state)` 方法 | ❌ | 通过重新应用 tr 重建 |

### 3.6 完整示例

```javascript
let tr = state.tr
  // 1. 修改 document (通过 Step)
  .replaceWith(5, 10, schema.node('paragraph', null, [
    schema.text('new text', [schema.marks.strong.create()])
  ]))

  // 2. 设置 selection (不是 Step)
  .setSelection(TextSelection.create(tr.doc, 15))

  // 3. 设置 storedMarks (不是 Step)
  .setStoredMarks([schema.marks.em.create()])

  // 4. Plugin state 自动通过 apply 方法更新

// 应用 Transaction
let newState = state.apply(tr)
```

---

## 总结

1. **ProseMirror 内置 4 种核心 Step**：`ReplaceStep`（内容替换）、`AddMarkStep`/`RemoveMarkStep`（标记操作）、`ReplaceAroundStep`（结构变换）

2. **Step 完全可扩展**，通过继承 `Step` 类并实现必需方法即可创建自定义 Step，适用于特殊的编辑操作需求

3. **Step 只操作 document**：
   - `selection` 通过 `tr.setSelection()` 更新，会自动通过 Steps 映射
   - `storedMarks` 通过 `tr.setStoredMarks()` 更新，文档变化后自动清除
   - `plugin state` 通过 Plugin 的 `apply(tr, state)` 方法更新
   - 这种设计分离了协同编辑所需的部分（document）和本地状态（selection 等）

## 参考资料

### Step 扩展
- [Creating a Step for all node attr changes - discuss.ProseMirror](https://discuss.prosemirror.net/t/creating-a-step-for-all-node-attr-changes/2506)
- [Has anyone needed to write custom steps? - discuss.ProseMirror](https://discuss.prosemirror.net/t/has-anyone-needed-to-write-custom-steps/797)
- [Decoration Mapping Through Custom Replace Step Behavior - discuss.ProseMirror](https://discuss.prosemirror.net/t/decoration-mapping-through-custom-replace-step-behavior/2694)

### Transaction 机制
- [Using setSelection at the end of chained transactions - discuss.ProseMirror](https://discuss.prosemirror.net/t/using-setselection-at-the-end-of-chained-transactions/2400)
- [Using steps for plugin state changes - discuss.ProseMirror](https://discuss.prosemirror.net/t/using-steps-for-plugin-state-changes/1554)
- [ProseMirror RFC #10 - replaceTransaction](https://github.com/ProseMirror/rfcs/pull/10/files)

### 官方文档
- [ProseMirror Reference manual](https://prosemirror.net/docs/ref/version/0.17.0.html)
- [ProseMirror State README](https://github.com/ProseMirror/prosemirror-state/blob/master/src/README.md)
