# 第 2 章 移动、Copy 与 Drop：所有权如何落到字节上

> 配套代码：`code/src/bin/ch02_move.rs`  
> 所有运行时数字均为本机实测（aarch64-apple-darwin，rustc 1.100.0）。

## 2.1 一句话机器模型

Rust 的移动不是“调用移动构造函数”，而是两件事的组合：

1. 把值的字节表示放到目的地；
2. 把“负责 drop 这个值”的义务从源位置转给目的地。

源位置随后在类型系统里被视为未初始化，不能再读、不能再移动，也不能再 drop。

```text
移动前：source [value] owns-drop
移动后：source [stale bytes] no-drop
        destination [value] owns-drop
```

这里最容易误解的是：**源字节不一定被清零，也不保证被覆盖。**  
“未初始化”是抽象机里的状态；机器上的旧字节可能还躺在原地址，只是安全代码不能再把它当作原值使用。

因此“移动 = memcpy”是一个有用的心智模型，但它只描述了数据搬运这一半，漏掉了所有权的另一半：

$$
\mathrm{move}(T) = \mathrm{bitwise\ transfer} + \mathrm{ownership\ transfer}
$$

并且实际代码生成仍可能把复制优化掉、直接传指针，或者只搬寄存器。

## 2.2 移动的是“句柄”，不是它拥有的资源

`String` 的布局是 24 字节的所有权句柄，堆上的字符数据不在句柄里：

```text
String
+----------------------+-------+-------+
| heap ptr             |  cap  |  len  |
+----------------------+-------+-------+
          |
          v
        heap: a heap-allocated string
```

探针先移动，再 clone：

```rust
let original = String::from("a heap-allocated string");
let heap_ptr = original.as_ptr();
let moved = original;
let cloned = moved.clone();
```

实测输出：

```text
String size = 24, same heap pointer after move = true, after clone = false
moved value = "a heap-allocated string"
cloned value = "a heap-allocated string"
```

结论：

- move 保留原来的堆指针，没有复制字符数据；
- clone 创建了新的堆分配，因此指针不同；
- 但 move 和 clone 在控制块上可能都表现为“写 24 字节”，区别在所有权是否转移、资源是否复制。

`Box<T>` 更直接，移动的只是一个指针。实测：

```text
Box<[u64; 4]> size = 8, same pointee after move = true
```

所以：

```text
移动大对象
≠
复制对象拥有的全部数据
```

真正需要搬多少字节，由类型的内联布局决定；堆上资源只移动指向它的所有权句柄。

## 2.3 MIR：move 与 copy 是两种 place 操作

配套函数：

```rust
#[unsafe(no_mangle)]
#[inline(never)]
pub fn mir_move_into_local(value: Tracked) -> Tracked {
    let moved = value;
    std::hint::black_box(&moved);
    moved
}
```

运行：

```bash
RUSTC="$(rustup which --toolchain nightly rustc)" \
  RUSTUP_TOOLCHAIN=nightly \
  cargo rustc --bin ch02_move -- -Zunpretty=mir 2>&1 | less
```

关键 MIR：

```text
fn mir_move_into_local(_1: Tracked) -> Tracked {
    let mut _0: Tracked;
    let _2: Tracked;
    let _3: &Tracked;
    let mut _4: &Tracked;

    bb0: {
        _2 = move _1;
        _4 = &_2;
        _3 = std::hint::black_box::<&Tracked>(move _4) -> [return: bb1, unwind: bb2];
    }

    bb1: {
        _0 = move _2;
        return;
    }

    bb2 (cleanup): {
        drop(_2) -> [return: bb3, unwind terminate(cleanup)];
    }
}
```

读法：

- `_1` 是函数参数；
- `_2 = move _1` 后，`_1` 不再负责这个值；
- `_0 = move _2` 又把责任交给返回值；
- cleanup 只 drop `_2`，不会 drop `_1`；
- 如果代码路径没有把 `_2` 移走，异常路径才会负责 drop 它。

再看一个“参数在函数结束时 drop”的函数：

```rust
pub fn mir_drop_on_return(value: String) -> usize {
    value.len()
}
```

核心 MIR：

```text
bb0: {
    _2 = &_1;
    _0 = String::len(move _2) -> [return: bb1, unwind: bb3];
}

bb1: {
    drop(_1) -> [return: bb2, unwind continue];
}

bb2: {
    return;
}

bb3 (cleanup): {
    drop(_1) -> [return: bb4, unwind terminate(cleanup)];
}
```

这里最值得看的是 `bb1` 与 `bb3`：

```text
正常返回：drop 参数
函数 panic：也要在 cleanup 中 drop 参数
```

所以 Rust 的析构不是“只在右花括号处发生”，而是 drop elaboration 把析构责任放到所有控制流出口上。

MIR 里出现 `copy` 也不一定说明类型实现了 `Copy`。某些优化阶段会把“已经确定无需保留源值”的 move 简化成 copy；判断所有权语义要看类型和早期 MIR，而不是只看某个 pass 的最终形态。

## 2.4 Copy：允许隐式位复制，源值保持有效

`Copy` 是一个 marker trait：

```rust
pub trait Copy: Clone {}
```

标准库的定义非常直接：

> Types whose values can be duplicated simply by copying bits.

```rust
#[derive(Clone, Copy)]
struct CopyWord(u64);

let source = CopyWord(7);
let destination = source;

println!("{} {}", source.0, destination.0); // 7 7
```

实测：

```text
CopyWord size = 8, source = 7, destination = 7
```

移动和复制在机器层都可能只是搬 bits，真正差别在语言规则：

| 操作 | 源值之后能否使用 | 是否可隐式发生 | 谁负责 drop |
|---|---|---|---|
| move | 否 | 是 | 目的地 |
| copy | 是 | 是，要求 `T: Copy` | 没有需要释放的所有权 |
| clone | 是 | 否，必须调用 `.clone()` | 新旧值各自负责自己的资源 |

`Copy` 与 `Clone` 的边界：

- `Copy: Clone`，所以 `Copy` 必须实现 `Clone`；
- `Copy` 的行为不可重载，就是位复制；
- `Clone` 可以分配新内存、复制文件描述符或做引用计数，因此可执行任意用户代码；
- `String` 是 `Clone` 但不是 `Copy`，否则两个句柄会同时释放同一块堆内存。

`Copy` 与 `Drop` 互斥。下面的代码会报 `E0184`：

```rust
#[derive(Clone, Copy)]
struct Bad;

impl Drop for Bad {
    fn drop(&mut self) {}
}
```

原因很直接：若一个值既可被隐式复制、又拥有 drop 责任，那么复制出来的每个 bit 副本都会认为自己要释放同一份资源。

## 2.5 Drop：先执行自定义逻辑，再递归 drop 字段

析构由两部分组成：

1. 如果类型实现了 `Drop`，调用 `Drop::drop(&mut self)`；
2. 编译器生成的 drop glue 递归析构字段。

标准库对这两层关系的定义是：

> A call to `Drop::drop` ... plus the automatically generated "drop glue" which recursively calls the destructors of all the fields.

探针的输出把顺序全部展开：

```text
after consume: ["consume Tracked(argument)", "drop Tracked(argument)"]

before reassignment: ["initial Tracked(old)"]
after reassignment: ["drop Tracked(old)"]
after explicit drop: ["drop Tracked(new)"]

after local scope: ["drop Tracked(second local)", "drop Tracked(first local)"]

after Outer drop: ["drop Outer", "drop Tracked(outer.first)", "drop Tracked(outer.second)"]
```

四条规则：

1. 移入函数参数的 `Tracked` 只在参数目的地 drop 一次；
2. 对已初始化变量重新赋值，旧值在赋值点立刻 drop；
3. 同一作用域的局部变量按声明逆序 drop；
4. 自定义 `Drop::drop` 先执行，之后字段按声明顺序 drop。

`Drop::drop` 接收 `&mut self`，不接收 `self`。因为执行析构方法时，值还没有被释放，之后编译器仍要处理字段和内存：

```text
Drop::drop(&mut self)
        ↓
递归 drop 字段
        ↓
释放最外层拥有的存储
```

`mem::drop(x)` 不是编译器魔法。它就是一个拿走 `x`、函数体为空的函数；`x` 在函数体结束时按普通规则析构：

```rust
pub fn drop<T>(_x: T) {}
```

## 2.6 不自动 drop：ManuallyDrop 与 forget

`ManuallyDrop<T>` 与 `T` 布局相同，但禁止自动析构：

```text
after ManuallyDrop scope: ["leaving ManuallyDrop scope"]
```

没有出现 `drop Tracked(manual)`。若确实需要提前析构，必须显式调用：

```rust
let mut value = ManuallyDrop::new(Tracked("manual"));
unsafe {
    ManuallyDrop::drop(&mut value);
}
```

这里的 `unsafe` 来自重复析构风险：`ManuallyDrop::drop` 不会把状态改成“已 drop”，调用者必须自己保证之后不再 drop。

`mem::forget` 也禁止析构，但它本身是安全函数：

```text
after mem::forget: []
```

它不是 `unsafe`，因为泄漏内存不会破坏 Rust 的安全保证。但它会泄漏所有未手动释放的资源。反过来，任何 `unsafe` 抽象都不能把“析构函数一定运行”当作安全前提：

```text
safe code 可以忘记一个值
=> unsafe code 必须容忍泄漏
```

这是设计 `ManuallyDrop`、`Arc`、FFI 所有权和 arena 时必须遵守的规则。

## 2.7 为什么移动会逼出 Pin

移动会改变值所在的地址。如果值内部保存了指向自己的指针，那么移动后旧地址失效：

```text
移动前:
self.field.ptr ─────┐
                    v
                [ self ]

移动后:
self.field.ptr ─────┐
                    v
               [stale old location]

                [ self ] 新的地址
```

`Pin<P>` 的价值是把“地址不会再变化”变成 API 前置条件，让自引用类型的内部指针继续保持有效。后续异步章节会再次回到这里；本章只需记住因果链：

```text
值在内存里可移动
    ↓
移动会改变地址
    ↓
依赖地址的自引用值不能任意移动
    ↓
需要 Pin 把地址稳定性写进类型约束
```

## 2.8 复现与观察点

```bash
cd code

# 运行时析构顺序和所有权观察
cargo run --bin ch02_move

# 核心 MIR：move、drop、cleanup
RUSTC="$(rustup which --toolchain nightly rustc)" \
  RUSTUP_TOOLCHAIN=nightly \
  cargo rustc --bin ch02_move -- -Zunpretty=mir 2>&1 | less

# 所有 MIR pass、借用检查前后形态
RUSTC="$(rustup which --toolchain nightly rustc)" \
  RUSTUP_TOOLCHAIN=nightly \
  cargo rustc --bin ch02_move -- -Zdump-mir=all
ls mir_dump/

# release 汇编
cargo rustc --release --bin ch02_move -- --emit asm -C debuginfo=0
```

观察时不要假设：

- 栈上局部变量一定有固定地址；
- move 一定要执行一次 `memcpy`；
- 源位置移动后一定被清零；
- 参数一定被放到调用者栈帧，或一定被放进寄存器；
- 自定义 `Drop` 会替代字段析构。

这些都可能受 ABI、优化器和具体类型布局影响。语言保证的是所有权和析构责任，不是某条固定指令序列。

## 2.9 练习

1. 给 `Tracked` 增加一个 `Drop` 日志，分别让它被 move、drop、`forget`、放进 `ManuallyDrop`，
   预测并验证析构次数。
2. 把 `Tracked` 改成 `#[derive(Clone, Copy)]`，观察编译器给出什么错误；解释为什么不能用
   “手动 drop 标志”绕过。
3. 写一个包含 `String` 和 `Box<u64>` 的结构体，移动前后观察两个堆地址；再 clone，比较变化。
4. 让参数在函数中途 `panic!()`，从 `-Zdump-mir=all` 中找到 cleanup 路径上的 `drop`。
5. 构造一个包含自引用指针的 `struct`，在 MIR 中确认 move 改变地址，再说明 `Pin` 必须约束什么。

## 2.10 小结

| 问题 | 答案 |
|---|---|
| move 到底做了什么 | 搬值并转移 drop 责任；源位置逻辑上失效 |
| move 一定是 memcpy 吗 | 不一定；可能被优化成寄存器传递、间接调用或消除 |
| move 会复制堆数据吗 | 不会复制它拥有的资源，只移动所有权句柄 |
| Copy 和 Clone 的区别 | Copy 是不可重载的隐式位复制；Clone 是显式、可执行用户代码的复制 |
| 为什么 Copy 不能配 Drop | 多个 bit 副本会重复承担同一资源的释放责任 |
| 什么时候 drop | 作用域结束、赋值覆盖、显式 `drop`、移动目的地的生命周期结束，以及 unwind cleanup |
| drop 的顺序 | 自定义 `Drop` 先于字段；字段按声明顺序；局部变量按声明逆序 |
| 如何阻止 drop | `ManuallyDrop` 或 `mem::forget`，代价是必须自己处理资源或容忍泄漏 |
| 为什么需要 Pin | 移动会改变地址，而部分类型依赖地址稳定 |
