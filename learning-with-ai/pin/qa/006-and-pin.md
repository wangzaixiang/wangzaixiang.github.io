# Q6: `&pin`——Pin 从库到语言的升级

## 问题

请介绍 Rust 中 `&pin`（原生 pinned 引用）的概念，与现有 `Pin<P>` 的对比，以及类型体系中的位置。

## 回答

### 一、从库到语言

```
现状（库）：                        愿景（语言）：
Pin<&mut T>                        &pin mut T
Pin<&T>                            &pin T
Pin<Box<T>>                        Box<pin T>
```

`&pin` 将是 `&` 和 `&mut` 之外的第三种原生引用类型，把 Pin 概念从 trait 门禁 + unsafe 契约升级为编译器的类型规则。

### 二、投影规则——不再需要 `#[pin]` 标注

```
&pin mut Self
     │
     │  编译器展开结构体布局，逐字段投影
     │
     ├── field_a: Unpin ──→ &mut field_a（自动）
     ├── field_b: Unpin ──→ &mut field_b（自动）
     └── field_c: !Unpin ─→ &pin mut field_c（保持）
```

与 `pin_project` 的对比：

```rust
// 今天：显式标注
#[pin_project]
struct MyFuture {
    counter: usize,       // project() → &mut usize
    #[pin]                // 手动标注
    inner: SomeFuture,    // project() → Pin<&mut SomeFuture>
}

// &pin 世界：不需要标注
struct MyFuture {
    counter: usize,       // usize: Unpin → 自动降级为 &mut
    inner: SomeFuture,    // SomeFuture: !Unpin → 保持 &pin mut
}
```

### 三、`&pin T` vs `&T`

运行时行为完全等效，类型系统上不完全等效：

```
&pin T  ──自动降级──→  &T      ✅ 永远可以
&T      ──无法升级──→  &pin T  ❌ 不能（丢失了 pin 标记）
```

### 四、`&pin mut T` vs `&mut T`

```
!Unpin 类型：
  &pin mut T  ──✗──→  &mut T    ❌ &pin mut 没有 swap 能力
  &mut T      ──✓──→  &pin mut T ✅ 放弃 swap 能力即可

Unpin 类型：
  双向等价，互转无障碍
```

### 五、能力格（Capability Lattice）

`!Unpin` 类型的四种引用构成偏序格：

```
      &mut T        ← 最高：读写 + swap + replace
       ├── &pin mut T  ← 中间：读写，不能移动
       │      └── &pin T
       └── &T      ← 最低：只读
              ↑—— &pin T
```

| 降级 | 放弃什么 |
|---|---|
| `&mut T → &pin mut T` | swap / replace 能力 |
| `&mut T → &T` | 写权限 + swap |
| `&pin mut T → &pin T` | 写权限 |
| `&pin T → &T` | Pin 标记（trivial） |

不可比较的对：`&pin mut T` 和 `&T`——一个能写不能 swap，一个不能写也不需要 swap，能力交集不同，互不转换。

`Unpin` 类型下菱形坍缩为两层链：`&mut ↔ &pin mut` → `&T ↔ &pin T`，Pin 标记无意义。

### 六、与 `Pin<P>` 世界的能力映射

| Pin 世界 | &pin 世界 |
|---|---|
| `Pin::get_mut()` 仅 Unpin | `&pin mut → &mut` 仅 Unpin |
| `unsafe get_unchecked_mut` | unsafe 块显式标记 |
| `unsafe Pin::new_unchecked` | 不需要：`&mut → &pin mut` 自动合法 |

### 七、当前状态（2026.07）

`&pin` 仍在 Nightly 实验阶段，短期无法稳定。需在 Nightly 下启用 `#![feature(pin_ergonomics)]`。

#### 已完成

| 进展 | 状态 |
|---|---|
| `&pin const\|mut T` → `&(mut) T` 类型强制（仅 `Unpin`） | ✅ 已合入 Nightly |
| `&pin const\|mut T` 的 pattern matching | ✅ 已合入 Nightly |
| `ref pin mut` 绑定模式 | ✅ 已合入 Nightly |

#### 待完成

| 任务 | 状态 |
|---|---|
| borrowck 对 `&pin mut $place` 的检查 | 📝 Draft PR |
| `Drop::pin_drop`（Pin 下的析构） | ❌ 未开始 |
| auto borrowing（方法调用自动借入） | ❌ 未开始 |
| 用户反馈 + RFC 更新 + 稳定化 | ❌ 未开始 |

#### 核心阻塞点

`&pin mut` 的 borrowck 语义与普通 `&mut` 不同：

```
&mut:      借出 → 独占访问 → 归还 → 地方解除锁定
&pin mut:  借出 → 独占访问 → 归还 → 但地方永久钉住！
```

打过 `&pin mut` 的栈位置，即使引用已归还，之后也不能再被移动。这和引用结束后效果消失的直觉冲突，需要大量用户反馈验证可教性。

**预估：最早 2027 年才有可能稳定。**

## 相关主题
