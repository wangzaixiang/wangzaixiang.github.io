+++
title = "Mandelbrot-set CPU vs GPU comparison"
description = "基于mandelbrot set示例的 CPU vs GPU 版本性能对比 "
date = 2024-08-17T15:19:42+00:00
draft = false
template = "blog/page.html"
+++

本文承接上一片文章 [wgpu 代码阅读](@/blog/wgpu.md), 在阅读 wgpu 代码的基础上，通过一个实例来对比 CPU 和 GPU 的性能。

算法： Mandelbrot Set，其中 CPU 版本来自 《 Programming Rust 》第二版第2章（单线程）、第19章（多线程），
详细代码可以参考：[单线程版本](https://github.com/wangzaixiang/mandelbrot_gpu/blob/main/src/cpu.rs),
[多线程版本](https://github.com/wangzaixiang/mandelbrot_gpu/blob/main/src/cpu_par.rs)。

参考[wgpu-rs](https://wgpu.rs)的例子，我编写了一个GPU的版本，详细的代码请参考：[GPU版本](https://github.com/wangzaixiang/mandelbrot_gpu/blob/main/src/gpu.rs)，
对这三个版本分别进行对照性能测试：（测试环境：M1 Max, 64G）

| CPU(1 thread) | CPU(parallel) | GPU    |
|---------------|---------------|--------|
| 3.097s        | 0.368s        | 0.064s |
| 1x            | 8.4x          | 48.4x  |

从这个测试结果来看，使用 GPU 进行加速的效果是非常显著的，相比单线程有 48.4 倍的加速效果。相对于多线程的加速效果也有 6 倍的提升。

参考资料：
- [wgpu 教程](https://webgpufundamentals.org/webgpu/lessons/webgpu-fundamentals.html) 

> - 又一个神奇的工具：[Bend: 并行编程语言](https://github.com/HigherOrderCO/Bend) 可以直接在 GPU 上执行，基于 HVM2，类 python 的并行编程语言，不过目前仅
> 支持 Cuda GPU。在我的 M1 Max 上无法体验。
> - [Rust GPU](https://rust-gpu.github.io/blog/transition-announcement/) 直接将 Rust 编译到 GPU 上执行，相比基于 wgpu 进行开发，易用性有显著的差异（
> 在wgpu上的开发，就像回到使用 win32 API 编写 Windows 图形界面的感觉，我今天是花了3-4个小时，才让 mandelbrot 的GPU版本跑起来）。
> 看来，不久的将来，我们就可以直接使用 Rust 语言编写 GPU 程序了。