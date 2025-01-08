+++
title = "LLVM学习系列二：从一段简单的C代码来学习LLVM-IR"
description = "本文通过一个简单的 C 代码来学习 LLVM IR 的生成过程，以及如何通过命令行工具来查看各个 pass 的输出。"
date = 2025-01-05
draft = false
template = "blog/page.html"

[extra]
toc = true
+++

本文承接 [LLVM 学习系列一：初读 LLVM-IR 示例代码](@/blog/2024-10-06-learning-llvm/index.md) 中，本文将通过一个简单的 C 代码来学习 LLVM IR 的生成过程，
以及如何通过命令行工具来查看各个 pass 的输出。

# 示例代码

```c
int demo1(int x) {
  int y = 0;

  if(x == 1) {
    y = 10;
  }
  else if(x == 100){
    y = 20;
  }
  else if(x == 200){
    y = 30;
  }
  else {
    y = 40;
  }
  return y;
}
```

# 实例学习 -O3 的优化过程: 
1. 编译：`clang -S -emit-llvm -O3 -mllvm -print-after-all demo1.c -o demo1-O3.ll 2>/tmp/passes.txt`

   查看 -O3 选项编译的全过程，可以看到每个 pass 执行后的 IR 代码。

2. `grep "Dump After" /tmp/passes.txt | wc -l` ： 共 106 个 pass
3. `clang -mllvm -debug-pass=Arguments -c demo1.c` 查看 opt 的参数：
   ```text
    Pass Arguments:  -tti -targetlibinfo -assumption-cache-tracker -targetpassconfig -machinemoduleinfo -profile-summary-info -tbaa -scoped-noalias-aa \
        -collector-metadata -machine-branch-prob -regalloc-evict -regalloc-priority -domtree -basic-aa -aa -objc-arc-contract -pre-isel-intrinsic-lowering \
        -expand-large-div-rem -expand-large-fp-convert -atomic-expand -aarch64-sve-intrinsic-opts -simplifycfg -domtree -loops -loop-simplify \
        -lazy-branch-prob -lazy-block-freq -opt-remark-emitter -scalar-evolution -loop-data-prefetch -aarch64-falkor-hwpf-fix -basic-aa \
        -loop-simplify -canon-freeze -iv-users -loop-reduce -basic-aa -aa -mergeicmps -loops -lazy-branch-prob -lazy-block-freq -expand-memcmp \
        -gc-lowering -shadow-stack-gc-lowering -lower-constant-intrinsics -lower-global-dtors -unreachableblockelim -domtree -loops -postdomtree \
        -branch-prob -block-freq -consthoist -replace-with-veclib -partially-inline-libcalls -expandvp -post-inline-ee-instrument \
        -scalarize-masked-mem-intrin -expand-reductions -loops -tlshoist -postdomtree -branch-prob -block-freq -lazy-branch-prob \
        -lazy-block-freq -opt-remark-emitter -select-optimize -aarch64-globals-tagging -stack-safety -domtree -basic-aa -aa -aarch64-stack-tagging \
        -complex-deinterleaving -aa -memoryssa -interleaved-load-combine -domtree -interleaved-access -aarch64-sme-abi -domtree -loops -type-promotion \
        -codegenprepare -domtree -dwarf-eh-prepare -aarch64-promote-const -global-merge -callbrprepare -safe-stack -stack-protector -domtree -basic-aa \
        -aa -loops -postdomtree -branch-prob -debug-ata -lazy-branch-prob -lazy-block-freq -aarch64-isel -finalize-isel -lazy-machine-block-freq \
        -early-tailduplication -opt-phis -slotindexes -stack-coloring -localstackalloc -dead-mi-elimination -machinedomtree -aarch64-condopt \
        -machine-loops -machine-trace-metrics -aarch64-ccmp -lazy-machine-block-freq -machine-combiner -aarch64-cond-br-tuning -machine-trace-metrics \ 
        -early-ifcvt -aarch64-stp-suppress -aarch64-simdinstr-opt -aarch64-stack-tagging-pre-ra -machinedomtree -machine-loops -machine-block-freq  \
        -early-machinelicm -machinedomtree -machine-block-freq -machine-cse -machinepostdomtree -machine-cycles -machine-sink -peephole-opt \
        -dead-mi-elimination -aarch64-mi-peephole-opt -aarch64-dead-defs -detect-dead-lanes -init-undef -processimpdefs -unreachable-mbb-elimination \ 
        -livevars -phi-node-elimination -twoaddressinstruction -machinedomtree -slotindexes -liveintervals -register-coalescer -rename-independent-subregs \ 
        -machine-scheduler -aarch64-post-coalescer-pass -machine-block-freq -livedebugvars -livestacks -virtregmap -liveregmatrix -edge-bundles \
        -spill-code-placement -lazy-machine-block-freq -machine-opt-remark-emitter -greedy -virtregrewriter -regallocscoringpass -stack-slot-coloring \ 
        -machine-cp -machinelicm -aarch64-copyelim -aarch64-a57-fp-load-balancing -removeredundantdebugvalues -fixup-statepoint-caller-saved  \
        -postra-machine-sink -machinedomtree -machine-loops -machine-block-freq -machinepostdomtree -lazy-machine-block-freq -machine-opt-remark-emitter \ 
        -shrink-wrap -prologepilog -machine-latecleanup -branch-folder -lazy-machine-block-freq -tailduplication -machine-cp -postrapseudos \
        -aarch64-expand-pseudo -aarch64-ldst-opt -kcfi -aarch64-speculation-hardening -machinedomtree -machine-loops -aarch64-falkor-hwpf-fix-late \ 
        -postmisched -gc-analysis -machine-block-freq -machinepostdomtree -block-placement -fentry-insert -xray-instrumentation -patchable-function \
        -aarch64-ldst-opt -machine-cp -aarch64-fix-cortex-a53-835769-pass -aarch64-collect-loh -funclet-layout -stackmap-liveness -livedebugvalues \
        -machine-sanmd -machine-outliner -aarch64-sls-hardening -aarch64-ptrauth -aarch64-branch-targets -branch-relaxation -aarch64-jump-tables \
        -cfi-fixup -lazy-machine-block-freq -machine-opt-remark-emitter -stack-frame-layout -unpack-mi-bundles -lazy-machine-block-freq \
        -machine-opt-remark-emitter
    Pass Arguments:  -domtree
    Pass Arguments:  -assumption-cache-tracker -targetlibinfo -domtree -loops -scalar-evolution -stack-safety-local
    Pass Arguments:  -domtree
   ```
   把这个参数直接丢给 opt 命令行是不行的，会报错误。
4. 使用如下的脚本来分析 -print-after-all

   为了更好的观察每个 pass 的输出，可以使用如下的脚本来拆分每个 pass 后的IR，并输出到一个独立的文件中。方便使用 diff 等工具
   来对比每个 pass 的演进过程。

   ```rust
   // src/bin/passes.rs
   use std::fs::File;
   use std::io::{self, BufRead, BufReader, Write};

   fn main() -> io::Result<()> {
       // args[1] is the input file like abc.ll
       let input_file = std::env::args().nth(1).expect("no filename given");
       if !input_file.ends_with(".ll") {
           panic!("input file must end with .ll");
       }

       let path = std::path::Path::new(&input_file);
       let basename = path.file_stem().expect("no basename found").to_str().expect("basename is not a valid UTF-8 string");

       let file = File::open(input_file.as_str())?;
       let reader = BufReader::new(file);

       let mut file_count = 0;
       let mut output_file = File::create(format!("./output/{basename}_{file_count}.ll"))?;

       for line in reader.lines() {
           let line = line?;
           if line.contains(" Dump After ") {
               file_count += 1;
               output_file = File::create(format!("./output/{basename}_{file_count}.ll"))?;
           }
           writeln!(output_file, "{}", line)?;
       }

       Ok(())
   }
   ```
   `cargo run --bin passes -- path/to/file.ll` 会在 output 目录下生成多个文件，每个文件对应一个 pass 的输出。
5. 使用 [`difft`](https://github.com/afnanenayet/diffsitter) 逐步的对比每个 pass 的输出，观察 IR 的演变过程，理解各个 pass 的职责。
   
   在这个小的demo中，主要是如下两个 pass 起到了关键作用：
   - simplifycfg: 简化控制流图，包括合并基本快，使用 switch 替代多个 if else 等。
   - SROA: An optimization pass providing Scalar Replacement of Aggregates. This pass takes allocations which can be completely
     analyzed (that is, they don't escape) and tries to turn them into scalar SSA values.
     刚开始的时候，IR 并不是严格意义上的 SSA，对每个变量的读写都是通过 alloca 和 load/store 来实现的，这个 pass 将这些变量转换为 SSA 形式。
   - pass 7: After SimplifyCFGPass
      {% mermaid() %}
      ```mermaid
      flowchart TD
      %% function demo1
      %1["  %2 = alloca i32, align 4
        %3 = alloca i32, align 4
        store i32 %0, ptr %2, align 4, !tbaa !5
        call void @llvm.lifetime.start.p0(i64 4, ptr %3) #2
        store i32 0, ptr %3, align 4, !tbaa !5
        %4 = load i32, ptr %2, align 4, !tbaa !5
        %5 = icmp eq i32 %4, 1
        br i1 %5, label %6, label %7"]
          %1 -->|%6| %6
      %6["  store i32 10, ptr %3, align 4, !tbaa !5
        br label %16"]
          %1 -->|%7| %7
      %7["  %8 = load i32, ptr %2, align 4, !tbaa !5
        %9 = icmp eq i32 %8, 100
        br i1 %9, label %10, label %11"]
          %7 -->|%10| %10
      %10["  store i32 20, ptr %3, align 4, !tbaa !5
        br label %16"]
          %7 -->|%11| %11
      %11["  %12 = load i32, ptr %2, align 4, !tbaa !5
        %13 = icmp eq i32 %12, 200
        br i1 %13, label %14, label %15"]
          %11 -->|%14| %14
      %14["  store i32 30, ptr %3, align 4, !tbaa !5
        br label %16"]
          %11 -->|%15| %15
      %15["  store i32 40, ptr %3, align 4, !tbaa !5
        br label %16"]
          %10 -->|%16| %16
          %15 -->|%16| %16
          %14 -->|%16| %16
          %6 -->|%16| %16
      %16["  %17 = load i32, ptr %3, align 4, !tbaa !5
        call void @llvm.lifetime.end.p0(i64 4, ptr %3) #2
        ret i32 %17"]
      style %16 stroke:#0f0
      ```
     {% end %}
   - 2: pass 8:  After SROAPass
      {% mermaid() %}
      ```mermaid
      flowchart TD
      %% function demo1
      %1["  %2 = icmp eq i32 %0, 1
        br i1 %2, label %3, label %4"]
          %1 -->|%3| %3
      %3["  br label %11"]
          %1 -->|%4| %4
      %4["  %5 = icmp eq i32 %0, 100
        br i1 %5, label %6, label %7"]
          %4 -->|%6| %6
      %6["  br label %11"]
          %4 -->|%7| %7
      %7["  %8 = icmp eq i32 %0, 200
        br i1 %8, label %9, label %10"]
          %7 -->|%9| %9
      %9["  br label %11"]
          %7 -->|%10| %10
      %10["  br label %11"]
          %6 -->|%11| %11
          %10 -->|%11| %11
          %9 -->|%11| %11
          %3 -->|%11| %11
      %11["  %12 = phi i32 [ 10, %3 ], [ 20, %6 ], [ 30, %9 ], [ 40, %10 ]
        ret i32 %12"]
      style %11 stroke:#0f0
      ```
      {% end %}

   - 3: pass 17:  After SimplifyCFGPass
      {% mermaid() %}
      ```mermaid
      flowchart TD
      %% function demo1
      %1["  switch i32 %0, label %4 [
          i32 1, label %5
          i32 100, label %2
          i32 200, label %3
        ]"]
              %1 -->|%2| %2
      %2["  br label %5"]
              %1 -->|%3| %3
      %3["  br label %5"]
              %1 -->|%4| %4
      %4["  br label %5"]
              %1 -->|%5| %5
              %2 -->|%5| %5
              %4 -->|%5| %5
              %3 -->|%5| %5
      %5["  %6 = phi i32 [ 20, %2 ], [ 30, %3 ], [ 40, %4 ], [ 10, %1 ]
        ret i32 %6"]
      style %5 stroke:#0f0
      ```
      {% end %}
   
6. 通过 opt 命令来重现某个 pass 的优化过程：（部份 pass 输出的 IL 需要简单的手工调整方能正确执行）
   ```shell
   opt -S output/demo1_6.ll -passes=simplifycfg -o -
   ```
   这里的 pass name 可以从 文件中的 `Dump After` 中找到。
   `opt -S output/demo1_6.ll -passes=simplifycfg,sroa,simplifycfg -o -` 使用这个命令，可以从 -O0 的 IR 优化到 -O3 的 IR。

# 命令行工具参考
1. 编译为 LLVM IR: `clang -S -emit-llvm demo1.c -o demo1.ll` 可结合 `-O1`, `-O3` 等优化选项。
2. 使用 `clang -c -mllvm -print-after-all demo1.c` 查看各个阶段的输出，查看各个pass后的 IR
3. `clang -mllvm --help`
4. `clang -mllvm --help-hidden` 查看隐藏的选项
5. `clang -mllvm -debug-pass=Arguments` print pass arguments to pass to opt.
6. `opt -S src.ll -passes=.. -o -` 使用 opt 来执行选择的 pass 并观察 IR 的演进。
7. `llc` from LLVM IR to assembly
8. `as` from assembly to object file

# 小结
1. 本文给出了一个学习 LLVM IR 的有效方法：即跟着 clang 的编译过程，逐步了解 IR 以及各个 pass 的作用。并给出了参考的命令行工具。
2. 本文中的 passes 生成工具，脚本是通过 github copilot 辅助生成的 rust 脚本，稍微调整一下后，就可以使用，来辅助分析 IR。 
3. 后续：
   - 对于复杂的 IR 代码，需要有一个从 IR 生成 CFG 的工具，这样可以更好的理解 IR 的控制流程。我会在后面的学习中，使用 rust 来编写这个工具。
     > 初稿已经完成，可以参考：[ll2cfg](https://github.com/wangzaixiang/my-llvm-tools/blob/main/src/bin/ll2cfg.rs)
     > 本文中的 CFG 流程图均使用该工具生成。 
   - 可以使用本文中介绍的方法，逐步阅读更为复杂的 LLVM IR 代码，学习 LR 的基本知识。
   - 下一步重点关注的是向量化代码的编译过程，评估直接基于 LLVM 生成向量化的关系计算代码的可行性。
4. 系列链接
   1. [LLVM 学习系列一：初读 LLVM-IR 示例代码](@/blog/2024-10-06-learning-llvm/index.md)
   2. [LLVM 学习系列二：从一段简单的C代码来学习LLVM-IR](@/blog/2025-01-05-learning-llvm-2/index.md) 