# 第 1 章 值的布局：字节、对齐与 niche

> 配套代码：`code/src/bin/ch01_layout.rs`、`code/src/bin/ch01_abi.rs`
> 所有数字均为本机实测（aarch64-apple-darwin，rustc 1.100.0）。

## 1.1 为什么先学布局

你有 DuckDB / Arrow 的底子，应该已经形成这个直觉：**性能的账大部分记在"数据怎么摆"上，而不是"语句怎么写"上**。
Rust 把这件事从"程序员的选择"变成了"类型系统的一部分"：

- `Option<&T>` 与 `&T` 同大小，不是语法糖，是布局性质；
- `Vec<T>` 一定是 3 个字，`dyn Trait` 一定是 2 个字；
- `enum` 的大小由"最大 variant + tag 是否能塞进 padding"决定，而不是简单的加法；
- `Pin` 存在的唯一理由，是某些类型的不变量依赖于**地址**——而地址是布局的一部分。

同时，布局也是 UB 的高发区：`transmute`、FFI、`repr(packed)`、手工算偏移。
所以这一章就是两条线的交汇点：**语言语义 ↔ 机器语义**。

## 1.2 size 与 align

两个基础量：

- `size_of::<T>()`：一个 T 占多少字节。硬约束：**size 一定是 align 的整数倍**。
- `align_of::<T>()`：合法地址必须满足 `addr % align == 0`。

为什么 size 必须是 align 的倍数？因为数组元素的地址是

$$
\mathrm{addr}(a[i]) = \mathrm{base} + i \times \mathrm{size}(T)
$$

如果 `size` 不是 `align` 的倍数，那么当 `base` 对齐时，`a[1]` 就可能不对齐，而 `&a[1]` 必须是合法的 `&T`。
这条推导是所有"布局规则"的来源。

实测（`cargo run --bin ch01_layout`）：

| 类型 | size | align | 备注 |
|---|---:|---:|---|
| `u8` | 1 | 1 | |
| `u32` | 4 | 4 | |
| `u64` | 8 | 8 | |
| `u128` | 16 | 16 | aarch64 上有原生 128 位对齐要求 |
| `bool` | 1 | 1 | **不是 1 bit** |
| `char` | 4 | 4 | 直接存 Unicode scalar value，与 UTF-8 无关 |
| `usize` | 8 | 8 | 只保证"能放下指针"，不保证 32 位 |

`char` 是 4 字节这件事值得留意：Rust 的 `char` 是"一个字符值"，不是一个 UTF-8 字节序列。
UTF-8 编码只出现在 `str` / `String` 的存储层，`char` 与 `u32` 的值域几乎相同（去掉代理区与越界值）。

## 1.3 结构体：`repr(Rust)` 会重排字段

```rust
struct Reordered { a: u8, b: u32, c: u8 }

#[repr(C)]
struct PaddedC { a: u8, b: u32, c: u8 }
```

实测偏移：

| 类型 | size | align | a | b | c |
|---|---:|---:|---:|---:|---:|
| `Reordered` | 8 | 4 | **4** | **0** | 5 |
| `PaddedC` | 12 | 4 | 0 | 4 | 8 |

`Reordered` 的字节图（`repr(Rust)` 实际排布）：

```text
offset  0    1    2    3    4    5    6    7
      +----+----+----+----+----+----+----+----+
      | b0 | b1 | b2 | b3 | a  | c  |pad |pad |
      +----+----+----+----+----+----+----+----+
```

`PaddedC` 的字节图（`repr(C)`，保持声明顺序）：

```text
offset  0    1    2    3    4    5    6    7    8    9   10   11
      +----+----+----+----+----+----+----+----+----+----+----+----+
      | a  |pad |pad |pad | b0 | b1 | b2 | b3 | c  |pad |pad |pad |
      +----+----+----+----+----+----+----+----+----+----+----+----+
```

`rustc -Zprint-type-sizes` 的输出直接印证了这种重排（注意字段被重新排序、尾部有 end padding）：

```text
print-type-size type: `Reordered`: 8 bytes, alignment: 4 bytes
print-type-size     field `.b`: 4 bytes
print-type-size     field `.a`: 1 bytes
print-type-size     field `.c`: 1 bytes
print-type-size     end padding: 2 bytes
```

四条推论：

1. **不要依赖 `repr(Rust)` 的字段顺序**。想稳定就得写 `#[repr(C)]`。
2. **字段顺序影响体积**。同一组字段，8 字节 vs 12 字节；在数组/heap 上会被放大 N 倍。
   实测的排布策略是"按对齐从大到小"，但这属于实现细节，不是契约。
3. `#[repr(transparent)]`：只有一个非零大小字段的 newtype 与内层**布局与 ABI 完全相同**。
   这是给 `u64` 包一层类型安全外衣的正当做法（`struct UserId(u64)`）。
4. `#[repr(align(N))]` 提高对齐（缓存行、SIMD、原子操作要求），
   `#[repr(packed)]` 降低对齐——但要记住：**对 packed 字段取引用会产生未对齐引用**，
   现代 Rust 直接报 `E0793`，你只能用 `read_unaligned` / 按值拷贝。

什么时候必须固定布局？不是"性能"，而是**边界**：

- FFI（`repr(C)`，或用 `bindgen` 生成的类型）；
- 磁盘格式 / mmap / 共享内存（跨进程、跨版本）；
- 对结构体字节做哈希、校验、序列化；
- 需要把 `&T` 当 `&[u8]` 用时。

## 1.4 枚举：tag + payload，但 tag 可能免费

```rust
enum Three { A, B, C }
enum Mixed { A, B(u32) }
enum Payload { A(u32), B(u64) }
enum NotReprC { A(u8, u8, u8), B }
```

实测：

| 类型 | size | align |
|---|---:|---:|
| `Three` | 1 | 1 |
| `Mixed` | 8 | 4 |
| `Payload` | 16 | 8 |
| `NotReprC` | **4** | **1** |

对应编译器报告：

```text
print-type-size type: `Three`: 1 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes

print-type-size type: `Mixed`: 8 bytes, alignment: 4 bytes
print-type-size     discriminant: 4 bytes
print-type-size     variant `B`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `A`: 0 bytes

print-type-size type: `Payload`: 16 bytes, alignment: 8 bytes
print-type-size     discriminant: 4 bytes
print-type-size     variant `B`: 12 bytes
print-type-size         padding: 4 bytes
print-type-size         field `.0`: 8 bytes, alignment: 8 bytes

print-type-size type: `NotReprC`: 4 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `A`: 3 bytes
```

三个反直觉的点：

1. **tag 宽度不只看 variant 数**。`Mixed` 只有 2 个 variant，1 字节就够，实测 discriminant 占 4 字节——
   这是为了让 tag 与 payload 的对齐"合拍"（省去拆字节的指令）。
2. **enum 的 align 是字段最大 align**。`NotReprC` 的 payload 全是 `u8`，所以 align = 1，整个 enum 只有 4 字节。
3. **tag 会被塞进 padding**。`Payload::B(u64)` 的 variant 区是 12 字节（4 字节 padding + 8 字节值），
   而 tag 只需要 4 字节——正好占用 `A(u32)` 之外的"槽位"。

这就是为什么"`enum` 大小 = 最大 variant + tag"是错的：真正的规则是
**"所有 variant 必须落在同一块内存里，tag 在能塞下的地方塞"**。

与 C 对照：C 里你要手写 `struct { int tag; union { ... } }`，Rust 的 `enum` 语义上是 tagged union，
布局上不承诺任何具体形状——只有 `#[repr(C)]` / `#[repr(u8)]` 之类才会固定下来。

## 1.5 niche optimization：Nil 也可以是"值域里的空位"

核心机制一句话：**如果某类型有一些"合法但不可能被构造出来"的位模式（niche），
就可以拿其中一个表示 `None`，于是 `Option<T>` 不需要额外 tag。**

实测：

| 类型 | size | 说明 |
|---|---:|---|
| `Option<&u8>` | 8 | 空指针 = None |
| `Option<Box<u8>>` | 8 | 同上 |
| `Option<fn()>` | 8 | 函数指针非空 |
| `Option<NonZeroU32>` | 4 | 0 = None |
| `Option<NonZeroNewtype>` | 4 | newtype **传递** niche |
| `Option<bool>` | 1 | bool 值域 {0,1}，2 可用作 niche |
| `Option<Option<NonZeroU32>>` | **8** | 内层已用掉 0，外层无处安放 |
| `Option<NotReprC>` | 4 | 用 tag 字节的剩余值（3..=255） |
| `Option<Vec<u8>>` | 24 | 与 `Vec` 同大小；报告里**没有** discriminant |
| `Option<Option<Vec<u8>>>` | **24** | 当前实现的 `RawVec` 仍有多个可用 niche |

`Option<Option<NonZeroU32>>` 是最有教学价值的一例：

```text
print-type-size type: `std::option::Option<std::option::Option<std::num::NonZero<u32>>>`: 8 bytes
print-type-size     discriminant: 4 bytes
print-type-size     variant `Some`: 4 bytes
```

`NonZeroU32` 只有一个空位（0），被内层的 `None` 用掉；外层 `None` 只能再加 4 字节 discriminant。
所以更准确的说法是：**能否省掉 tag，取决于内层类型还剩多少种可编码状态；不能用"嵌套 Option 一定变大/不变"一概而论。**
例如本机实测 `Option<Option<Vec<u8>>>` 仍是 24 字节，因为当前的 `RawVec` 提供了多个 niche。

再看 `Option<Vec<u8>>`：

```text
print-type-size type: `std::option::Option<std::vec::Vec<u8>>`: 24 bytes, alignment: 8 bytes
print-type-size     variant `Some`: 24 bytes
print-type-size     variant `None`: 0 bytes
```

没有 discriminant 行 → 用了内层类型提供的 niche。当前 std 源码里，`RawVecInner` 的形状是：

```rust
type Cap = core::num::niche_types::UsizeNoHighBit;

struct RawVecInner<A: Allocator = Global> {
    ptr: NonNull<u8>,
    cap: Cap,
    alloc: A,
}
```

`Cap` 的合法范围是 `0..=isize::MAX`，最高位恒为 0；编译器用其非法编码表示 `None`。
这是**当前实现细节**，不是 `Option<Vec<T>>` 的稳定保证。注意这里 24 == 24，
**"大小没变"既可能是没优化，也可能是优化得太好**，判断依据只能是编译器报告和源码。

保证与不保证，必须分清：

| 情形 | 是否保证同大小 |
|---|---|
| `Option<&T>` / `Option<&mut T>` / `Option<Box<T>>` / `Option<NonNull<T>>` / `Option<fn()>` | **保证**（stdlib 文档明确承诺） |
| `Option<NonZeroXxx>` | **保证**（`NonZero` 文档明确承诺） |
| `Option<bool>` / `Option<char>` / `Option<enum>` / 嵌套 `Option` | 不保证，只是当前实现的优化 |

工程含义：

- 结构体里放 `Option<&T>` 不花额外空间，放 `Option<u32>` 会花 8 字节（4 字节 payload + 4 字节 tag）。
  想让 `Option<u32>` 变成 4 字节，就换 `NonZeroU32` + 用 0 之外的值表示"无效"。
- 自定义 newtype 只要能保住内层的 niche（不引入其他字段、不加 `repr` 扰动），niche 就能穿过去。
- `Vec`/`String` 做 `Option` 时不要为了"省 4 字节"去做手工编码；先测量，再决定。

## 1.6 胖指针与容器：std 的基本形状

| 类型 | size | 内部形状 |
|---|---:|---|
| `&[u8]` / `&str` | 16 | ptr + len |
| `Box<[u8]>` | 16 | 独有的 ptr + len |
| `Option<Box<[u8]>>` | 16 | 同上，ptr 做 niche |
| `Vec<u8>` | 24 | `RawVecInner{cap, ptr}`(16) + `len`(8) |
| `String` | 24 | 内部就是一个 `Vec<u8>` |
| `Rc<String>` | 8 | 一个非空指针 |
| `RcInner<String>` | 40 | `strong`(8) + `weak`(8) + `value`(24) |

报告里的关键几行：

```text
print-type-size type: `std::vec::Vec<u8>`: 24 bytes, alignment: 8 bytes
print-type-size     field `.buf`: 16 bytes
print-type-size     field `.len`: 8 bytes

print-type-size type: `alloc::raw_vec::RawVecInner`: 16 bytes, alignment: 8 bytes
print-type-size     field `.cap`: 8 bytes
print-type-size     field `.ptr`: 8 bytes

print-type-size type: `std::string::String`: 24 bytes, alignment: 8 bytes
print-type-size     field `.vec`: 24 bytes

print-type-size type: `RcInner<std::string::String>`: 40 bytes, alignment: 8 bytes
print-type-size     field `.value`: 24 bytes
print-type-size     field `.strong`: 8 bytes
print-type-size     field `.weak`: 8 bytes
```

几个直接对得上你已有笔记的事实：

- `RcInner` 的 40 字节与你 `rust-rc-layout` 笔记里的结论一致；`Rc` 本身只是一个指针，
  所以 `Rc::clone` 是 `strong += 1`，不是深拷贝。
- `String` 是 `Vec<u8>` 的**零成本**包装（`repr(transparent)` 级别的布局等价），
  所以 `String → Vec<u8>`（`into_bytes`）不搬数据。
- `Box<[u8]>` 与 `Vec<u8>` 的差别不是"省了 8 字节"，而是**丢掉了 capacity**：
  不可变、不可 push，换来的是 `Option<Box<[u8]>>` 也能 16 字节。

与 Arrow 的连接：`ArrayData` 用 `ptr + len + offset + nulls`（外加 buffer 所有权表），
而不是 `Vec<Option<T>>`，本质原因就在这里——`Vec<Option<u32>>` 每个元素 8 字节，
而 `Vec<u32>` + bitmap 是 4 字节 + 0.125 字节/元素。
**"null 是列级属性，不是值级属性"** 这个 Arrow 设计决策，在 Rust 类型系统里的对偶就是
`Option<Vec<T>>`（整个数组可空）vs `Vec<Option<T>>`（每个元素可空）。

## 1.7 布局决定 ABI

`code/src/bin/ch01_abi.rs` 里四个函数，`--emit asm` 后的 aarch64 汇编：

```asm
_take_pair:                 ; fn((u64, u64)) -> u64
        add     x0, x1, x0
        ret

_take_triple:               ; fn((u64, u64, u64)) -> u64
        ldp     x8, x9, [x0]   ; x0 是调用者传入的隐藏指针
        ldr     x10, [x0, #16]
        add     x8, x9, x8
        add     x0, x8, x10
        ret

_take_opt_ref:              ; fn(Option<&u8>) -> u64
        cbz     x0, LBB5_2     ; 判空 = 判 None
        ldrb    w0, [x0]
        ret
LBB5_2:
        mov     x0, #-1
        ret

_take_opt_u32:              ; fn(Option<NonZeroU32>) -> u64
        mov     w0, w0         ; 整个 match 消失了
        ret
```

四个结论：

1. **≤ 16 字节的聚合值走寄存器**（`x0`/`x1`）；更大的走隐藏返回/入参指针（sret）。
   所以"返回一个大结构体"不等于"必然拷贝"，但也不等于免费——要看优化器能否把它拆开。
2. `Option<&T>` 在 ABI 层面与 `&T` **完全一致**，判空就是一条 `cbz`。
3. `Option<NonZeroU32>` 的 `match` 编译成了 `mov w0, w0`：
   因为 `None` 的编码（0）恰好等于 `None` 分支要返回的 0，整段分支被折叠掉了。
   这是 niche 优化在机器码层面的直接兑现。
4. **布局会影响 ABI，ABI 会影响微基准**。把 `(bool, u64)` 与 `(u64, bool)` 放在一起比性能，
   你测到的可能是寄存器分配差异，而不是算法差异。

## 1.8 自己验证：本章所有结论的复现方法

```bash
# 1) 运行时探针
cd code && cargo run --bin ch01_layout

# 2) 编译器布局报告（nightly；必须触发重新编译）
touch src/bin/ch01_layout.rs
cargo +nightly rustc --bin ch01_layout -- -Zprint-type-sizes 2>&1 | less

# 3) 汇编（stable 即可）
cargo rustc --release --bin ch01_abi -- --emit asm -C debuginfo=0
# 产物不在 target/release/deps/，而在：
ls target/release/build/rust-book/*/out/ch01_abi.s

# 4) 未对齐/别名/越界检查（后续 unsafe 章节会用到）
cargo +nightly miri run --bin ch01_layout
```

> 提醒：本章所有布局细节都是 **实现细节**（除 std 明确承诺的 niche 情形）。
> 写进代码的应该是"测量"，不是"假设"。跨版本、跨 target、跨 `-C target-cpu` 都可能变。

## 1.9 练习

1. 先预测再验证：`struct S { a: u8, b: u16, c: u8, d: u32 }` 的 `size`/`align`，
   以及加 `#[repr(C)]` 后的 `size`。用 `offset_of!` 解释差出来的字节在哪。
2. 构造 `enum Ip { V4([u8; 4]), V6([u8; 16]) }`，预测大小；再与 `std::net::IpAddr` 比较。
   如果不一样，从 `-Zprint-type-sizes` 里找出原因。
3. 找出 std 中 3 个"保证 niche 优化"的类型，和 1 个"看起来应该有、实际没有"的类型。
4. 把 `struct Header { version: u8, payload: Vec<u8> }` 改成 `payload: Box<[u8]>`，
   说明 size 变化、`Option` 变化，以及你为此放弃了什么能力。
5. 写一个 64 字节对齐的结构体数组，两个线程分别写相邻元素，观察耗时；
   再把 `#[repr(align(64))]` 去掉对比。（伪共享的最小验证）

## 1.10 小结

| 问题 | 答案 |
|---|---|
| `size` 和 `align` 的关系 | `size % align == 0`，否则数组无法保证元素对齐 |
| `repr(Rust)` 字段顺序 | 不稳定，实测按对齐重排以省 padding |
| `enum` 大小 | tag + payload 合一，tag 常被塞进 padding 或 payload 空位 |
| `Option<T>` 何时零开销 | 存在可用的 niche；只有部分情形被 std 承诺 |
| 嵌套 `Option` 是否同大小 | 看内层还剩多少可编码状态；例如 `NonZeroU32` 只有一个空位，`RawVec` 有多个 |
| 容器大小 | `Vec`/`String` 24，切片/`Box<[T]>` 16，`Rc`/`Box` 8 |
| 布局为何重要 | 它同时决定内存占用、ABI、以及哪些不变量能成立 |

下一章把布局和"移动"接起来：**移动 = memcpy**，而正是这条等式逼出了 `Pin`。
