# 第 5 章 堆上的所有权：Box、Vec 与 String

> 配套代码：`code/src/bin/ch05_heap.rs`  
> 所有运行时数字均为本机实测（aarch64-apple-darwin，rustc 1.100.0）。

## 5.1 堆上所有权解决什么问题

前三章的所有权主要发生在栈或寄存器里的值上：

```rust
let value = 42_u64;
let moved = value;
```

一旦值需要动态大小，就会变成另一种机器模型：

```text
栈上的句柄
    │
    ├─ 指向堆分配
    ├─ 记录长度
    └─ 负责释放
```

`Box<T>`、`Vec<T>` 和 `String` 都拥有这种句柄。**移动它们时，默认移动的是句柄，
不是句柄所管理的堆数据。**

这一章要分清三件事：

1. 哪些字节在栈上，哪些字节在堆上；
2. 长度、容量和初始化范围分别约束什么；
3. 扩容、重新分配和 unsafe 操作会怎样破坏指针与所有权不变量。

## 5.2 Box：只拥有一个 pointee

当前 `Box` 的定义是：

```rust
pub struct Box<T: ?Sized, A: Allocator = Global>(
    Unique<T>,
    A,
);
```

对 `T: Sized`，标准库保证 `Box<T>` 表示为单个指针，并且 ABI 与 C 指针兼容。
但对 `T: ?Sized`，它可能是一个胖指针：

```text
Box<u64>     → 普通指针：一个地址
Box<[u64]>   → 胖指针：地址 + 元素个数
Box<str>     → 胖指针：地址 + 字节长度
```

实测：

| 类型 | size | align |
|---|---:|---:|
| `Box<u64>` | 8 | 8 |
| `Box<[u64]>` | 16 | 8 |
| `Option<Box<u64>>` | 8 | 8 |

因此“`Box` 就是一个指针”只对 sized pointee 成立。`Box<[T]>` 与 `&[T]` 一样，
还需要保存长度 metadata。

### 移动 Box

```rust
let original = Box::new([7_u64; 4]);
let before = original.as_ptr() as usize;
let moved = original;
let after = moved.as_ptr() as usize;
```

实测：

```text
same pointee after move = true, value[3] = 7
```

移动 `Box` 没有复制堆上的四个 `u64`，只是把所有权指针交给新位置。
旧变量此后不能再使用，否则借用检查器会报错。

### drop Box

`Box` 的析构顺序是：

```text
1. 递归 drop pointee T
2. Box 的 Drop 实现使用原分配 layout 释放内存
```

std 源码中 `Box::drop` 的注释明确写道：

```text
the T in the Box is dropped by the compiler before the destructor is run
```

探针：

```rust
{
    let _boxed_drop = Box::new(DropSpy { name: "boxed value" });
    println!("before end of box scope");
}
println!("after end of box scope");
```

实测：

```text
before end of box scope
drop boxed value
after end of box scope
```

ZST 是特殊情况：`Box::<()>::new(())` 不需要真实堆分配。零大小分配的指针必须对齐且非空，
但 Box 不会为 ZST 调用 `deallocate`。

## 5.3 Vec：指针、长度、容量

`Vec<T>` 的所有权可以用三个量描述：

```text
ptr  → 缓冲区起始地址
len  → 已初始化的元素个数
cap  → 不重新分配时最多容纳的元素个数
```

当前 `Vec` 的源码形状是：

```rust
pub struct Vec<T, A: Allocator = Global> {
    buf: RawVec<T, A>,
    len: usize,
}

pub(crate) struct RawVec<T, A: Allocator = Global> {
    inner: RawVecInner<A>,
    _marker: PhantomData<T>,
}

struct RawVecInner<A: Allocator = Global> {
    ptr: NonNull<u8>,
    cap: Cap,
    alloc: A,
}
```

对于本机的 `Global` allocator 和普通 sized `T`，可画成：

```text
Vec<T> on stack
+-------------+------+------+
| ptr         | cap  | len  |
+-------------+------+------+
       |
       v
heap: [ T0 ][ T1 ][ T2 ][ spare uninitialized slots ... ]
       \_____________/     \___________________________/
          initialized                 spare capacity
            len elements               cap - len
```

实测布局：

| 类型 | size | align |
|---|---:|---:|
| `Vec<u8>` | 24 | 8 |
| `String` | 24 | 8 |
| `Option<Vec<u8>>` | 24 | 8 |
| `Option<String>` | 24 | 8 |

`Vec` 在 64 位平台正好是三个 machine word。具体字段顺序和 `RawVec` 的内部形状属于
实现细节，不是跨版本 ABI。

### Vec 的核心不变量

对非 ZST、当前 `Global` allocator，可以近似写成：

$$
0 \le \mathrm{len} \le \mathrm{cap}
$$

$$
\mathrm{allocation\_bytes} \ge \mathrm{cap} \times \mathrm{size\_of}(T)
$$

更重要的是初始化边界：

```text
ptr[0..len]     必须都是有效、已初始化的 T
ptr[len..cap]   可以作为未初始化 scratch space
ptr[cap..]      不属于这个 Vec
```

第 `i` 个元素的地址由布局公式决定：

$$
\mathrm{addr}(v_i) = \mathrm{ptr} + i \times \mathrm{size\_of}(T)
$$

但只有 `i < len` 时，安全 API 才能把它当作一个有效 `T`。

## 5.4 空 Vec 与 ZST

### 空 Vec 不分配

```rust
let values = Vec::<u8>::new();
```

`Vec::new()` 的文档保证：在 push 元素之前不会分配。

实测：

```text
new          ptr=0x1 len= 0 cap= 0
```

`ptr=0x1` 不是有效堆地址，而是满足 `u8` 对齐要求的 dangling non-null pointer。
`cap == 0` 时不得解引用它，也不得通过它读写。

当前 `Global` allocator 的空 `Vec<u8>` 使用 `0x1`，这条具体数值属于实现细节；
稳定契约是“不分配、不会解引用这个指针”。

### with_capacity 至少分配请求容量

```rust
let exact = Vec::<u64>::with_capacity(3);
```

实测：

```text
Vec<u64>::with_capacity(3): len=0, cap=3
```

文档保证 `capacity >= requested`，但允许分配更多。当前本机实测恰好等于 3，
不能把这个结果写成所有 allocator 的保证。

### ZST 不需要缓冲区

```rust
let mut zst = Vec::<()>::new();
zst.extend(std::iter::repeat_n((), 1024));
```

实测：

```text
Vec<()>: size=0 len=1024 cap=18446744073709551615 ptr_unchanged=true
```

`usize::MAX` 是 64 位平台上的 `18_446_744_073_709_551_615`。

零大小类型没有字节可分配，因此：

```text
capacity = usize::MAX
push 只增加 len
pointer 始终是某个对齐的 dangling 地址
```

只有长度增长到 `usize::MAX` 时才会遇到容量溢出。

## 5.5 扩容：容量增长与地址失效

`Vec::push` 的核心路径是：

```rust
if len == capacity {
    grow_one();
}

write(ptr.add(len), value);
len += 1;
```

当前 `RawVec` 的摊还增长策略是：

```rust
let cap = cmp::max(self.cap * 2, required_cap);
```

还会设置一个最小非零容量。对 1 字节元素，源码当前选择 8：

```rust
const fn min_non_zero_cap(size: usize) -> usize {
    if size == 1 {
        8
    } else if size <= 1024 {
        4
    } else {
        1
    }
}
```

本机从空 `Vec<u8>` 开始 push，容量变化为：

| push 后 len | capacity |
|---:|---:|
| 0 | 0 |
| 1 | 8 |
| 9 | 16 |
| 17 | 32 |
| 20 | 32 |

完整输出：

```text
new          ptr=0x1 len= 0 cap= 0
grew         ptr=0x1026d8d40 len= 1 cap= 8
grew         ptr=0x1026d8d40 len= 9 cap=16
grew         ptr=0x1026da5b0 len=17 cap=32
final        ptr=0x1026da5b0 len=20 cap=32
```

这里覆盖两个不同层面：

```text
摊还 O(1)   → push 的复杂度结论
8/16/32     → 当前实现的增长序列
```

增长倍率和最小容量都不是语言层面的保证。可以依赖 `reserve` 后容量足够，
不能依赖下一次扩容一定是两倍。

### 为什么总体仍接近 O(1)

如果用倍增策略，扩容时的复制量近似为：

$$
1 + 2 + 4 + \dots + \frac{n}{2} < n
$$

把 n 次 push 的总复制成本平摊后：

$$
\frac{O(n)}{n} = O(1)
$$

单次 push 仍可能是 $O(n)$，因为扩容要复制已有元素；摊还分析把这次成本分散到了
之前的容量空位。

想避免关键路径上的突发复制，可以预先：

```rust
let mut values = Vec::with_capacity(expected_len);
// 或
values.reserve(additional);
```

本机探针从 `cap=1, len=1` 调用：

```rust
values.reserve(4096);
```

得到：

```text
after reserve(4096): cap=4097, pointer_changed=true
```

容量至少为：

$$
\mathrm{len} + \mathrm{additional} = 1 + 4096 = 4097
$$

`reserve_exact` 表达更小的过分配倾向，但 allocator 仍可提供更多空间。

### 扩容会使指向元素的指针失效

安全 Rust 会阻止这种代码：

```rust
// compile_fail
let mut values = vec![1, 2, 3];
let first = &values[0];
values.push(4);
println!("{first}");
```

`push` 需要 `&mut Vec<T>`，而 `first` 仍持有内部元素的共享借用，因此借用检查失败。

如果通过裸指针绕开借用检查：

```text
old_ptr = values.as_ptr()
push 导致 realloc
继续解引用 old_ptr
```

旧指针可能指向已经释放的旧分配，属于未定义行为。allocator 可能原地扩展，也可能移动整个缓冲区；
“这次地址没变”不能成为安全依据。

### Vec 转 Box<[T]>

如果 `len == capacity`，`Vec<T>` 与 `Box<[T]>` 可以复用同一块分配：

```rust
let mut values = Vec::<u64>::with_capacity(3);
values.extend([1, 2, 3]);
let before = values.as_ptr();
let boxed = values.into_boxed_slice();
```

实测：

```text
Vec(len=cap=3) -> Box<[u64]>: same allocation=true, len=3
```

这是所有权句柄从 `Vec` 转成 `Box<[T]>`，堆内容没有复制。

## 5.6 len 是承诺，不只是计数器

`Vec::len` 告诉编译器和库：

> 前 `len` 个槽位都已经初始化，drop 时需要对它们逐一执行 drop。

因此 `set_len` 是 low-level 操作：

```rust
pub const unsafe fn set_len(&mut self, new_len: usize)
```

安全条件至少包括：

1. `new_len <= capacity`；
2. `old_len..new_len` 范围内的元素已经初始化。

正确用法是先通过 `MaybeUninit` 写入，再提升长度：

```rust
let mut values = Vec::<u32>::with_capacity(4);

{
    let spare = values.spare_capacity_mut();
    spare[0].write(11);
    spare[1].write(22);
    spare[2].write(33);
}

unsafe {
    values.set_len(3);
}
```

实测：

```text
after explicit initialization: values=[11, 22, 33] len=3 cap=4
```

三种典型错误：

| 操作 | 后果 |
|---|---|
| 未初始化就提高 `len` | 后续读取未初始化内存，或 drop 非法值，UB |
| 只降低 `len` 而不先 drop 元素 | 资源泄漏；其安全性取决于元素类型 |
| 把 `len` 设置到 `capacity` 以上 | 破坏 Vec 不变量，UB |

`Vec` 不会主动清零 spare capacity，也不会在 `set_len(0)` 时替你 drop 被隐藏的元素。
它的内存只是 buffer，不是自动管理生命周期的安全影子。

同理，`Vec::from_raw_parts` 接收的是完整所有权转移：

```text
谁交出 raw pointer
→ 谁放弃释放这块分配的权利

Vec 接过来
→ 之后由 Vec 负责 realloc/drop/deallocate
```

指针来源、len、capacity、allocator 和对齐必须全部匹配。

## 5.7 Vec 的 drop

当前 `Vec::drop` 的实现核心是：

```rust
self.as_mut_ptr()
    .cast_slice(self.len)
    .drop_in_place();
```

也就是：

```text
1. drop 前 len 个已初始化元素
2. RawVec drop 释放底层分配
```

探针输出：

```text
before Vec drop
drop element 0
drop element 1
drop element 2
after Vec drop
```

但标准库文档明确说：

> Currently, Vec does not guarantee the order in which elements are dropped.

所以这段输出只说明当前实现的顺序，不能作为程序依赖的契约。

如果 `set_len` 把真实元素从长度中隐藏，drop 就看不到它们：

```rust
let mut values = vec![
    String::from("a"),
    String::from("b"),
];

// 安全，但泄漏两个 String
unsafe {
    values.set_len(0);
}
```

`set_len(0)` 本身没有越界，但撤掉了 drop 责任，造成资源泄漏。安全代码应使用
`clear()`，它会先 drop 元素，再把 `len` 设为 0。

## 5.8 String：带 UTF-8 不变量的 `Vec<u8>`

当前 `String` 的定义非常直接：

```rust
pub struct String {
    vec: Vec<u8>,
}
```

实测两者布局：

```text
String size=24 align=8
Vec<u8> size=24 align=8
```

但 `String` 不只是 `Vec<u8>` 的类型别名，它增加了不变量：

```text
vec 中前 len 个字节必须构成合法 UTF-8
```

### 字节长度与字符数量不是一回事

```rust
let text = String::from("A💖");
```

实测：

```text
text="A💖" bytes=5 chars=2 cap=5 byte_len(u32)=4
bytes = [65, 240, 159, 146, 150]
chars = ['A', '💖']
```

机器上：

```text
A   → 1 个 UTF-8 字节: 65
💖  → 4 个 UTF-8 字节: 240,159,146,150
```

内存中的 `char` 通常固定 4 字节，但 `String` 不存储 `[char]`；它存储 UTF-8 字节。
因此：

```text
String::len()      = 字节数，O(1)
String::chars()    = Unicode scalar 序列，需要扫描，O(n)
char::len_utf8()   = 该字符的 UTF-8 字节数
```

`String` 不能用单个 `usize` 直接索引：

```rust
// 编译失败：String 没有 Index<usize>
let first = text[0];
```

字符串切片使用字节范围：

```rust
let heart = &text[1..5];
```

但范围起点和终点必须落在 UTF-8 character boundary 上。切在 `💖` 的中间字节会 panic。

### 安全转换与 unsafe 转换

安全构造会验证 UTF-8：

```rust
let invalid = vec![b'h', 0xff];
let result = String::from_utf8(invalid);
```

实测：

```text
invalid UTF-8 rejected: valid_up_to=1, original_len=2
lossy conversion = "h�"
```

`from_utf8_unchecked` 跳过检查：

```rust
pub unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> String
```

前置条件不是“字符串看起来还能用”，而是：

```text
bytes 必须是合法 UTF-8
```

一旦违反，`String` 的抽象不变量就被破坏。标准库后续方法可能根据“这是合法 UTF-8”
做 unchecked 扫描和切片，因此后果可能是内存安全问题，而不仅是乱码。

`UTF-8 合法` 也不等于 Unicode 规范化：

```text
e + combining acute
    与
é
```

可能不同字节、不同长度，但都合法。`String` 只保证编码合法，不做 normalization。

## 5.9 性能模型：连续存储，但不是魔法

`Vec<T>` 的核心优势是：

```text
元素连续存储
按公式直接计算地址
顺序遍历对 cache 友好
对迭代器和 SIMD 友好
```

代价是：

```text
尾部插入摊还 O(1)
中间插入/删除 O(n)
扩容可能复制全部元素
地址只在当前分配有效
```

几个工程判断：

| 场景 | 选择 |
|---|---|
| 已知大致规模 | `Vec::with_capacity(n)` 或 `reserve` |
| 大小基本固定，后续只读 | `Box<[T]>` / `Vec` 后 `into_boxed_slice` |
| 大量小对象 | 先测布局和间接层，不要假设 `Vec<Vec<T>>` 足够局部 |
| 二维数据 | 优先考虑扁平 buffer + shape/offset，而不是层层 `Vec` |
| 字节串且必须 UTF-8 | `String` |
| 任意二进制字节 | `Vec<u8>` / `&[u8]` |
| 零大小元素 | 直接理解 `Vec<()>` 只保存长度 |

一个常见误区是把 `String::chars().count()` 当成 O(1)。它不是；`len()` 才是 O(1) 的字节数。
另一个误区是认为 `reserve` 会精确分配请求容量。它只保证“至少足够”，实际策略由实现和 allocator 决定。

## 5.10 复现与观察点

```bash
cd code
cargo run --bin ch05_heap
```

探针覆盖：

```text
Box<T> / Box<[T]> / Vec<T> / String 的 size/align
Box 移动前后的 pointee 地址
空 Vec 的 ptr/len/cap
push 触发扩容时的容量序列
realloc 前后缓冲区地址
ZST 的 cap 与地址
spare_capacity_mut + set_len
String 的字节、字符与非法 UTF-8
Vec drop 的当前观察顺序
```

布局报告可以进一步验证：

```bash
touch src/bin/ch05_heap.rs
cargo +nightly rustc --bin ch05_heap -- -Zprint-type-sizes 2>&1 | less
```

报告中重点看：

```text
Vec / RawVec / RawVecInner 的嵌套字段
Box<[T]> 或 String 的 metadata
Option<Vec<T>> 的 niche
spare capacity 是否存在额外状态
```

## 5.11 练习

1. 分别打印 `Vec<u8>`、`Vec<u64>`、`String` 从空开始的容量增长，比较当前
   `min_non_zero_cap` 对不同元素大小的选择。
2. 对比 `reserve(n)` 与 `reserve_exact(n)` 后的 `capacity`，并解释 allocator 为什么可以
   给出大于请求值的容量。
3. 用 `spare_capacity_mut()` 初始化三个元素，再用 `set_len(3)` 构造合法 Vec。
   如果把写入数量改成两个却调用 `set_len(3)`，用 Miri 观察失败。
4. 对 `"A💖"` 打印 `len()`、`chars().count()`、`char_indices()`，并测试哪些字节区间
   可以作为字符串 slice。
5. 比较 `Vec<T>::into_boxed_slice()` 在 `len < cap` 与 `len == cap` 时的地址变化。
6. 写一个带 `Drop` 的元素类型，观察当前 Vec 的析构顺序；再查当前 std 文档，说明为什么
   这个顺序不能写成库的设计前提。
7. 用 `Vec<Vec<u64>>` 与扁平 `Vec<u64>` 实现同一个二维求和，先测量地址间距和遍历时间，
   再讨论结果为何依赖规模、cache 和编译优化。

## 5.12 小结

| 问题 | 答案 |
|---|---|
| `Box<T>` 移动了什么 | sized pointee 时只移动一个拥有所有权的指针 |
| `Box<[T]>` 为什么更大 | 它是带长度 metadata 的胖指针 |
| `Vec<T>` 的三个量 | `ptr`、`len`、`capacity` |
| `len` 与 `capacity` 的区别 | 已初始化元素数 vs 不扩容最多元素数 |
| 空 `Vec::new()` 是否分配 | 不分配，使用对齐的非空 dangling pointer |
| `Vec<()>` 如何工作 | 无字节分配，容量为 `usize::MAX`，只增长 len |
| push 为什么摊还 O(1) | 倍增扩容用几何级数摊平复制成本 |
| 扩容后旧元素指针还能用吗 | 不能；realloc 可能已经释放或移动旧分配 |
| `set_len` 做了什么 | 只修改逻辑长度，不初始化也不 drop |
| `Vec` 的 drop | drop 前 len 个元素，再释放缓冲区 |
| `String` 是什么 | 带“合法 UTF-8”不变量的 `Vec<u8>` |
| `String::len()` 是什么 | UTF-8 字节数，不是字符数 |
| `from_utf8_unchecked` 的边界 | 调用者必须证明字节合法，否则破坏 String 不变量 |
