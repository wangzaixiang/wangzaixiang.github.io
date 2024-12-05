+++
title = "Zig Comptime"
description = "本文记录我对 zig comptime 的一些理解、思考"
date = 2024-12-05
draft = false
template = "blog/page.html"

[extra]
toc = true
+++

# what is zig other than others?
1. zen: Only one obvious way to do things. 显示，直接。
   - No Hidden control flow. 没有隐藏的函数调用。（当然也没有析构函数、隐式转换、自动解引用等等）。缺点是抽象可能不够，优点是代码更直接。
   - No hidden allocations. 看一个函数是否有内存分配，只需要看它是否有 allocator 参数了
2. comptime
   - zig comptime vs rust macro
   - zig comptime vs Scala meta programming. 
   

# comptime expression 是如何执行的？

```zig
// main.zig
const std = @import("std");

// 这个方法没有太大的业务逻辑，目的仅仅是防止优化，让 longLoop(n) 方法的耗时更明显
fn longLoop(n: usize) usize {
    var sum: usize = 0;

    var i: usize = 0;
    while(i < n) : (i += 1) {
        var sum2: usize = 0;
        const skip = i % 10 + 1;
        var j: usize = 0;
        while(j < n) : (j += skip) {
            sum2 += j;
        }
        sum += sum2;
    }
    return sum;
}

pub fn main() !void {
    const n = 6000;

    // show times of longLoop(n)
    const start = std.time.milliTimestamp();
    const result = longLoop(n);
    const end = std.time.milliTimestamp();
    std.debug.print("runtime  eval: n = {}, result = {}, time = {}ms\n", .{ n,  result, end - start});

    const start2 = std.time.milliTimestamp();
    @setEvalBranchQuota(1_000_000_000);
    const result2 = comptime longLoop(n);
    const end2 = std.time.milliTimestamp();
    std.debug.print("comptime eval: n = {}, result = {}, time = {}ms\n", .{ n,  result2, end2 - start2});

}

```

1. 编译 main.zig, `zig build-exe -O ReleaseFast src/main.zig`, 耗时 21s. 调整 comptime longLoop(n) 的参数， 分别耗时如下：

   | n       | compile time | runtime eval | comptime eval |
   |---------|--------------|--------------|---------------|
   | 1       | 4.5s         | 0ms          | 0ms           |
   | 10      | 4.5s         | 0ms          | 0ms           |
   | 100     | 4.5s         | 0ms          | 0ms           |
   | 1000    | 5.0s         | 0ms          | 0ms           |
   | 2000    | 6.6s         | 1ms          | 0ms           |
   | 3000    | 8.6s         | 3ms          | 0ms           |
   | 4000    | 11.9s        | 3ms          | 0ms           |
   | 5000    | 15.9s        | 4ms          | 0ms           |
   | 6000    | 21.0s        | 9ms          | 0ms           |
   | 7000    | 27.0s        | 12ms         | 0ms           |
   | 8000    | 33.9s        | 14ms         | 0ms           |
   | 9000    | 41.9s        | 18ms         | 0ms           |
   | 10000   | 50.7s        | 19ms         | 0ms           |

   从上述数据可以看出，`comptime longLoop(n)` 随着 n 的增长， compile time 会显著增长，n == 1000 时，编译时长为5s，而 n = 10000 时
   编译时长为50s。而 runtime eval 的耗时仅仅是从 0ms 增长到 19ms, 这可以说明，compile 阶段，comptime eval 并非 native 方式执行 longLoop
   代码，而是采用了一种 AST interpreter 的方式执行代码，在这个场景中，效率有上千倍的差距。（这个案例仅为测试目的，实际 comptime 的耗时差距一般
   会显著低于这个差距，甚至在大部份情况下，对使用者无明显感知）。

2. comptime evaluation 是在 Sema 阶段完成的。参考文档：[Zig Sema](https://mitchellh.com/zig/sema)

   我还没有看懂这篇文章。

## 对比 Scala3 Macro

我对 Scala3 Macro 了解较多，wsql, wjson等项目都深度依赖 macro 提供的强大能力，rust macro 则只是泛泛了解。因此，很多时候，会对照
Scala3 macro 来理解 zig comptime. 
   
由于目前 zig comptime 执行的一些限制，例如：

1. 不能有 runtime side effects, 加上目前的 interpret 执行方式的可能性能损失，zig comptime 会有一些限制，而 scala3 macro 则不会受上述限制。
   > - All code with runtime side effects or depending on runtime values emits a compile error.
   > - All function calls cause the compiler to interpret the function at compile-time,
   >  emitting a compile error if the function tries to do something that has global runtime side effects.
2. Scala3 quotes API 有更强的反射能力，可以直接操作 AST，在一些动态生成代码的场景会更灵活（这个API本身是 Scala Compiler 的API）。
   zig 目前来看，是不提供对代码的 AST 反射、操纵的能力的。
3. 限制：Scala3 目前的 macro 提供的都是 blackbox macro, 也就是说，macro 自身并不影响外部的类型系统，例如，不能添加新的
   类型、方法、变量等，即使添加了，也无法为其他代码所感知。zig comptime 则可以直接创新新的类型。（这也是 zig generic 的实现方式）