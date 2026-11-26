# 附录：验证工具链

Rust 的"机制"很少能靠读文档学明白，必须能自己把中间产物调出来。
本页是本学习区的工具箱，每章都会引用。

## 1. 运行时探针（stable）

`std::mem` 与 `offset_of!`（1.77+）足够回答 80% 的布局问题：

```rust
use std::mem::{align_of, offset_of, size_of};

println!("{} {} {}", size_of::<T>(), align_of::<T>(), offset_of!(T, field));
```

原则：**能跑出数字的，不要靠记忆。** 把探针写进 `code/src/bin/chNN_*.rs`。

## 2. 编译器布局报告（nightly）

```bash
touch src/bin/ch01_layout.rs
cargo +nightly rustc --bin ch01_layout -- -Zprint-type-sizes 2>&1 | less
```

要点：

- **必须重新编译**才会输出（cargo 认为已是最新时什么都不打印，容易误判成"没有输出"）；
- 输出在 stderr；
- 它同时告诉你 `discriminant` 占几字节、padding 在哪、`None` 是否被 niche 吃掉
  （"没有 discriminant 行" 就是用了 niche）。

如果 Homebrew/系统安装的 `cargo` 抢在 rustup 代理之前，`cargo +nightly` 会报
`no such command: +nightly`。当前机器的可复现写法是显式指定 nightly 的 `rustc`：

```bash
RUSTC="$(rustup which --toolchain nightly rustc)" \
  RUSTUP_TOOLCHAIN=nightly \
  cargo rustc --bin ch01_layout -- -Zprint-type-sizes
```

## 3. MIR（类型检查之后、借用检查所在的层）

```bash
RUSTC="$(rustup which --toolchain nightly rustc)" \
  RUSTUP_TOOLCHAIN=nightly \
  cargo rustc --bin ch02_move -- -Zunpretty=mir 2>&1 | less
# 或写入文件
RUSTC="$(rustup which --toolchain nightly rustc)" \
  RUSTUP_TOOLCHAIN=nightly \
  cargo rustc --bin ch02_move -- -Zunpretty=mir > /tmp/ch02.mir
```

MIR 是读"所有权/生命周期/Drop"的最佳层：`StorageLive/StorageDead`、`drop(x)`、
`_3 = Len(_2)` 这些语句直接对应语言规则。

实测（`rustc +nightly -Zunpretty=mir`）：输出是**借用检查之后**的形态——
有借用冲突时 rustc 直接报 E0502 并不输出 MIR；成功时能看到 `drop(_1)` 与 `cleanup` 块：

```text
fn takes(_1: String) -> usize {
    let mut _0: usize;
    let mut _2: &std::string::String;
    bb0: { _2 = &_1; _0 = String::len(move _2) -> [return: bb1, unwind: bb3]; }
    bb1: { drop(_1) -> [return: bb2, unwind continue]; }
    bb3 (cleanup): { drop(_1) -> [return: bb4, unwind terminate(cleanup)]; }
}
```

想看各 pass 的差异（包括借用检查之前的形态、region 约束、polonius）：

```bash
RUSTC="$(rustup which --toolchain nightly rustc)" \
  RUSTUP_TOOLCHAIN=nightly \
  cargo rustc --bin ch02_move -- -Zdump-mir=all
ls mir_dump/     # *.nll.0.mir（借用检查后）、*.regioncx.*.dot、*.polonius.0.html
```

## 4. LLVM IR

```bash
cargo rustc --release --bin ch01_abi -- --emit=llvm-ir
# 产物：target/release/deps/ch01_abi-*.ll
```

看优化的真实结果：循环是否被向量化、`Option` 的判空是否被消掉、泛型实例被内联的程度。

## 5. 汇编

```bash
cargo rustc --release --bin ch01_abi -- --emit asm -C debuginfo=0
ls target/release/build/rust-book/*/out/ch01_abi.s
```

注意两点：

- 用 `--emit asm` 时，产物落在 `target/release/build/<pkg>/<hash>/out/`，不是 `deps/`；
- macOS 符号带前导下划线（`_take_pair`），`#[unsafe(no_mangle)]` 可避免名字被 mangle。

Intel 语法（更接近 x86 习惯）：

```bash
cargo rustc --release --bin ch01_abi -- --emit asm -C llvm-args=-x86-asm-syntax=intel
```

`cargo asm` 也在本机装了，但在当前环境报 `No such file or directory`（缺 `objdump`），
暂以 `--emit asm` 为准。

## 6. 宏展开

```bash
cargo expand --bin ch07_iter            # 全 crate
cargo expand --bin ch07_iter path::to::item
```

第 7、9 章会频繁使用：`async` 状态机、`?`、`for`、`vec![]` 的真身都在这一层。

## 7. 动态检查

```bash
cargo +nightly miri run --bin ch02_move
cargo +nightly miri test
```

Miri 查的是 **UB**，不是逻辑错误：未对齐读写、悬垂指针、别名冲突、越界、`transmute` 非法值。
第 2、10、14 章会用它给 `unsafe` 代码做证。

## 8. 依赖与体积

```bash
cargo tree -e features -i serde    # 谁把某个 feature 打开了
cargo bloat --crate-name rust_book # 二进制里谁最占地方（需额外安装）
```

## 9. 版本与环境

本章所有数字的记录格式：

```text
rustc --version : rustc 1.100.0 (0ef1bc386 2026-11-09)
target          : aarch64-apple-darwin
profile         : dev（除注明 release）
```

跨 target / 跨版本的数字会变，凡涉及具体布局的结论都在正文里标注"实现细节"。
