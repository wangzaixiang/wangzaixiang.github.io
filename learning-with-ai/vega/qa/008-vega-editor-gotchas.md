# QA: 为什么 Vega Editor 中 View API 更新无效？

## 问题
在本地 HTML 中测试 `view.change(...).run()` 正常，但在 [Vega Editor](https://vega.github.io/editor/) 的控制台中运行同样的代码，虽然 `view.data('table')` 显示数据已追加，但图表视图没有更新。

## 原因分析
这是 Vega Editor 自身架构带来的一个**特例陷阱**，而不是 Vega 库本身的问题。

### 1. Vega Editor 的工作原理
Vega Editor 是一个基于 React/Redux 的复杂应用。
*   它维护着 Spec 的状态。
*   当你修改代码时，它会**销毁**旧的 Vega View 实例，并根据新的 Spec 字符串**重新创建**一个新的 View 实例。

### 2. 全局变量 `view` 的陷阱
*   在 Vega Editor 的控制台中，全局变量 `VIEW` (注意大写，或者是 `view`) 通常是指向**当前**正在渲染的 Vega 实例。
*   **但是**，当你通过控制台直接操作这个 `view` 对象时，Vega Editor 的 React 状态管理层并不知道你绕过它修改了内部状态。
*   这就产生了一个**竞态条件 (Race Condition)** 或 **状态脱节**：
    1.  你通过 JS 修改了 View 的数据流。
    2.  Vega 内部确实更新了数据（所以 `view.data()` 能看到）。
    3.  但是，Vega Editor 可能挂载了某种监听器，或者因为你没有触发它的 React 渲染循环，导致 Canvas 没有重绘到屏幕上（尽管这在纯 DOM 环境很少见）。

### 3. 更常见的原因：Scale Domain 没有自动刷新
如果你的 Spec 中 Scale Domain 是硬编码的，或者虽然绑定了 data 但因为某种原因没有触发 Dataflow 的传播。

但在 Editor 中最可能的原因是：**Vega Editor 的调试 View 变量名不一定是 `view`**。
*   在 Vega Editor 中，你可以尝试访问 `VEGA_DEBUG.view` 来确保你拿到的是正确的实例。

### 4. 最可能的“软”原因：run() 的异步性
Vega 的 `run()` 是同步的，但渲染可能是异步的（RequestAnimationFrame）。
在某些浏览器控制台环境中，如果你紧接着去检查状态，可能还没渲染完。
尝试使用 `await view.change(...).runAsync()`。

## 验证方法
在 Vega Editor 中，不要直接在 Console 里敲代码。
尝试在 Spec 的 `signals` 中添加一个点击事件来触发数据更新，看看在**图表内部交互**是否生效。如果生效，说明 Vega 逻辑没问题，只是 Console 调试环境的问题。

```json
{
  "signals": [
    {
      "name": "addData",
      "on": [
        {
          "events": "click",
          "update": "modify('table', append({'category': 'New', 'amount': 50}))"
          // 注意：这是伪代码，Vega 表达式语言里没有直接的 modify 函数，
          // 通常这种操作要在 JS 宿主代码里做，或者用 Trigger。
        }
      ]
    }
  ]
}
```
*纠正：Vega 表达式语言确实不支持直接修改数据源。必须通过 View API。*

## 结论
如果你在本地 HTML 中是正常的，那么你的理解和代码是**正确**的。Vega Editor 的控制台行为不能作为标准判定依据，因为它包裹了太多的中间层。
