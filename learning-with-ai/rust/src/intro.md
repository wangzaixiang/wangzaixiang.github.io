# Rust 语言机制：从字节到运行时

## 这个学习区是什么

定位：**机制级**的 Rust 学习区。不重复《Rust 程序设计语言》，也不做"入门到放弃"式教程。
每一章都回答同一个问题：**这条语言规则在机器上兑现成了什么？**

读者画像（按仓库证据建立，如有偏差随时纠正）：

- Scala / 函数式背景，长期做商业软件（零售 ERP）后端；
- 有 C 的深度实践，读得懂汇编、LLVM IR、MIR；
- 读过 DuckDB / Arrow / DataFusion 源码，对向量化、内存布局、执行引擎有直觉；
- 已经在 `books/learning` 写过 Rust 类型系统、语法糖、`Rc` 布局、unsafe 转换、生命周期，
  在 `learning-with-ai/pin` 系统拆过 `Pin` 与 `Future`。

结论：**不是零基础，是知识零散**。Rust 的概念你大多碰过，缺的是一条从
"所有权 / 布局" 到 "分派 / 单态化" 到 "并发 / 异步运行时" 的主干，以及 std 核心库的横向全貌。

## 学习方法：每章五步

| 步骤 | 内容 | 产出 |
|---|---|---|
| 1. 机器模型 | 先讲清楚"硬件/内存里发生了什么" | 心智模型 |
| 2. 语言规则 | 再讲"编译器要求什么" | 可复述的规则 |
| 3. 实测 | 写代码 + 看 `size_of` / `-Zprint-type-sizes` / MIR / ASM | **可复现的数字** |
| 4. std 对照 | 回到标准库源码，看官方怎么用 | 真实案例 |
| 5. 练习 | 3–5 个能被验证的小题 | 手感 |

纪律：**凡未被测量/查源码确认的结论，一律标注为推断。** 这本书里出现的每个数字都写过对应的探针代码。

问答走 `qa/` 目录：随时提问，回答先落到 `qa/NNN-主题.md`（追加，不覆盖），
阶段性回灌到正文。正文只保留沉淀后的结论。

## 路线图

状态：✅ 已写 · 🚧 进行中 · ⬜ 计划

### 第一部分：数据与内存

| 章 | 主题 | 核心问题 | 状态 |
|---|---|---|---|
| 1 | [值的布局](memory/layout.md) | size/align/offset、`enum` 的 tag、niche、布局如何决定 ABI | ✅ |
| 2 | [移动、Copy、Drop](memory/move-copy-drop.md) | 为什么"移动"是 memcpy？drop glue 长什么样？`Pin` 的必要性从哪来 | ✅ |
| 3 | [引用与生命周期](memory/references-lifetimes.md) | NLL 之后借用检查到底还剩什么？variance 为什么会影响 `&'a mut T` | ✅ |
| 4 | [Cell 家族](memory/cells.md) | `UnsafeCell`、`Cell`、`RefCell`、`OnceCell` 的布局、检查与线程约束 | ✅ |
| 5 | [堆上的所有权](memory/heap-ownership.md) | `Vec` 增长与不变量、realloc、`String` 的 UTF-8 不变量 | ✅ |

### 第二部分：抽象机制

| 章 | 主题 | 核心问题 | 状态 |
|---|---|---|---|
| 6 | trait 与分派 | vtable 的字节布局、`dyn` 为什么是 16 字节、对象安全的机器原因 | ⬜ |
| 7 | 泛型与单态化 | 代码膨胀的代价、跨 crate inline、`impl Trait` 三种位置的区别 | ⬜ |
| 8 | 迭代器零成本 | 适配器状态机、`size_hint`、`collect` 的 `FromIterator`、为什么常被内联掉 | ⬜ |
| 9 | 错误与 panic | `?` 的脱糖与 `From`、`panic=abort` 的取舍、`no_std` 下没有 `Error` | ⬜ |
| 10 | 宏 | `macro_rules!` 的匹配与卫生性、三类过程宏、`cargo expand` 读法 | ⬜ |

### 第三部分：std 核心库（横向补齐）

| 章 | 主题 | 核心问题 | 状态 |
|---|---|---|---|
| 11 | 智能指针与原子引用计数 | `Box`/`Rc`/`Arc` 布局与原子序、`Weak`、别名模型与释放顺序 | ⬜ |
| 12 | 并发原语 | `Send`/`Sync` 如何自动推导、`Mutex`/`RwLock`/`Condvar`、channel 的实现、内存序 | ⬜ |
| 13 | 容器 | `HashMap`(SwissTable)、`BTreeMap`、`VecDeque` 的布局与常数因子 | ⬜ |
| 14 | 异步运行时 | `async` 脱糖成什么状态机、`Waker` 的 vtable、手写一个 executor | ⬜ |

### 第四部分：落地

| 章 | 主题 | 核心问题 | 状态 |
|---|---|---|---|
| 15 | 工程化 | workspace、feature 组合爆炸、测试分层、miri/clippy 与 unsafe 审计 | ⬜ |
| 16 | 综合案例 | 用 std 实现一个小型列式向量 + 算子，对齐 Arrow 心智 | ⬜ |

顺序可按你的追问重排——`qa/` 里提问优先级高于路线图。

## 与前序材料的关系

| 已有材料 | 本书中的位置 |
|---|---|
| `books/learning/src/rust-rc-layout.md` | 第 11 章的实测素材 |
| `books/learning/src/rust_type_system.md` | 第 6、7 章的案例（Arrow 的 `ArrowPrimitiveType`） |
| `books/learning/src/rust_sugars.md` | 第 8、9 章的语法糖对照 |
| `learning-with-ai/pin` | 第 2、14 章的前置；本书只补"为什么能移动"这一层 |
| `books/learning/src/rust_vectorization-1.md` | 第 7、8 章的编译优化视角 |

不重复写，只做**补齐与串联**。

## 代码与构建

```bash
cd code
cargo run --bin ch01_layout                  # 运行时探针
cargo rustc --release --bin ch01_abi -- --emit asm -C debuginfo=0
touch src/bin/ch01_layout.rs
cargo +nightly rustc --bin ch01_layout -- -Zprint-type-sizes   # 编译器布局报告

# 构建本书
mdbook serve . -d /tmp/rust-book --open
```

工具链速查见[附录：验证工具链](appendix/toolchain.md)。

## 约定

- 代码全部可运行，`code/src/bin/chNN_*.rs` 与章节一一对应；
- 实测输出直接贴在正文里，并注明命令与平台；
- 属于**实现细节**的结论会显式标注，不写成契约；
- 中英混排沿用仓库习惯，术语首次出现给英文原文。
