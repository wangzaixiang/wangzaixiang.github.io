+++
title = "Mandelbrot-set CPU vs GPU comparison"
description = "基于mandelbrot set示例的 CPU vs GPU 版本性能对比 "
date = 2024-08-17T15:19:42+00:00
draft = false
template = "blog/page.html"

[taxonomies]
authors = ["wangzx"]
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

## 小结
1. 通过从零开始构建一个 GPU 实例，对 webgpu 的 API 和一些概念 有了更好的理解。算是入了个门了。
2. 目前基于 webgpu 的开发模式还是太繁琐了，相当于把现代语言的 link, loading 过程都要在代码中实现一次。类似于 Rust GPU、Bend 这样的技术
   未来一定会有巨大空间，让 GPU 开发的门槛与 CPU 开发对齐起来。
3. 类似于 dataframe 一类的向量计算，GPU 应该具有非常巨大的想象空间。可以和 SIMD 形成更好的搭配：SIMD 更轻便，但限制多，提升倍数有限。
   GPU 重（内存复制、CPU/GPU协同），支持复杂的分支，具有理论上更高的并发度。而目前，dataframe技术，即便在 SIMD 上，也优化有限，
   大部份的项目主要依赖语言级的 vectorize, 显示的 SIMD 算法并未大量使用，更谈不上 GPU 了，这一块，有非常广阔的发展空格。 

参考资料：
- [wgpu 教程](https://webgpufundamentals.org/webgpu/lessons/webgpu-fundamentals.html) 
- [Bend: 并行编程语言](https://github.com/HigherOrderCO/Bend) 可以直接在 GPU 上执行，基于 HVM2，类 python 的并行编程语言，
   不过目前仅支持 Cuda GPU。在我的 M1 Max 上无法体验。
- [Rust GPU](https://rust-gpu.github.io/blog/transition-announcement/) 直接将 Rust 编译到 GPU 上执行，
  相比基于 wgpu 进行开发，易用性有显著的差异（ webgpu 的开发模式还是很 low-level, 需要模拟 linker, loading 等复杂逻辑，
  优点回到上世纪使用 win32 API 编写 Windows 图形界面的感觉，我今天是花了3-4个小时，才让 mandelbrot 的GPU版本跑起来。 
  而 RustGPU/Bend 等则试图让 GPU 开发更接近于现代的编程。
- [faster Mandelbrot with SIMD](https://pythonspeed.com/articles/optimizing-with-simd/)
  这个算法如何利用 SIMD 进行优化，可以作为一个学习的例子。