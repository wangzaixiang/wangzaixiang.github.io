+++
title = "measure IPC for M1"
description = "要挑战CPU的极致性能，IPC是一个重要的指标，本文介绍我在M1进行IPC度量的一些实践，以及因此而获得一些关于CPU的认知。"
date = 2025-02-06
draft = false
template = "blog/page.html"

[extra]
toc = true
+++

现在的高性能CPU基本上都是超标量 + SIMD 支持了，超标量的设计是提高指标并行度（IPC: Instruction Per Cycle）, SIMD 是提高数据并行度。二者
结合在很多高性能的场景下，可以发挥出很好的性能。本文注重与通过一些代码示例来度量 IPC，以及通过 IPC 来认知 CPU 的相关优化特性。

# 1. 如何度量 IPC?
在 x86 平台上，我们可以通过 perf 来度量 IPC（但大部分的云虚拟环境下，perf 都不支持 IPC的度量），但是在 M1 上，perf 并不支持 IPC 度量。

经过不懈的搜索，终于找到了一些 M1 上度量 IPC 的方法，参考：
- [M1: A demo shows how to read Intel or Apple M1 CPU performance counter in macOS.](https://gist.github.com/ibireme/173517c208c7dc333ba962c1f0d67d12) 
- [rust-kperf](https://github.com/El-Naizin/rust-kperf) MacOS 下的未公开的API 的逆向工程。
- [我改进的 M1 IPC 度量小工具](https://github.com/wangzaixiang/m1_ipc_measure) 在上述资料的基础上，我稍微改进的一个命令行工具，可以度量某个命令的 IPC 值。

本文中主要基于上述改进的 M1 IPC 度量小工具来度量 IPC。这个工具还很不完善，主要原因是 M1 的 kperf 库的API 资料还相当缺失，大部份都是通过逆向工程得到的，
我目前对这个 API 还是一知半解，目前这个小工具也只是勉强凑合，限制还很多，这些在该项目的 readme.md 中都有说明。

# 2. the m1 microarchitecture.
本文是在 M1 Max(Mac Book Pro 2021)上进行的， 下文中的 IPC 分析会参考这个 CPU 的特性。这里附上 M1 微架构图（来源：[中文](https://zhuanlan.zhihu.com/p/700865927)、
[原文](https://dougallj.github.io/applecpu/firestorm.html)）

{{ resize_image(path="@/blog/2025-02-06-measure-ipc-for-m1/M1-Firestorm-MicroArchitecture.png", width=1500, height=400, op="fit_width") }}

# 3. IPC度量

## 3.1 test1: IPC ～ 3.74, 为什么不是 6+ ?
代码：[完整源代码](https://github.com/wangzaixiang/onebrc_rust/master/src/bin/TestIPC.rs#L44)
```rust
fn test1(){
    let base0 = 1234i64;
    let base1 = 1234i64;
    let mut i1 = 10i64;
    let mut i2 = 20i64;
    let mut i3 = 30i64;
    let mut i4 = 40i64;
    let mut i5 = 50i64;
    let mut i6 = 60i64;
    let mut i7 = 70i64;
    let mut i8 = 80i64;
    let mut i9 = 90i64;
    let mut i10 = 100i64;
    let mut i11 = 110i64;
    let mut i12 = 120i64;

    let time0 = std::time::Instant::now();

    for _ in 0..100_0000_0000u64 {

        unsafe { asm! {
        "add {i1}, {i1}, {base0}",
        "add {i2}, {i2}, {base0}",
        "add {i3}, {i3}, {base0}",
        "add {i4}, {i4}, {base0}",
        "add {i5}, {i5}, {base0}",
        "add {i6}, {i6}, {base0}",
        "add {i7}, {i7}, {base0}",
        "add {i8}, {i8}, {base0}",
        "add {i9}, {i9}, {base0}",
        "add {i10}, {i10}, {base0}",
        "add {i11}, {i11}, {base0}",
        "add {i12}, {i12}, {base0}",
        "add {i1}, {i1}, {base1}",
        "add {i2}, {i2}, {base1}",
        "add {i3}, {i3}, {base1}",
        "add {i4}, {i4}, {base1}",
        "add {i5}, {i5}, {base1}",
        "add {i6}, {i6}, {base1}",
        "add {i7}, {i7}, {base1}",
        "add {i8}, {i8}, {base1}",
        "add {i9}, {i9}, {base1}",
        "add {i10}, {i10}, {base1}",
        "add {i11}, {i11}, {base1}",
        "add {i12}, {i12}, {base1}",
        base0 = in(reg) base0,
        base1 = in(reg) base1,
        i1 = inout(reg) i1,
        i2 = inout(reg) i2,
        i3 = inout(reg) i3,
        i4 = inout(reg) i4,
        i5 = inout(reg) i5,
        i6 = inout(reg) i6,
        i7 = inout(reg) i7,
        i8 = inout(reg) i8,
        i9 = inout(reg) i9,
        i10 = inout(reg) i10,
        i11 = inout(reg) i11,
        i12 = inout(reg) i12,
        }
        }

    }

    let time1 = time0.elapsed();
    println!("test1 total time: {:?}, iteration: {:.2}ns", time1, time1.as_nanos() as f64  / 100_0000_0000.0);
}
```

使用 inline asm 的方式，避免来自rust的优化，这样可以更为直接的观察 CPU 的执行情况。

这个示例循环100亿次，每次循环中执行 12 * 2 次的 add 指令，每12条 add 指令是完全无上下文依赖的，第2组对第一组有依赖，即每隔12条指令有上下文依赖。

度量结果：IPC: 3.74

按照架构图的设计，M1 有 6个 ALU 单元，都可以执行 add 指令， 理论上 IPC 值应该可以 达到 6，但只测试到了 3.74, 这个原因还有待分析。

## 3.2 test2: IPC ~1.84
```rust
fn test2(){
    let base0 = 1234i64;
    let base1 = 1234i64;
    let mut i1 = 10i64;
    let mut i2 = 20i64;
    let mut i3 = 30i64;
    let mut i4 = 40i64;
    let mut i5 = 50i64;
    let mut i6 = 60i64;
    let mut i7 = 70i64;
    let mut i8 = 80i64;
    let mut i9 = 90i64;
    let mut i10 = 100i64;
    let mut i11 = 110i64;
    let mut i12 = 120i64;

    let time0 = std::time::Instant::now();

    for _ in 0..100_0000_0000u64 {

        unsafe { asm! {
        "add {i1}, {i1}, {base0}",  // 1. loop N: instr 0 -> loop N - 1 : instr 12
        "add {i2}, {i2}, {i1}",     // 2
        "add {i3}, {i3}, {i2}",     // 3
        "add {i4}, {i4}, {i3}",     // 4
        "add {i5}, {i5}, {i4}",     // 5
        "add {i6}, {i6}, {i5}",     // 6
        "add {i7}, {i7}, {i6}",     // 7
        "add {i8}, {i8}, {i7}",     // 8
        "add {i9}, {i9}, {i8}",     // 9
        "add {i10}, {i10}, {i9}",   // 10
        "add {i11}, {i11}, {i10}",  // 11
        "add {i12}, {i12}, {i11}",  // 12
        "add {i1}, {i1}, {i12}",    // 13
        "add {i2}, {i2}, {i1}",     // 14
        "add {i3}, {i3}, {i2}",     // 15
        "add {i4}, {i4}, {i3}",     // 16
        "add {i5}, {i5}, {i4}",     // 17
        "add {i6}, {i6}, {i5}",     // 18
        "add {i7}, {i7}, {i6}",     // 19
        "add {i8}, {i8}, {i7}",     // 20
        "add {i9}, {i9}, {i8}",     // 21
        "add {i10}, {i10}, {i9}",   // 22
        "add {i11}, {i11}, {i10}",  // 23
        "add {i12}, {i12}, {i11}",  // 24
        base0 = in(reg) base0,
        // base1 = in(reg) base1,
        i1 = inout(reg) i1,
        i2 = inout(reg) i2,
        i3 = inout(reg) i3,
        i4 = inout(reg) i4,
        i5 = inout(reg) i5,
        i6 = inout(reg) i6,
        i7 = inout(reg) i7,
        i8 = inout(reg) i8,
        i9 = inout(reg) i9,
        i10 = inout(reg) i10,
        i11 = inout(reg) i11,
        i12 = inout(reg) i12,
        }
        }

    }

    let time1 = time0.elapsed();
    println!("test2 total time: {:?}, iteration: {:.2}ns", time1, time1.as_nanos() as f64  / 100_0000_0000.0);
}

```

在这段代码中，每轮循环中的24条指令是有上下文依赖的，即每条指令都依赖于上一条指令的结果，理论上只能串行执行，但很奇怪的是，IPC 仍然达到了 1.84，
而不是1，即每个周期内，CPU 仍然执行了1.84条指令。

仔细分析，虽然在每个循环内部，有上下文依赖，但下一个循环的第一条指令仅依赖于上一轮循环的第13条指令，所以，理论上 Loop1:14 和 Loop2:1 可以并行执行，
Loop1:15 和 Loop2:2 可以并行执行，Loop1:24 和 Loop2:11 可以并行执行，在大部份时间范围内，CPU 仍然可以并行2条指令。

那么为什么 Loop3 不能加入到并行执行中来呢？这样的话，IPC就可以接近3了？对照 M1 的架构图，我们可以猜测：由于受 ALU dispatch queue 的限制，
但具体的原因还有待进一步的分析。

在如此串行的代码中，IPC 仍然这么高，对简单的ALU操作，是否有可能限制到 1 呢？如果让 Loop2:head 依赖 Loop1:tail，理论上这样的代码就不太可能
并行执行了。我们可以尝试一下。

## 3.3 test2_1: IPC ~1.0
```rust
fn test2_1(){
    let base0 = 1234i64;
    let base1 = 1234i64;
    let mut i1 = 10i64;
    let mut i2 = 20i64;
    let mut i3 = 30i64;
    let mut i4 = 40i64;
    let mut i5 = 50i64;
    let mut i6 = 60i64;
    let mut i7 = 70i64;
    let mut i8 = 80i64;
    let mut i9 = 90i64;
    let mut i10 = 100i64;
    let mut i11 = 110i64;
    let mut i12 = 120i64;

    let time0 = std::time::Instant::now();

    for _ in 0..100_0000_0000u64 {

        unsafe { asm! {
        "add {i1}, {i12}, {base0}",
        "add {i2}, {i2}, {i1}",
        "add {i3}, {i3}, {i2}",
        "add {i4}, {i4}, {i3}",
        "add {i5}, {i5}, {i4}",
        "add {i6}, {i6}, {i5}",
        "add {i7}, {i7}, {i6}",
        "add {i8}, {i8}, {i7}",
        "add {i9}, {i9}, {i8}",
        "add {i10}, {i10}, {i9}",
        "add {i11}, {i11}, {i10}",
        "add {i12}, {i12}, {i11}",
        "add {i1}, {i1}, {i12}",
        "add {i2}, {i2}, {i1}",
        "add {i3}, {i3}, {i2}",
        "add {i4}, {i4}, {i3}",
        "add {i5}, {i5}, {i4}",
        "add {i6}, {i6}, {i5}",
        "add {i7}, {i7}, {i6}",
        "add {i8}, {i8}, {i7}",
        "add {i9}, {i9}, {i8}",
        "add {i10}, {i10}, {i9}",
        "add {i11}, {i11}, {i10}",
        "add {i12}, {i12}, {i11}",
        base0 = in(reg) base0,
        // base1 = in(reg) base1,
        i1 = inout(reg) i1,
        i2 = inout(reg) i2,
        i3 = inout(reg) i3,
        i4 = inout(reg) i4,
        i5 = inout(reg) i5,
        i6 = inout(reg) i6,
        i7 = inout(reg) i7,
        i8 = inout(reg) i8,
        i9 = inout(reg) i9,
        i10 = inout(reg) i10,
        i11 = inout(reg) i11,
        i12 = inout(reg) i12,
        }
        }

    }

    let time1 = time0.elapsed();
    println!("test2_1 total time: {:?}, iteration: {:.2}ns", time1, time1.as_nanos() as f64  / 100_0000_0000.0);
}
```

对这个完全首尾相衔的代码，IPC 约为：~ 1.08，至于为什么仍然大于1，应该是处理循环的额外指令，这些指令可以被并发执行，从而稍微的提高了 IPC。

## 3.4 如何使 IPC 达到最大值 8？

## 3.5 理解 register renaming

## 3.6 测试 load/store 指令