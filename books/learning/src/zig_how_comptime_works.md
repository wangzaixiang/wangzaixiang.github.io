# comptime expression 的执行机制

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