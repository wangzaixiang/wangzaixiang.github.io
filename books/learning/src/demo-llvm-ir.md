# 从一段简单的C代码来学习LLVM-IR

## 代码

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

## 命令行工具
1. 编译为 LLVM IR: `clang -S -emit-llvm demo1.c -o demo1.ll` 可结合 `-O1`, `-O3` 等优化选项。
2. 使用 `clang -c -mllvm -print-after-all demo1.c` 查看各个阶段的输出，查看各个pass后的 IR
3. `clang -mllvm --help` 
4. `clang -mllvm --help-hidden` 查看隐藏的选项
5. `clang -mllvm -debug-pass=Arguments` print pass arguments to pass to opt.

## 查看 -O3 的优化过程: `clang -S -emit-llvm -O3 -mllvm -print-after-all demo1.c 
1. 编译：`clang -S -emit-llvm -O3 -mllvm -print-after-all demo1.c -o demo1-O3.ll 2>/tmp/passes.txt`
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
5. 可以逐步的对比每个 pass 的输出，观察 IR 的演变过程，理解各个 pass 的职责。
   
   在这个小的demo中，主要是如下两个 pass 起到了关键作用：
   - simplifycfg: 简化控制流图，包括合并基本快，使用 switch 替代多个 if else 等。
   - SROA: An optimization pass providing Scalar Replacement of Aggregates. This pass takes allocations which can be completely
     analyzed (that is, they don't escape) and tries to turn them into scalar SSA values.
     刚开始的时候，IR 并不是严格意义上的 SSA，对每个变量的读写都是通过 alloca 和 load/store 来实现的，这个 pass 将这些变量转换为 SSA 形式。
   
6. 通过 opt 命令来重现某个 pass 的优化过程：（部份 pass 输出的 IL 需要简单的手工调整方能正确执行）
   ```shell
   opt -S output/demo1_6.ll -passes=simplifycfg -o -
   ```
   这里的 pass name 可以从 文件中的 `Dump After` 中找到。
   `opt -S output/demo1_6.ll -passes=simplifycfg,sroa,simplifycfg -o -` 使用这个命令，可以从 -O0 的 IR 优化到 -O3 的 IR。

## 小结
1. 本文给出了一个学习 LLVM IR 的有效方法：即跟着 clang 的编译过程，逐步了解 IR 以及各个 pass 的作用。并给出了参考的命令行工具。
2. 本文中的 passes 生成工具，脚本是通过 github copilot 辅助生成的 rust 脚本，稍微调整一下后，就可以使用，来辅助分析 IR。 
3. 对于复杂的 IR 代码，需要有一个从 IR 生成 CFG 的工具，这样可以更好的理解 IR 的控制流程。我会在后面的学习中，使用 rust 来编写这个工具。