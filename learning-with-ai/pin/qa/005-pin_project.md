# Q5: Pin 下的内部字段修改——pin_project

## 问题

`!Unpin` 的 `Pin<&mut T>` 封死了 `get_mut`，如何在安全代码中修改内部字段？

## 回答

### 核心思路：结构投影（Struct Projection）

字段本身不构成自引用的那部分，可以安全拿到 `&mut`。需要向编译器证明"这个字段的 `&mut` 不会导致整体被移动"。

### 方案：`pin_project` 宏

```rust
use pin_project::pin_project;

#[pin_project]
struct MyFuture<F: Future> {
    counter: usize,           // 普通字段
    #[pin]                    // 需要 Pin 投影
    inner: F,
}

impl<F: Future> Future for MyFuture<F> {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.project();
        *this.counter += 1;          // ✅ &mut usize
        this.inner.poll(cx)          // ✅ Pin<&mut F>
    }
}
```

### 生成的类型签名

```rust
// 1. 可变投影
impl<F: Future> MyFuture<F> {
    fn project<'a>(self: Pin<&'a mut Self>) -> Projection<'a, F>;
}

struct Projection<'a, F: Future> {
    pub counter: &'a mut usize,       // 普通字段 → &mut
    pub inner: Pin<&'a mut F>,        // #[pin] 字段 → Pin<&mut>
}

// 2. 共享投影
impl<F: Future> MyFuture<F> {
    fn project_ref<'a>(self: Pin<&'a Self>) -> ProjectionRef<'a, F>;
}

struct ProjectionRef<'a, F: Future> {
    pub counter: &'a usize,           // 普通字段 → &
    pub inner: Pin<&'a F>,            // #[pin] 字段 → Pin<&>
}

// 3. 投影结构实现 Unpin（安全移动）
impl<F: Future> Unpin for Projection<'_, F> {}
impl<F: Future> Unpin for ProjectionRef<'_, F> {}
```

| 原始字段 | `project()` 类型 | `project_ref()` 类型 |
|---|---|---|
| `a: T` | `&'a mut T` | `&'a T` |
| `#[pin] b: U` | `Pin<&'a mut U>` | `Pin<&'a U>` |

### Pin 是"防君子不防小人"吗？

`unsafe` 下可以 `get_unchecked_mut` 拿到 `&mut T`，绕过 Pin。但这和 Rust 的整体安全模型一致：安全 Rust 保证无 UB，unsafe 块由程序员负责。Pin 将"保证不移动"的责任从整个代码库缩小到 unsafe 调用点。

## 相关主题

- Pin 核心机制
- Pin 与 Future
- `&pin` 语义
