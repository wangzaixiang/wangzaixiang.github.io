+++
title = "Jun 2025"
description = "Jun 2025"
date = 2025-06-02
draft = false
template = "blog/page.html"
+++

# Languages
1. Rust
   - Async from scratch 系列教程
     - [1: What is in a Future, anyway?](https://natkr.com/2025-04-10-async-from-scratch-1/)
     - [2: Wake me maybe](https://natkr.com/2025-04-15-async-from-scratch-2/)
     - [3: Pinned against the wall ](https://natkr.com/2025-05-22-async-from-scratch-3/)
   - [Roto](https://github.com/NLnetLabs/roto) An Embedded script language that is fast, safe and easy to use.
     这个脚本语言在某种程度上很符合我的个人偏好，不过，内在的不支持 loop 的设计是否会限制语言的应用场景？作为一个 safe 的脚本语言，其边界必须是
     精心控制的，强类型适合于 compiled（基于[cranelift](https://cranelift.dev)项目），这有利于鲁棒性和性能。
   - SIMD in zlib-rs
     - [Autovectorization and target features](https://tweedegolf.nl/en/blog/153/simd-in-zlib-rs-part-1-autovectorization-and-target-features)
     - [compare256](https://tweedegolf.nl/en/blog/155/simd-in-zlib-rs-part-2-compare256)
   - [Rust memory latency and performance](https://developerlife.com/2025/05/19/rust-mem-latency/)
     - [New CPUs don't speed up old code](https://www.youtube.com/watch?v=m7PVZixO35c&feature=youtu.be)
     - [Data oriented design](https://youtu.be/WwkuAqObplU): Flat dara structure are better for memory locality
     - [Memory Latency vs CPU operation](https://youtu.be/Dhn-JgZaBWo)
     - [Memory Allocation Tips](https://youtu.be/pJ-FRRB5E84&t=1831)
       - [smallvec](https://docs.rs/smallvec/latest/smallvec/struct.SmallVec.html) 类似于 Arrow 的 [Variable-size Binary View Layout](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-view-layout)
       - [smallstr](https://docs.rs/smallstr/0.3.0/smallstr/)
     - ![memory_latency.svg](memory_latency.svg)
     - Stack vs heap 以及广义的类 String/Vec 数据结构中的一级存储、二级存储。
     - rust drop 开销比 alloc 要大: 是否是因为 drop 自身的开销而非 dealloc？
     - memory alignment 对性能的影响
     - Global Allocators
       - jemalloc: 多线程、大量小对象(chunk设计)
       - default/ptmalloc:  单线程
       - tcmalloc: 三级 ThreadCache + CentralCache + PageHeap， 自旋锁, 小对象无锁分配，瞬态高并发
     - RingBuffer：使用数组模拟链表，更好的内存局部性
2. SIMD
   - [simd-everywhere](https://github.com/simd-everywhere/simde) 软件仿真的方式模拟 SIMD 指令。ARM 的 SIMD 指令参考资料很难阅读，可以通过
     这个站点，来理解这些指令是如何仿真执行的。不过，好像有不少指令并没有实现。
   - [a plan for simd](https://linebender.org/blog/a-plan-for-simd/) `Linebender`的 SIMD 实现计划。
     - 使用 256 bit width: 能很好的匹配 AVX2, 在 NEON 上使用2个寄存器进行模拟。考虑到 Neon 有32个128位寄存器，仍然有足够的处理能力（在M1系列芯片上，基本等效于16 x 256 的AVX2）。
     - 文中提到 AVX512 相比 AVX256，性能提升有限，`a 512 bit vector is processed in two clock cycles (see mersenneforum post for more details), each handling 256 bits`, 
       这种说法，但没有提到确切的信息来源。
     - 在 [Milvis](https://mp.weixin.qq.com/s?__biz=MzUzMDI5OTA5NQ==&mid=2247487111&idx=1&sn=6d0577675df2d7649c78434d7c3aa3df&chksm=fb8cd799a5e3c39b87e6073f8b27607da43498205097d82bb4ef383ccadeb708fb48f098d0f0#rd)
       中有 AVX2 vs AVX512 的对比，有提升 20%-65%。
     - AVX512 可能会导致 CPU 降频，最终性能提升打了折扣。

# MPP & OLAP

# Web & Frontend

# Tools & Libraries

