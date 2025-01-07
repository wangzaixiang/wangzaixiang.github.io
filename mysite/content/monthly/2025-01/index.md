+++
title = "January 2025"
description = "2025-01 monthly digest"
date = 2025-01-01
draft = false
template = "blog/page.html"
+++

# Languages
1. [Performance optimization — branchless programming](https://medium.com/@techhara/performance-optimization-technique-branchless-programming-a40c0a35511e)
   ```cpp
   int branchful(std::optional<int> x, std::optional<int> y) {
    const int NULL_VALUE = -1;

    if (x.has_value() && y.has_value()) {
        return *x * *y;
    } else {
        return NULL_VALUE;
    }
   }
   ```
   消除分支：
   ```cpp
   boolean hasX = x.has_value();
   boolean hasY = y.has_value();
   boolean all = hasX && hasY;
   return all ? *x * *y : NULL_VALUE;
   ```
   6-7倍性能提升。 在 CPU 层，cmov 指令可以实现分支消除，
   
   工具： perf stat -e instructions,branches,branch-misses  command args
   ![img.png](img.png)
2. [Performance optimization—efficient cache programming 1](https://medium.com/@techhara/performance-optimization-efficient-cache-programming-f107dce3bef0)
   1. 减少 struct 的大小，对大数组类的数据结构，可以减少内存的占用，从而提高 cache 的命中率。
3. [How to write code to make the cpu execute faster](https://blog.devgenius.io/cpu-cache-how-to-write-code-to-make-the-cpu-execute-faster-cc0cf4969c4b)
   - Linux: `/sys/devices/system/cpu/cpu0/cache/index[0123]/size` for Level L1(IC) L1(DC), 2, 3 cache size.
   - `/sys/devices/system/cpu/cpu0/cache/index[0123]/coherency_line_size` for cache line size.
   - sched_setaffinity: set the CPU affinity of a process.
   - `array[i][j]` 访问顺序对 cache 的影响。
4. [Zig Comptime is Bonkers Good 疯狂的好](https://www.scottredig.com/blog/bonkers_comptime/)
   - view 0: 语法噪音少，你几乎可以忽略它。
   - view 1: 没有泛型，但有更好的泛型。
   - view 2: 编译期执行的标准 zig 代码。
   - view 3/4/5: 混合 comptime + runtime，展开成新的代码。

# MPP & OLAP

# Web & Visualization

# Tools & Libraries