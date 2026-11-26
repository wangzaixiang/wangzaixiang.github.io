# 第 4 章 Cell 家族：在共享引用下受控修改

> 配套代码：`code/src/bin/ch04_cells.rs`  
> 所有布局数字均为本机实测（aarch64-apple-darwin，rustc 1.100.0）。

## 4.1 为什么需要 Cell

普通 `&T` 把 `T` 视为只读。即使 `T` 是：

```rust
struct Counter {
    value: u32,
}
```

下面仍然不能通过 `&Counter` 修改字段：

```rust
let counter = Counter { value: 0 };
let shared = &counter;
// shared.value += 1; // error
```

`Cell` 家族的共同作用是：**允许通过共享引用修改某个位置，同时用不同强度的手段维持别名安全。**

从弱到强的控制层级：

```text
UnsafeCell<T>
    只标记“这里允许内部可变性”
    访问和别名规则由 unsafe 调用者负责
        ↓
Cell<T>
    只允许整体读出/写入/替换
    不建立内部引用
        ↓
RefCell<T>
    允许取得 T 的内部引用
    运行时检查 shared / exclusive borrow
        ↓
OnceCell<T> / LazyCell<T, F>
    写入一次，之后主要提供共享引用
```

它们都只解决**单线程内的别名与可变性**。除另行替换成同步原语外，它们不能用于跨线程共享。

## 4.2 UnsafeCell：唯一的底层原语

当前定义：

```rust
#[lang = "unsafe_cell"]
#[repr(transparent)]
pub struct UnsafeCell<T: ?Sized> {
    value: T,
}
```

`UnsafeCell<T>` 与 `T` 的 size、align 和 ABI 相同。它没有运行时状态，也没有检查：

```text
u8              size= 1 align= 1
UnsafeCell<u8>  size= 1 align= 1

String             size=24 align= 8
UnsafeCell<String> size=24 align= 8
```

核心 API 是：

```rust
pub const fn get(&self) -> *mut T
```

它把共享引用变成裸指针，后续写入必须由 `unsafe` 代码证明合法：

```rust
let cell = UnsafeCell::new(10_i32);
let shared = &cell;

unsafe {
    *shared.get() += 1;
}

assert_eq!(cell.into_inner(), 11);
```

实测输出：

```text
UnsafeCell value = 11
```

这个例子安全，因为：

- 只有一个线程；
- 写入期间没有其他 `&i32` 或 `&mut i32` 同时存在；
- 修改不会违反 `i32` 的值有效性。

`UnsafeCell` 并不自动保证这些条件。

### 语义作用：退出 Freeze

编译器通常可以假设：通过共享引用间接到达的数据不会变化。这个能力与内部 auto trait `Freeze` 相关。

`UnsafeCell<T>` 是编译器识别出的内部可变性入口：

```text
&T
  默认可以认为其 pointee 不会被修改

&UnsafeCell<T>
  编译器不再作这种假设
```

这就是为什么 `&Cell<T>`、`&RefCell<T>` 可以在不拥有 `&mut` 的情况下修改内部值。

`Freeze` 在当前 `core` 中的定义是：

```rust
#[lang = "freeze"]
#[unstable(feature = "freeze", issue = "121675")]
pub unsafe auto trait Freeze {}

impl<T: PointeeSized> !Freeze for UnsafeCell<T> {}
```

这段定义说明了两个关键点：

- `Freeze` 是编译器使用的内部 trait，当前仍标为 unstable；
  稳定代码不能写 `T: Freeze`，也不应该把它当成用户可扩展的接口。
- `UnsafeCell<T>` 被明确排除在 `Freeze` 之外；其他类型默认按字段结构推导。

`core` 对它的说明是：检查一个类型是否**直接或嵌套地包含 `UnsafeCell`，但不穿透间接层**。
所以这里的“冻结”是指一段连续存储中的字段不会通过共享引用改变，
不是“这个对象在程序整个生命周期中永远不变”。

#### Freeze 看的是内联布局，不是 pointee

`Freeze` 描述的是类型自身直接占有的字节。引用、裸指针和堆分配会切断这个判断：

| 类型 | `Freeze` | 原因 |
|---|---:|---|
| `u32` | 是 | 不包含 `UnsafeCell` |
| `UnsafeCell<u32>` | 否 | 定义中显式 `!Freeze` |
| `Cell<u32>` | 否 | 内联字段是 `UnsafeCell<u32>` |
| `struct Mixed { version: u32, counter: Cell<u32> }` | 否 | 结构体直接包含 `Cell<u32>` |
| `&Cell<u32>` | 是 | `&T` 自身只是指针；它指向的内容经过间接层 |
| `Box<Cell<u32>>` | 是 | `Box` 的内联部分是所有权指针，元素在间接层后 |
| `Vec<Cell<u32>>` | 是 | `Vec` 的内联 header 不含 `UnsafeCell`，元素在堆分配中 |
| `AtomicU32` | 否 | 内部通过 `UnsafeCell` 表达原子内部可变性 |

当前 `core` 还显式给出了引用与指针的 impl：

```rust
unsafe Freeze for
    {T: PointeeSized} *const T,
    {T: PointeeSized} *mut T,
    {T: PointeeSized} &T,
    {T: PointeeSized} &mut T,
```

因此 `&Cell<u32>: Freeze` 与“不能通过 `&Cell<u32>` 修改值”并不矛盾：

```text
&Cell<u32> 这份引用值本身不可变
    它指向的 Cell 仍可在共享借用下修改
```

`Freeze` 在类型层是粗粒度的，但别名规则并不是“整个 `Mixed` 的所有字节都能写”。
在 `Mixed` 中，只有 `counter` 内部的 `UnsafeCell<u32>` 获得了共享引用下的写入许可；
若把 `&version` 强转成裸指针再写，仍然是未定义行为。编译器可以把整个类型标为
`!Freeze` 以保守处理，但 `UnsafeCell` 并不会外溢成整个结构体的 write token。

#### 成对示例：一次读取，还是必须重新读取

下面两个函数形状相同：先读一次，把引用交给另一个函数，再读一次。
区别只在被读类型是普通 `u32` 还是 `Cell<u32>`。完整代码位于
`code/src/bin/ch04_cells.rs`：

```rust
use std::cell::Cell;
use std::hint::black_box;

#[inline(never)]
fn observe_plain(value: &u32) {
    black_box(value);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn freeze_read_plain(value: &u32) -> u64 {
    let before = u64::from(*value);
    observe_plain(black_box(value));
    let after = u64::from(*value);
    (after << 32) | before
}

#[inline(never)]
fn observe_cell(value: &Cell<u32>) {
    black_box(value);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn freeze_read_cell(value: &Cell<u32>) -> u64 {
    let before = u64::from(value.get());
    observe_cell(black_box(value));
    let after = u64::from(value.get());
    (after << 32) | before
}
```

`observe_*` 没有修改任何值；`black_box` 只阻止优化器直接看穿这次引用传递。
运行探针：

```text
plain before/after = 0000000b0000000b
Cell  before/after = 0000000b0000000b
Cell mutation before/after = 11/22
```

前两行数值相同，不能揭示优化边界。编译到 aarch64 汇编后才能看到区别：

```bash
cd code
cargo rustc --release --bin ch04_cells -- --emit asm -C debuginfo=0
```

本机 `rustc 1.100.0` 的关键指令如下：

```asm
; freeze_read_plain
ldr     w19, [x0]             ; 调用前读取一次
bl      ...observe_plain
orr     x0, x19, x19, lsl #32 ; 调用后继续使用同一个寄存器值

; freeze_read_cell
ldr     w20, [x0]             ; 调用前读取
bl      ...observe_cell
ldr     w8, [x19]             ; 调用后必须重新读取
orr     x0, x20, x8, lsl #32
```

语义差别是：

```text
&u32
  在共享借用有效期内，合法 Rust 代码不能修改 pointee
  → 编译器可以把 before 的 load 跨过 observe_plain

&Cell<u32>
  被调用函数可以通过 Cell API 修改 pointee
  → observe_cell 之后必须重新读取
```

这是本机、本版本、开启优化后的**观察结果**，不是 ABI 或汇编承诺。
真正的契约是：优化器只能在保持合法程序语义的前提下做变换；`Freeze`
给出了“共享引用期间这些字节不会改变”的证明依据。

`Cell<u8>` 与 `u8` 可以同大小，正说明布局相同不等于类型语义相同。
`Cell` 没有运行时标签；编译器区分的依据是类型中的 `UnsafeCell`，
不是某几个额外字节。

#### 越过 UnsafeCell 就会进入 UB 边界

普通 `u32` 不能通过把 `&u32` 强转成 `*mut u32` 来合法修改：

```rust
let value = 1_u32;
let pointer = &value as *const u32 as *mut u32;

unsafe {
    *pointer = 2; // UB：pointee 不在 UnsafeCell 内
}
```

这里能通过 `unsafe` 编译，不代表操作合法。`unsafe` 只是在要求调用者自行证明
前置条件；编译器仍然可以按 `u32` 在共享引用下不可变来优化。

正确形式必须把可修改存储放进 `UnsafeCell`：

```rust
let value = UnsafeCell::new(1_u32);
let pointer = value.get();

unsafe {
    *pointer = 2; // 仍需由调用者维护别名、生命周期和有效性
}
```

但 `UnsafeCell` 只解除一条约束，不是通用免检：

| 边界 | `UnsafeCell` 是否解除 |
|---|---:|
| 共享引用 `&T` 下 pointee 不可修改 | 是，仅限 `UnsafeCell` 内的字节 |
| 可变引用 `&mut T` 必须唯一 | 否 |
| 多线程数据竞争 | 否，仍需原子操作或同步 |
| 引用期间对象必须存活 | 否 |
| 值的有效性与 drop 规则 | 否 |
| `Sync` 的线程共享检查 | 否；裸 `UnsafeCell<T>: !Sync` |
| 动态借用冲突 | 否；`RefCell` 仍需在运行时检查 |

#### Freeze 不是 Sync，也不是“值永远不变”

`Freeze` 回答的是“共享引用期间，内联字节是否可能改变”；
`Sync` 回答的是“`&T` 能否安全跨线程共享”；`Unpin` 回答的是
“固定到地址后能否再次移动”。它们是不同的性质。

例如：

```text
Cell<u32>     : !Freeze, Send, !Sync
AtomicU32    : !Freeze, Send, Sync
&AtomicU32   :  Freeze, Send, Sync
```

`AtomicU32` 内部也有 `UnsafeCell`，所以同样不是 `Freeze`；但原子操作使它
可以安全地被多线程共享。反过来，`&AtomicU32` 自身是 `Freeze`，
因为它只是一个不可变的指针值；pointee 的内部可变性没有因此消失。

`Freeze` 也不是“对象永远不变”。下面完全合法：

```rust
let mut value = 0_u32;
let before = {
    let shared = &value;
    *shared
};

value = 1;

let after = {
    let shared = &value;
    *shared
};

assert_eq!((before, after), (0, 1));
```

边界是每次共享借用期间：借用结束后，可以再次通过 `&mut` 或所有权修改；
下一次共享借用再冻结新的值。`Freeze` 是类型性质，不是运行期的一把锁。
即使 `OnceCell<T>` 已经初始化完成，它的类型也不会动态变成 `Freeze`。

所以可以把编译器边界压成三层：

1. **类型检查边界**：安全代码拒绝通过普通 `&T` 写入；`Cell` / `RefCell`
   通过封装把允许的写操作暴露成安全 API。
2. **别名与有效性边界**：`unsafe` 代码仍必须保证共享/独占引用、生命周期、
   初始化和并发规则；越过 `UnsafeCell` 往往不是编译错误，而是 UB。
3. **优化边界**：优化器可以利用 `Freeze` 消除重复 load、把只读静态放入只读段等；
   具体优化和汇编结果属于实现细节，必须随合法语义变化。

### 线程约束

```rust
impl<T: ?Sized> !Sync for UnsafeCell<T> {}
```

无论 `T` 是否 `Sync`，裸 `UnsafeCell<T>` 都不是 `Sync`，避免用户无意中把无同步的内部可变性共享给多个线程。

稳定版没有 `SyncUnsafeCell`；当前 `core` 中存在同名类型但它仍是 unstable。它只是在用户自己保证同步时放宽 `Sync`，不会替你提供同步。

## 4.3 Cell：整体值替换，不借出内部引用

当前定义：

```rust
#[repr(transparent)]
pub struct Cell<T: ?Sized> {
    value: UnsafeCell<T>,
}
```

因此：

```text
Cell<T> 与 T 同大小、同对齐
```

实测：

```text
u8        size= 1 align=1
Cell<u8>  size= 1 align=1

String       size=24 align=8
Cell<String> size=24 align=8
```

`Cell<T>` 没有 borrow counter。它的安全策略是：**不让你取得指向内部 `T` 的长期引用**。

### 常用操作

```rust
let number = Cell::new(5_u32);
let shared = &number;

shared.set(shared.get() + 1);
assert_eq!(number.get(), 6);
```

关键方法：

| 方法 | 条件 | 行为 |
|---|---|---|
| `get` | `T: Copy` | 复制出当前值 |
| `set` | `T: Sized` | 覆盖旧值 |
| `replace` | `T: Sized` | 换入新值，返回旧值 |
| `take` | `T: Default` | 取出值，留下 `T::default()` |
| `get_mut` | `&mut Cell<T>` | 编译期独占，安全取得 `&mut T` |
| `into_inner` | 取得所有权 | 取出内部值 |
| `as_ptr` | 任意 | 返回裸指针，后续写需要 unsafe |

为什么 `get` 要求 `T: Copy`？

如果安全地返回 `&T`，外部就可能在 `Cell::set` 后继续持有旧数据的引用，重新引入别名冲突：

```text
&Cell<T> 提供写入能力
再借出 &T          // 两者会冲突
```

`Cell` 拒绝这个方向，只允许把当前值复制出来。

### 非 Copy 值

`Cell<String>` 不能 `get()` 出 `String`，但仍可以整体替换：

```rust
let text = Cell::new(String::from("old"));
let old = text.replace(String::from("new"));
let current = text.take();
```

实测：

```text
Cell<String> old = "old", current = "new"
```

`Cell` 在替换时正常转移和析构完整值，不会泄漏内部 `String`。

### 线程约束

```rust
unsafe impl<T: ?Sized> Send for Cell<T> where T: Send {}
impl<T: ?Sized> !Sync for Cell<T> {}
```

含义：

```text
Cell<T> 可以 move 到另一个线程
但不能通过 &Cell<T> 同时被多个线程共享
```

需要原子变量时使用 `AtomicUsize` 等；需要跨线程修改任意 `T` 时使用 `Mutex` 或 `RwLock`。

## 4.4 RefCell：运行时借用检查

当前核心布局：

```rust
pub struct RefCell<T: ?Sized> {
    borrow: Cell<BorrowCounter>,
    value: UnsafeCell<T>,
}

type BorrowCounter = isize;
```

`BorrowCounter` 在单线程内记录当前借用状态：

```text
0   无借用
> 0 shared borrow 数量
< 0 mutable borrow 状态
```

实测布局：

```text
u8              size= 1 align=1
RefCell<u8>     size=16 align=8

String          size=24 align=8
RefCell<String> size=32 align=8
```

`RefCell<u8>` 从 1 字节增长到 16 字节，是因为它额外保存一个 `isize` borrow counter，并满足 8 字节对齐。

### shared borrow

```rust
let values = RefCell::new(vec![1, 2, 3]);

let first = values.borrow();
let second = values.borrow();
```

允许多个 `Ref` 同时存在。每次 borrow 增加 shared 计数，最后一个 guard drop 后计数回到 0。

### mutable borrow

```rust
let mut exclusive = values.borrow_mut();
exclusive.push(4);
```

如果仍有 shared 或 mutable guard 存活，`borrow_mut()` 会 panic：

```rust
let first = values.borrow();
// values.borrow_mut(); // panic
drop(first);
```

想返回错误而不是 panic，使用：

```rust
values.try_borrow_mut()
```

探针实测：

```text
shared borrows = 6, try_borrow_mut is_err = true
values after RefMut = [1, 2, 3, 4]
```

### Ref 与 RefMut 是 RAII guard

```rust
let guard: Ref<'_, Vec<u32>> = values.borrow();
```

guard 本身负责在 drop 时释放 borrow 计数：

```text
borrow()      → counter += 1
Ref drop      → counter -= 1

borrow_mut()  → counter 进入写状态
RefMut drop   → 写状态结束
```

如果通过 `mem::forget` 泄漏 guard，cell 会一直处于已借出状态，后续借用会持续 panic。但这是安全行为，不会产生悬垂引用或数据竞争。

`RefMut::map_split` 可以在一个 mutable borrow 中把不重叠区域拆成多个 `RefMut`。这依赖切片范围不重叠，并不是让普通 `&mut T` 同时复制。

### 常见组合：`Rc<RefCell<T>>`

```rust
let values = Rc::new(RefCell::new(vec![10]));
let another = Rc::clone(&values);
another.borrow_mut().push(20);
```

实测：

```text
Rc count = 2, shared values = [10, 20]
```

这个组合提供单线程的“共享 owner + 运行时可变借用”：

```text
Rc
→ 多个所有者

RefCell
→ 多个所有者之间动态协调可变访问
```

代价是：

- `Rc` 不是线程安全；
- `RefCell` 也不是 `Sync`；
- 借用冲突会在运行时 panic；
- `Rc` 循环会泄漏。

跨线程需要 `Arc<Mutex<T>>` 或 `Arc<RwLock<T>>`。

## 4.5 OnceCell 与 LazyCell

### OnceCell：最多写入一次

当前定义：

```rust
pub struct OnceCell<T> {
    // Invariant: written to at most once.
    inner: UnsafeCell<Option<T>>,
}
```

它不需要 borrow counter，因为初始化后的正常访问只有 `&T`：

```text
None -> 允许 set / get_or_init
Some -> 只允许读取；再次 set 返回 Err
```

实测：

```text
before initialization = None
value = "initialized", second set rejected = true,
stored = Some("initialized")
```

布局实测：

```text
u8              size=1
OnceCell<u8>    size=2     // Option<u8>: payload + tag

String          size=24
OnceCell<String> size=24   // Option<String> 当前用 niche 消灭 tag
```

`OnceCell<String>` 的 24 字节沿用当前 `Option<String>` 布局，属于实现细节。

`OnceCell<T>` 明确 `!Sync`。其线程安全对应物是：

```text
std::cell::OnceCell<T>  → std::sync::OnceLock<T>
```

### LazyCell：首次访问时执行初始化函数

当前核心定义：

```rust
pub struct LazyCell<T, F = fn() -> T> {
    state: UnsafeCell<State<T, F>>,
}

enum State<T, F> {
    Uninit(F),
    Init(T),
    Poisoned,
}
```

状态机：

```text
Uninit(F)
    │ Deref / force
    ▼
临时 Poisoned
    │ F() 成功
    ▼
Init(T)

如果 F() panic
→ 保持 Poisoned
→ 后续访问继续 panic
```

探针实测：

```text
before deref: count = 0
first = "lazy"
second = "lazy"
after derefs: count = 1
```

首次解引用执行闭包，第二次只读取已经初始化好的值。

布局实测：

```text
LazyCell<String, fn()> size=24 align=8
```

具体省掉了哪些 tag/padding 取决于 `State<T, F>` 的 niche 利用，属于实现细节。

线程安全对应物：

```text
std::cell::LazyCell<T, F>  → std::sync::LazyLock<T, F>
```

`LazyLock` 可以用于 `static`，并带有跨线程同步。

## 4.6 并发约束与替代物

Cell 家族的常见组合约束：

| 类型 | `Send` | `Sync` | 说明 |
|---|---|---|---|
| `UnsafeCell<T>` | `T: Send` | 否 | 裸内部可变性入口 |
| `Cell<T>` | `T: Send` | 否 | 只有整体读写 |
| `RefCell<T>` | `T: Send` | 否 | 单线程运行时借用检查 |
| `OnceCell<T>` | `T: Send` | 否 | 单线程一次性初始化 |
| `LazyCell<T, F>` | 条件性 `Send` | 否 | 通过 `UnsafeCell` 内部修改 |
| `OnceLock<T>` | `T: Send` | `T: Send + Sync` 时 | 线程安全一次性初始化 |
| `LazyLock<T, F>` | 视 `T/F` | 线程安全版本 | 可用于 `static` |

准确边界应以具体类型的 auto trait 推导和标准库 impl 为准；这张表表达的是设计方向。

不要用 `Cell` 或 `RefCell` 绕过 `Sync`：

```text
Cell / RefCell 不是“没有锁的 Mutex”
它们只是没有跨线程同步
编译器通过 !Sync 阻止安全代码跨线程共享
```

需要跨线程时：

| 需求 | 类型 |
|---|---|
| 任意 `T` 的互斥访问 | `Mutex<T>` |
| 多读或单写 | `RwLock<T>` |
| 数值原子更新 | `AtomicUsize`、`AtomicBool` 等 |
| 一次性全局初始化 | `OnceLock<T>` |
| 延迟全局初始化 | `LazyLock<T, F>` |

## 4.7 Variance：内部可变性为什么使 T 不变

在[第 3 章](references-lifetimes.md)已经看到：

```text
UnsafeCell<T> 对 T invariant
```

原因仍然相同：

```text
&T     只读        → covariant
&UnsafeCell<T> 可写 → invariant
```

`Cell<T>` 与 `RefCell<T>` 都包含 `UnsafeCell<T>`，因此对 `T` invariant。

```rust
// compile_fail
use std::cell::Cell;

fn overwrite<'a>(cell: &Cell<&'a str>, value: &'a str) {
    cell.set(value);
}

fn main() {
    let cell: Cell<&'static str> = Cell::new("static");
    let local = String::from("local");
    overwrite(&cell, &local);
    println!("{}", cell.get());
}
```

如果 `Cell<&'static str>` 可以变成 `Cell<&'short str>`，短引用就能写进原本承诺为 `'static` 的槽位。

`OnceCell<T>` 和 `LazyCell<T, F>` 也通过 `UnsafeCell` 提供内部写入，因此它们的 `T` 同样不能依靠外层结构协变。

## 4.8 如何选择

| 场景 | 选择 |
|---|---|
| 只需要复制、替换一个 `Copy` 小值 | `Cell<T>` |
| 需要修改非 `Copy` 值但不需要内部引用 | `Cell<T>` + `replace` / `take` |
| 需要长期借用 `T` 的内部字段并动态检查冲突 | `RefCell<T>` |
| 单线程、多 owner 且需要可变内容 | `Rc<RefCell<T>>` |
| 只需要初始化一次 | `OnceCell<T>` |
| 首次使用才初始化 | `LazyCell<T, F>` |
| 实现 unsafe 容器，自己维护别名规则 | `UnsafeCell<T>` |
| 跨线程修改任意值 | `Mutex<T>` / `RwLock<T>` |
| 跨线程一次性初始化 | `OnceLock<T>` / `LazyLock<T, F>` |

一个直接的选择规则：

```text
能不用内部可变性 → 用 &mut T
只需要整体替换   → Cell<T>
需要借出内部引用 → RefCell<T>
需要跨线程       → Mutex/RwLock/原子/OnceLock
需要自己审计底层 → UnsafeCell<T>
```

## 4.9 复现与观察点

```bash
cd code
cargo run --bin ch04_cells
cargo rustc --release --bin ch04_cells -- --emit asm -C debuginfo=0
find target/release/build/rust-book -name ch04_cells.s
```

探针层次：

```text
size_of / align_of / needs_drop
    → 包装成本

UnsafeCell 裸写
    → 没有运行时检查

plain u32 / Cell<u32> 前后读取
    → Freeze 使寄存器复用成为可能；Cell 调用后重新 load

Cell::set / replace / take
    → 整体值交换

RefCell::try_borrow_mut
    → 运行时冲突检测

OnceCell / LazyCell
    → 初始化状态机
```

不要仅根据大小判断语义：

```text
Cell<u8>       与 u8 同大小
→ 没有额外运行时状态

RefCell<u8>    16 字节
→ 有 borrow counter

OnceCell<u8>   2 字节
→ 存的是 UnsafeCell<Option<u8>>

OnceCell<String> 24 字节
→ Option<String> 当前利用 niche，不代表没有初始化状态
```

## 4.10 练习

1. 用 `Cell<u64>` 实现单线程计数器，再用 `AtomicU64` 实现多线程版本，比较共享引用要求和内存序。
2. 用 `RefCell<Vec<u32>>` 制造 shared borrow 与 mutable borrow 冲突，分别测试 `borrow_mut()` 的 panic
   和 `try_borrow_mut()` 的 `Result`。
3. 把 `Ref` guard `mem::forget`，观察后续借用错误；解释为什么这是安全但不可恢复的状态。
4. 用 `OnceCell<String>` 保存一个只计算一次的值，比较 `OnceCell`、`OnceLock` 和 `Mutex<Option<T>>`。
5. 用 `LazyCell` 的初始化闭包 `panic!`，确认后续访问因 `Poisoned` 状态继续 panic。
6. 自己用 `UnsafeCell<Vec<T>>` 写一个最小 `MyCell`，列出它必须维护的别名与 drop 安全条件，
   再与标准库 `Cell` 的 API 对比。
7. 修改 `freeze_read_plain`：在第一次读取后加入 `std::sync::atomic::fence` 或真实的
   FFI 调用，观察第二次 load 是否出现，并解释为什么“某次没优化”不能反推别名规则改变。

## 4.11 小结

| 问题 | 答案 |
|---|---|
| 所有 Cell 的底层基础 | `UnsafeCell<T>` |
| `UnsafeCell` 是否检查别名 | 不检查，调用者负责 unsafe 条件 |
| `Cell<T>` 为什么零开销 | 只做整体值操作，不保存 borrow 状态 |
| `Cell::get` 为什么要求 `Copy` | 避免借出与后续写入冲突的内部引用 |
| `RefCell<T>` 保存什么状态 | 一个有符号 borrow counter，以及内部 `UnsafeCell<T>` |
| `RefCell` 冲突怎么处理 | `borrow_mut()` panic，`try_borrow_mut()` 返回 `Result` |
| `OnceCell` 与 `RefCell` 的区别 | 前者只允许初始化一次，之后主要共享只读 |
| `LazyCell` 多出什么 | 未初始化闭包、初始化完成值和 poisoned 状态 |
| 能否跨线程共享 | `Cell` 家族通常 `!Sync`，使用 sync 对应物 |
| 为什么对 `T` invariant | 能通过共享引用写入 `T` |
| `Freeze` 是什么 | 编译器内部 auto trait：类型是否不穿透间接层地包含 `UnsafeCell` |
| `Freeze` 说明值永远不变吗 | 不；只约束每次共享借用有效期内可修改的字节 |
| `&Cell<T>` 是 `Freeze` 吗 | 是；引用值本身是指针，pointee 仍可内部可变 |
| `UnsafeCell` 是否解除线程安全检查 | 否；裸 `UnsafeCell` 仍 `!Sync`，并发写入仍需同步 |
| 越界写普通 `&T` 会怎样 | 未定义行为；编译器可能按 `T: Freeze` 优化 |
