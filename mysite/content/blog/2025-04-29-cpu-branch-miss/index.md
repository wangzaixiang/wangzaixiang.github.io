+++
title = "M1 CPU 分支预测失误对性能的影响测试"
description = "在超标量处理器中，分支预测的准确率如何，以及分支预测准确率对性能的影响如何，这个实验是在 M1 CPU 上的一个测试，为我们更好的理解现代超标量处理器下分支预测的工作原理提供了一个很好的视角，帮助我们更好的编写高性能的代码。"
date = 2025-04-29
draft = false
template = "blog/page.html"

[extra]
toc = true
+++

# M1 CPU 分支预测失误对性能的影响测试

在超标量处理器中，分支预测的准确率如何，以及分支预测准确率对性能的影响如何，这个实验是在 M1 CPU 上的一个测试，为我们更好的理解现代超标量处理器下
分支预测的工作原理提供了一个很好的视角，帮助我们更好的编写高性能的代码。

[测试代码](https://github.com/wangzaixiang/vectorize_engine/blob/main/playgrounds/try_cpu/src/bin/TestLoop.rs)：
```rust
struct TestCase {
    name: String,   // same as mmap file
    numbers: usize, // number of elements
    mmap: Mmap
}
impl TestCase {
    
    /// 遍历数组，对每一个元素遍历其每一个为1的二进制位
    fn execute(&self) -> (u64, u64) {
        let mut sum = 0u64;
        let mut loops = 0u64;
        let numbers = unsafe { std::slice::from_raw_parts(self.mmap.as_ptr() as *const u64, self.numbers) };

        numbers.iter().for_each(|&n| {
            let mut n = n;
            while n > 0 { /// 这个分支判断的准确度是本测试的关键点，随着样本数据的随机性不同，其预测准确度也会不同
                let bit = n.trailing_zeros();
                n &= !(1 << bit);
                sum += bit as u64;
                loops += 1;
            }
        });

        (sum, loops)
    }
}

/// data1: 0..n 样本数据比较有序，预期分支预测成功率高
fn generate_data1(slice: &mut [u64]) {
    (0..slice.len()).for_each( |n| slice[n] = n as u64);
}

/// data2: 每个样本的 popcnt 都是 15，预期分支预测成功率最高
fn generate_data2(slice: &mut [u64]) {
    let nums = [0xFFF7u64, 0xFFF70, 0xFFF700, 0xFFF7000, 0xFFF70000, 0xFFF700000, 0xFFF7000000, 0xFFF70000000];
    (0..slice.len()).for_each( |n| slice[n] = nums[ n & 0x7 ]);
}

/// data3: 完全随机的数据样本，预期分支预测成功率低
fn generate_data3(slice: &mut [u64]) {
    let mut random = rand::thread_rng();
    (0..slice.len()).for_each( |n| slice[n] = random.next_u64());
}
```

## 测试结果

本次测试是在 M1 Max（64G) 和 M4(32G) 上进行，由于采用 mmap 方式读取数据，由于测试文件的大小 < 8G，在两个环境中，均可保证文件全部在 page cache 中，
测试结果一般采用第2轮后的结果，以避免 IO 的影响。

| test case | IPC   | branch-misses | ns/iter | description |
|-----------|-------|---------------|---------|-------------|
| test1-M1  | 3.996 | 4.88e-3       | 0.776   | 测试样本较为有序    |
| test2-M1  | 4.596 | 6e-7          | 0.678   | 测试样本非常有序    |
| test3-M1  | 1.908 | 2.9e-2        | 1.555   | 测试样本随机无序    |
| test1-M4  | 4.63  | 3.44e-3       | 0.563   |             |
| test2-M4  | 5.307 | 1.5e-7        | 0.498   |             |
| test3-M4  | 2.034 | 2.9e-2        | 1.136   |             |

M1 vs M4 微架构对比（来源于 deepseek ） 
1. 解码器宽度优化
   - 指令解码能力提升
     - 采用 10 宽解码单元（每个时钟周期可解码 10 条指令），相比 M3 的 9 宽进一步加宽，推测这是苹果自 M1（推测为 8 宽）以来的最大规模前端优化。
     - M1：基于 ARMv8 架构，解码器宽度较窄（推测为 8 宽），指令吞吐量较低，限制了并行处理能力。
   - 前端与后端协同改进
     - M4 的解码器与后端执行单元匹配更高效，通过 扩大分支预测窗口和 优化指令预取逻辑，减少流水线停顿，提升指令级并行度（ILP）。
2. 后端执行架构升级
   - 执行单元与调度队列扩展
     - M4：后端 Dispatch Buffer（指令派发缓冲区）和 浮点调度队列容量显著增加，支持更复杂的乱序执行（Out-of-Order Execution）。例如：
       - 浮点单元（FPU）调度队列深度增加 20%，支持更多指令并行执行。
       - 新增 SME 单元（可扩展矩阵扩展，类似 ARMv9 的 SVE2），专为 AI 和 SIMD 密集型任务优化，加速矩阵运算。
     - M1：后端执行资源较少，缺乏专用 AI 加速单元，依赖传统 SIMD 指令（如 NEON）处理并行任务。
   - 内存子系统优化
     - M4 采用 -7500 内存，延迟从 M1 的 96ns 降低至 88ns，配合更大的共享 L2 缓存（16MB），减少后端执行单元等待数据的时间。

## 分析

以 test1 在 M1 上的某次样本为例：
```text
thread: 88508440, trace time: 4.999996
        cycles: 16016222700
  instructions: 63551443474
      branches: 7620789323
 branch-misses: 37781905
 
case: test1, sum: 213330485248, loops: 14846928128, elapsed: 11.525sec, avg: 0.776 ns/iter
```

1. 1 iter
   - 0.776ns
   - cycles: 2.486
   - IPC: 3.97
   - instructions: 9.86: 这个数与 下面的汇编指令是一致的（9，考虑到外层循环还有一些指令的分担，后面看看是否有更精准的测试方法）
2. 核心循环的 asm: 一共 9 条指令，根据指令的依赖性，至少需要 5 个 cycle 才能完成一次迭代
    ```asm
    0x1000028a0:
    1:	rbit   x11, x10             ; 1 x11 = reverse x10 bits, u1-6            
    2:	clz    x11, x11             ; 2 x11 = count leading zeros of x11, u1-6    
    3:	lsl    x12, x9, x11         ; 3 x12 = 1 << x11  u1-6    
    4:	add    x25, x11, x25        ; 4.1   sum += x11  u1-6  
    5:	add    x24, x24, #0x1       ; 4.2   loops += 1  u1-6  
    6:	mov    x1, x24              ; 5.1   x1 = sum  ; 这条指令本来可以挪到循环外面，rustc 没有进行这个优化  
    7:	mov    x0, x25              ; 5.2   x0 = loop ; 这条指令本来可以挪到循环外面，rustc  没有进行这个优化 
    8:	bics   x10, x10, x12        ; 4.3   x10 = x10 & ~x12    2 * u1-3
    9:	b.ne   0x1000028a0          ; 5.3. <+3112> [inlined] core::num::<impl u64>::trailing_zeros at uint_macros.rs:162:20
                                    ; u1-2
    ```
3. 大致相当于在每 5 cycle 中完成了 2 次迭代，即相当于 loop i 和 loop i+1 同时执行
   - 限制只能执行 2 个迭代的原因，估计是 bics / b.ne 指令仅能在 u1-2 中执行，其执行单元的限制导致了后续的指令无法继续并行。
   - 如果要进一步提升并发行，则需要 cpu 在设计上进一步提高执行单元的数量。
4. 在大部份情况下，CPU 的 分支预测准确率相当之高，比如 test1 中准确度高达：99.512%，test2 则是 99.9999%，而 test3 则是 97.1%。
   其中 test3 的样本数据是完全随机的，分支预测的准确率仍然高达 97.1%。这个值是远高于我的预期的。
5. 分支预测准确度对性能的影响是巨大的，test3 的准确度降低了2.9%，但整体性能则下降了 约 50%。
   > 假设分支预测成功时 ns/iter 为 x，失败时为 y，则：
   >
   >  - x = 0.678
   >  - (0.971 * x + 0.029 * y) = 1.555
   >  - y = 30.91
   > 
   >  y/x = 45.6 即：1次分支预测失败的成本约等于 45.6 次分支预测成功的成本。
   
   当分支预测失败时，进入到流水线后面的指令需要进行清理，并重置前端取指、译码等操作，在这个案例中，则相当于 ～50-100 个 cycle 的成本，
   这个值显然高于预期了。（网上的说法是 15-20 个 cycle）

## 思考
当数据本身的无序性较高时，CPU 的分支预测能力会大幅度下降，其对于性能的影响也会大幅度上升。如何应对这类问题呢，这里是一些我的思考：
1. 通过数据的预处理，提升数据的有序性，来提升分支预测的准确度。
2. 评估消除分支的可能性，例如，使用 算数操作 来替代分支操作。
   - 简单条件判断转换为 CMOV(x86), CSEL(ARM) 指令
   - `if (flag) sum += x; ` 改写为 `sum += flag * x;`
   - 使用 SIMD 实现无分支的并行计算
