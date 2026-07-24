# Q3: Pin 与 Future——async 中 Pin 的作用

## 问题

请介绍 Pin 与 Rust 异步编程（Future）的关系，为什么 Future 需要 Pin？

## 回答

### 一、async 的本质：编译器生成自引用状态机

当你写：

```rust
async fn foo(x: &str) -> usize {
    let a = bar().await;
    let b = baz(x).await;
    a + b
}
```

编译器生成类似这样的匿名结构体：

```rust
enum FooFuture<'x> {
    State0 { x: &'x str },
    State1 { x: &'x str, bar_fut: BarFuture, bar_result: usize },
    Done,
}
```

`State1` 中 `x` 和 `bar_fut` 存在引用关系——`bar_fut` 可能引用了局部变量。这就是**自引用**。编译器生成的 Future 跨 `.await` 点需要保留状态，必然产生自引用，因此通常是 `!Unpin`。

### 二、为什么 poll 需要 `Pin<&mut Self>`

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

如果 poll 接受普通的 `&mut self`，调用者可以用 `mem::replace` 或 `mem::swap` 移动 Future → 自引用指针悬空 → UB。`Pin<&mut Self>` 禁止了这些操作：`get_mut` 对 `!Unpin` 不可用。

**Pin 是 async 安全的基石。**

### 三、手动实现 Future 时的安全姿势

标准方案：`pin_project` 宏：

```rust
use pin_project::pin_project;

#[pin_project]
struct MyFuture {
    data: String,
    #[pin]
    inner: SomeOtherFuture,
}

impl Future for MyFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.project();
        // this.data: &mut String
        // this.inner: Pin<&mut SomeOtherFuture>
        this.inner.poll(cx)
    }
}
```

## 相关主题

- Pin 核心机制
- 为什么需要 Pin
- 实战中的 Pin 使用
