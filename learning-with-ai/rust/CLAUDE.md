# Rust 语言机制学习

本目录是机制级的 Rust 学习区：不写入门教程，只回答"这条语言规则在机器上兑现成了什么"。

## 读者与目标

- 读者背景：Scala/FP 为主，C 深度实践，读得懂汇编/LLVM IR/MIR，读过 DuckDB/Arrow/DataFusion 源码；
- 已有 Rust 材料：`books/learning/src/rust_*.md`（类型系统、语法糖、`Rc` 布局、unsafe、生命周期）、
  `learning-with-ai/pin`（`Pin`/`Future`）；
- 目标：补齐从布局 → 所有权/drop → trait 分派/单态化 → std 核心库 → 并发/异步运行时的主干。

## 目录约定

- `src/`：mdBook 正文；`src/SUMMARY.md` 只登记**已写完**的章节；
- `qa/`：问答原始记录，文件命名 `NNN-主题.md`，追问**追加**到原文件末尾，不覆盖；
- `code/`：可运行的验证代码，单个 cargo package，一章一个 `src/bin/chNN_*.rs`；
- 正文引用的每个数字都必须有对应探针代码，并注明命令与平台。

## 写作纪律

1. 先机器模型，再语言规则，再实测，再 std 源码对照，最后练习；
2. 属于**实现细节**的结论（布局、优化、内联）显式标注，不写成契约；
3. std 明确承诺的性质（如 `Option<&T>` 同大小）标注为"保证";
4. 未测量的推断不写成事实。

## 命令

```bash
# 代码
cd code
cargo run --bin ch01_layout
cargo rustc --release --bin ch01_abi -- --emit asm -C debuginfo=0

# 布局报告（nightly，必须触发重新编译）
touch src/bin/ch01_layout.rs
cargo +nightly rustc --bin ch01_layout -- -Zprint-type-sizes 2>&1 | less

# 构建本书
mdbook build . -d /tmp/rust-book
mdbook serve . -d /tmp/rust-book --open .
```

本项目未接入 `.github/workflows/main.yml` 的部署流水线；需要发布时再加构建步骤与
`mysite/content/_index.md` 的入口。
