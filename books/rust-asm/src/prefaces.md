最近，阅读了 [rust-under-the-hood](https://github.com/eventhelix/ruth)一书，从生成的汇编代码来理解 Rust 语言，颇有一些收获：
1. Rust 语言编译后的汇编代码，在很多方面的优化，是令人惊讶的。例如，对于 vector 的函数式操作，与 for 循环等相比，生成的代码是等效的，这
   既享受了语法上的优雅简洁（如Scala），又享受了性能的优势（而这是 scala 等望尘莫及的）
2. SIMD 指令集的使用，让应用代码可以更好的利用 CPU 的并行计算能力。

加之，最近在阅读 DuckDB 的源代码，也对向量计算非常的感兴趣，写这个系列，是想进一步的实践、研究 向量相关的编译优化技术，为后续的一些性能
优化工作做些筹备。

1. 哪些场景 适合于 compiler vectorization？
2. 使用 portable simd 库来编写处理向量的代码？是否会有更好的性能提升？

## tips
1. 查看 HIR 代码：`cargo rustc --release -- -Zunpretty=hir`
2. 查看 MIR 代码：`cargo rustc --release -- -Zunpretty=mir`
3. 查看 LLVM IR 代码：`cargo rustc --release -- --emit llvm-ir`，生成的文件在 `target/release/deps/` 目录下。
4. 查看 ASM 代码： `cargo rustc --release -- --emit asm -C llvm-args=-x86-asm-syntax=intel`，生成 intel 风格的汇编代码 (move dest src)
5. 编译选项：`-C target-cpu=native`，生成针对当前 CPU 的优化代码。
6. 编译选项：`-C target-feature=+avx2`，生成针对 AVX2 指令集的优化代码。
7. 编译选项：`-C target-feature=+avx512f`，生成针对 AVX512 指令集的优化代码。
8. 交叉编译 `--target x86_64-apple-darwin` 在 M1 下编译生成 x86_64 的代码。
9. 对有的 cargo 命令，如 cargo bench，可以使用 RUSTFLAGS 环境变量传递