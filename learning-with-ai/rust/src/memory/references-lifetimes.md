# 第 3 章 引用与生命周期：指针有值，生命周期无表示

> 配套代码：`code/src/bin/ch03_refs.rs`  
> 所有运行时数字均为本机实测（aarch64-apple-darwin，rustc 1.100.0）。

## 3.1 一句话机器模型

Rust 的引用在运行时是**指针或胖指针**；生命周期是编译期的**访问许可约束**。

```text
运行时：&T        → pointer
        &[T]、&str → (pointer, length)
        &dyn Trait → (data pointer, vtable pointer)

编译期：'a        → 引用合法存在的一组程序点
```

生命周期不是字段，不是 tag，也不是运行时 lease 对象。引用布局中没有地方存 `'a`。

实测：

```text
&u32              size = 8
&mut u32          size = 8
&[u32]            size = 16
&str              size = 16
&dyn Debug        size = 16
SliceView<'a>     size = 24  (fat pointer + offset)
PhantomPtr<'a,T>  size = 8  (lifetime 只存在于类型系统)
Option<&u32>      size = 8
```

其中：

```rust
struct SliceView<'a> {
    data: &'a [u32],
    offset: usize,
}

struct PhantomPtr<'a, T> {
    ptr: *const T,
    marker: PhantomData<&'a T>,
}
```

`SliceView` 的 24 字节来自 `(ptr, len) + offset`；`PhantomPtr` 的 8 字节只来自裸指针，`PhantomData<&'a T>` 是 ZST。

## 3.2 生命周期如何被擦除

两个函数：

```rust
#[unsafe(no_mangle)]
#[inline(never)]
pub fn lifetime_identity<'a>(value: &'a u32) -> &'a u32 {
    value
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn lifetime_identity_static(value: &'static u32) -> &'static u32 {
    value
}
```

MIR 中只剩：

```text
fn lifetime_identity(_1: &u32) -> &u32
fn lifetime_identity_static(_1: &u32) -> &u32
```

看不到 `'a` 或 `'static`。

release 汇编中两个符号直接合并：

```asm
_lifetime_identity:
        ret

_lifetime_identity_static = _lifetime_identity
```

函数只需要返回 `x0` 中已有的指针，连额外搬移都不需要。

这说明：

- 生命周期影响类型检查、借用检查、variance 和 drop check；
- 生命周期不进入机器调用 ABI；
- 生命周期擦除后，两个只在 region 上不同的函数签名可以拥有完全相同的 ABI。

但不要把“擦除”理解成“随便标都一样”。擦除发生在 borrow checker 已经证明程序安全之后。

## 3.3 NLL：借用在哪里结束

更早的借用规则可以用词法作用域近似理解；NLL（non-lexical lifetimes）之后，判断依据是引用在控制流中是否仍会被使用。

```rust
fn nll_local(mut value: u32) -> u32 {
    let first = &mut value;
    *first += 1;

    let second = &mut value;
    *second *= 2;

    value
}
```

`first` 的变量作用域还没有结束，但最后一次使用已经结束，因此第二次可变借用合法。

实测：

```text
nll_local(20) = 42
```

对应 MIR：

```text
bb0: {
    _2 = &mut _1;
    ...
}

bb1: {
    (*_2) = ...;
    _4 = &mut _1;
    ...
}
```

两次 `&mut _1` 顺序出现，MIR 中没有“借用计数器”或运行时 lease：

```text
borrow checker 接受程序
→ 生命周期信息被擦除
→ codegen 只看到普通地址运算
```

如果共享借用之后仍要使用，而其间又修改 owner，则冲突：

```rust,compile_fail
fn main() {
    let mut value = 1;
    let borrowed = &value;
    value = 2;
    println!("{borrowed}");
}
```

编译器报告：

```text
error[E0506]: cannot assign to `value` because it is borrowed
```

原因不是“变量还在花括号内”，而是共享借用 `borrowed` 在赋值之后仍有 use。

### Loans 与 liveness

可以用两个集合理解 borrow checker 的核心问题：

```text
活跃引用：哪些引用在这些程序点仍会被使用
活跃 loan：这些引用当前代表对哪个 place 的读/写权限
```

一条 shared loan 与一条 mutable loan 重叠时，是否冲突取决于对应 place 是否重叠以及引用是否仍 live。

NLL 的具体求解算法属于编译器实现细节；Polonius 等新借用模型也表明这部分仍会演进。语言层可依赖的是借用规则和已接受的程序，不是某张特定 region 图。

## 3.4 Reborrow 与 two-phase borrow

`&mut T` 不能 Copy，但可以从已有的可变引用临时 reborrow：

```rust
fn sequential_reborrow(values: &mut Vec<u32>) {
    let first = &mut *values;
    first.push(1);

    let second = &mut *values;
    second.push(2);
}
```

实测：

```text
values = [0, 1, 2]
```

这里不是同一时刻存在两个可用的可变访问权，而是：

```text
first loan 最后一次使用结束
→ 创建 second reborrow
```

MIR 中两个 push 也是顺序调用：

```text
bb0: Vec::push(copy _1, 1)
bb1: Vec::push(copy _1, 2)
```

更微妙的是 two-phase borrow：

```rust
fn two_phase_borrow(values: &mut Vec<usize>) {
    values.push(values.len());
}
```

实测：

```text
values = [10, 20, 2]
```

看起来同一个 `values` 同时需要：

```text
push 的 &mut Vec
len 的 &Vec
```

two-phase borrow 把它拆成：

```text
1. 方法接收者先保留一个 mutable borrow reservation
2. 求值参数时暂时只创建 shared borrow
3. 真正调用方法时，mutable borrow 才激活
4. shared borrow 在激活前结束
```

MIR 清楚展示了这个顺序：

```text
bb0: {
    _4 = &(*_1);
    _3 = Vec::<usize>::len(move _4) -> bb1;
}

bb1: {
    _2 = Vec::<usize>::push(copy _1, move _3) -> bb2;
}
```

two-phase 不是运行时协议，而是 borrow checker 对特定接收者的两阶段借出规则。

## 3.5 生命周期省略不是“自动猜一个任意 lifetime”

Rust Reference 的函数生命周期省略规则可以压缩为：

1. 每个省略的输入 lifetime 都成为独立参数；
2. 如果只有一个输入 lifetime，输出中省略的 lifetime 使用它；
3. 如果有 `&self` 或 `&mut self`，输出使用 receiver 的 lifetime。

例如：

```rust
fn first(values: &[u8]) -> &u8 {
    &values[0]
}
```

等价于：

```rust
fn first<'a>(values: &'a [u8]) -> &'a u8
```

但多个输入引用时：

```rust,compile_fail
fn pick(a: &u32, b: &u32) -> &u32 {
    a
}
```

无法根据省略规则决定返回借用自哪个输入，必须显式写 lifetime 或返回拥有所有权的值。

### `'static` 不等于“对象永生”

`'static` 表示引用可以满足任意长生命周期的要求，不表示 pointee 必然永久存在或永不释放。

常见来源：

- 字符串字面量；
- `static` item；
- `Box::leak`；
- 其他显式获得永久所有权的机制。

## 3.6 Variance：泛型参数中的生命周期子类型

生命周期存在子类型关系：

```text
'static <: 'a      当 'static 比 'a 更长
```

含义是：更长生命周期的引用可以在只需要较短生命周期的地方使用。

Reference 给出三类 variance：

| Variance | 直观含义 |
|---|---|
| covariant | 子类型关系按原方向传递 |
| contravariant | 子类型关系反转 |
| invariant | 不能传递 |

关键内建类型：

| 类型 | 对 `'a` | 对 `T` |
|---|---|---|
| `&'a T` | covariant | covariant |
| `&'a mut T` | covariant | **invariant** |
| `*const T` | | covariant |
| `*mut T` | | **invariant** |
| `fn() -> T` | | covariant |
| `fn(T)` | | contravariant |
| `UnsafeCell<T>` | | **invariant** |

### `&mut` 在 lifetime 上协变

```rust
fn shorten_mut_reference<'long, 'short>(
    value: &'long mut u32,
) -> &'short mut u32
where
    'long: 'short,
{
    value
}
```

较长的可变借用可以缩短成较短借用。实测：

```text
number = 8
```

MIR 中仍然只是：

```text
fn shorten_mut_reference(_1: &mut u32) -> &mut u32
```

### `&mut T` 对 `T` 不变

下面不能编译：

```rust,compile_fail
fn overwrite<'a>(slot: &mut &'a str, value: &'a str) {
    *slot = value;
}

fn main() {
    let mut slot: &'static str = "static";
    let local = String::from("local");
    overwrite(&mut slot, &local);
    println!("{slot}");
}
```

错误是：

```text
error[E0597]: `local` does not live long enough
```

假设 `&mut &'a str` 对 `'a` 协变，就可能把：

```text
&mut &'static str
```

当成：

```text
&mut &'a str
```

传进函数，再把短生命周期的 `&'a str` 写进原本声明为 `'static` 的槽位。函数返回后，owner 仍以为槽中有 `&'static str`，但实际 pointee 已经失效。

因此 `&mut T` 必须对 `T` invariant：

```text
既能读出 T
又能写入 T
→ T 的类型不能单向变化
```

这也是 `Cell<T>`、`UnsafeCell<T>` 对 `T` invariant 的核心原因：它们允许通过共享引用修改内部值。

### 关键对照：只读可缩短，可写不可缩短

同样把 `&'static str` 与一个局部 `&str` 放在一起，只读调用可以成功：

```rust
fn cmp<'a>(s1: &'a str, s2: &'a str) -> bool {
    s1.len() == s2.len()
}

fn main() {
    let slot: &'static str = "static";
    let local = String::from("local");

    cmp(slot, &local);
    println!("{slot}");
}
```

这里 `cmp` 的 `'a` 可以取局部引用的较短 lifetime：

```text
slot:   &'static str  →  &'short str
&local: &'short str   →  &'short str

'a = 'short
```

`&T` 是协变的，且 `cmp` 只读。把长 lifetime 的共享引用缩短为短 lifetime 的共享引用，没有增加任何写权限。

而前面的 `overwrite` 接收：

```text
slot: &mut &'static str
```

第二个 `'static` 位于 `&mut T` 的 `T` 中：

```text
&'borrow mut &'static str
  │           │
  │           └── invariant
  └── covariant
```

外层 `&mut` 的借用时长可以缩短，但内部 pointee 类型 `&'static str` 不能变成 `&'short str`。否则函数就能把局部引用写入一个仍被调用者承诺为 `'static` 的槽位。

区别可以压缩成：

```text
cmp:       复制只读引用，并缩短它    → 安全
overwrite: 获得写入槽位的权限        → 槽位中的 T 必须保持 invariant
```

### 函数参数为什么 contravariant

函数 `fn(T)` 的调用者负责提供 `T`：

```text
能接受更宽参数的函数
可以替代只接受更窄参数的函数
```

所以参数位置是 contravariant，返回值位置是 covariant。函数 item、函数指针和 HRTB 会把这个规则带入更复杂的签名。

## 3.7 生命周期与 Drop

如果类型在析构时可能访问借用数据，那么被借用值必须存活得足够久：

```rust
struct Borrowing<'a> {
    value: &'a String,
}

impl Drop for Borrowing<'_> {
    fn drop(&mut self) {
        println!("{}", self.value.len());
    }
}
```

析构函数在 `Borrowing` drop 时读取 `self.value`，因此 `'a` 不能在被借用对象已经 drop 后继续存在。

这就是 drop check 的边界。NLL 判断普通 use 的 loan liveness；dropck 额外考虑析构代码可能访问什么。`#[may_dangle]` 是标准库内部用于放宽该规则的受控机制，不应在业务代码中模仿。

## 3.8 常见借用错误对应什么

| 错误 | 场景 | 机制 |
|---|---|---|
| `E0506` | 有共享借用仍使用，却修改 owner | shared loan 与 write 重叠 |
| `E0515` | 返回局部变量的引用 | 返回引用会超过 owner 生命周期 |
| `E0597` | 短生命周期值不满足约束 | region inclusion 不成立 |
| `E0499` | 同一时刻两个可用 `&mut` | 两个 mutable loan 重叠 |
| `E0502` | 同一时刻 shared + mutable | 读权限和写权限重叠 |

这些错误发生在编译期，不会留下运行时检查分支。

## 3.9 复现与观察点

```bash
cd code

# 引用布局、NLL、reborrow、two-phase borrow
cargo run --bin ch03_refs

# MIR：生命周期已被擦除
RUSTC="$(rustup which --toolchain nightly rustc)" \
  RUSTUP_TOOLCHAIN=nightly \
  cargo rustc --bin ch03_refs -- -Zunpretty=mir 2>&1 | less

# 各 pass 的 region 约束和 borrow checker 中间结果
RUSTC="$(rustup which --toolchain nightly rustc)" \
  RUSTUP_TOOLCHAIN=nightly \
  cargo rustc --bin ch03_refs -- -Zdump-mir=all
ls mir_dump/

# release 汇编：不同 lifetime 的 identity 符号合并
cargo rustc --release --bin ch03_refs -- --emit asm -C debuginfo=0
rg -n -A5 "lifetime_identity" target/release/build/rust-book/*/out/ch03_refs.s
```

观察时区分：

```text
MIR 里的 loan / region constraint
    → borrow checker 的编译期证明

codegen 后的 &T
    → 普通指针

运行时无 lifetime 字段
    → ABI 不携带生命周期
```

## 3.10 练习

1. 把 `nll_local` 中的第二次借用改成在 `first` 最后一次使用之前，观察 `E0499`。
2. 把 `sequential_reborrow` 改成同时持有两个 `&mut`，再改成一个 `&` 和一个 `&mut`，分别记录错误码。
3. 用 `PhantomData<&'a T>`、`PhantomData<&'a mut T>`、`PhantomData<Cell<&'a T>>`
   构造三个零大小 marker，比较它们对 variance 的影响。
4. 写一个函数返回两个输入引用中较长者的切片，分别尝试省略 lifetime 和显式标注，
   解释哪种能够编译。
5. 给 `Borrowing<'a>` 的析构函数增加对借用数据的访问，再构造 owner 与借用者 drop 顺序相反的例子，
   从错误信息定位 drop check 约束。

## 3.11 小结

| 问题 | 答案 |
|---|---|
| 引用运行时存什么 | 单指针、ptr+len 或 data+vtable |
| lifetime 运行时存什么 | 不存；codegen 前擦除 |
| NLL 看词法作用域吗 | 主要看各程序点上引用是否仍会被使用 |
| reborrow 是什么 | 从已有引用临时构造一个子借用 |
| two-phase borrow 解决什么 | receiver 的 mutable borrow 与其参数求值之间的顺序冲突 |
| `&mut` variance | 对 lifetime covariant，对 pointee type invariant |
| 为什么 `&mut T` 对 `T` invariant | 它允许写入 `T`，单向子类型转换会造成生命周期不安全 |
| `'static` 是什么 | 可满足任意长生命周期要求的约束，不是运行时所有权 |
| 何时需要 PhantomData | 结构体逻辑上借用/拥有类型参数，但布局中没有相应字段 |
| 为什么 Drop 会影响生命周期 | 析构函数可能在 owner drop 时访问借用数据，需要 drop check |
