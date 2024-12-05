+++
title = "一个 Zig 编译器的 Bug"
description = "一个 Zig 编译器的 Bug"
date = 2024-12-05
draft = false
template = "blog/page.html"

[extra]
toc = true
+++
# 一个 Zig 编译器的 Bug

在动手学习 Zig 的过程中，在探索stack中变量的memory layout时，发现了了一个 Bug，已提交到 [github](https://github.com/ziglang/zig/issues/22140)。 
在这里记录一下：

```zig
const std = @import("std");

const SIZE = 1024 * 256;
pub fn main() !void {
    var arr: [SIZE]u32 = undefined;
    for (arr, 0..) |_, i| {
        arr[i] = @intCast(i);
    }
    std.debug.print("main &arr = {*}\n", .{&arr});
    std.debug.print("main &arr = {*} same as above \n", .{&arr}); // same as above

    std.debug.print("\npassArray for mutable array\n", .{});
    passArray(arr);
}

fn passArray(arr: [SIZE]u32) void {
    const p1: *const [SIZE]u32 = &arr;
    const p2 = &arr;
    const p3: [*]const u32 = &arr;
    std.debug.print("inside passArray &arr = {*} p2 = {*} p3 = {*}  p1 != p2 != p3 \n", .{ p1, p2, p3 });

    const LOOP = 3; // when LOOP = 14, the program will crash

    std.debug.print("LOOP = {} \n", .{LOOP});
    inline for (0..LOOP) |_| {
        std.debug.print("inside passArray, &arr = {*} not same as above.\n", .{&arr}); // &arr increase SIZE * 4 every time
    }
} 
```

{{ resize_image(path="/learning/images/a_zig_bug.png", width=1000, height=400, op="fit_width") }}

因为每次 `&arr` 操作都导致在栈上复制了一个数组，因此，如果数组长度较大，`&arr` 操作次数较多时，例如，在上述的代码中，1M * 14 = 14M， 在我
的 Mac 上，就会出现 SIGSEV 错误 ( 应该是 StackOverflow 了 )。

在国内的 Zig 群问了一下，众说纷纭，有大神坚持认为这个不是 bug，而是 constcast 的必然结果，不过我并不能理解：
1. `&arr` 只是一个取地址操作，并不会改变数据类型。如果原来是 const 的，结果就是 `*const [N]` 否则就是 `*[N]`
2. 不同于 rust, zig 并没有 `&x` 和 `&mut x` 的区别。
3. `&arr` 导致数组复制，不仅会导致栈内存的浪费，而且也增加了不必要的代码成本。更严重的会导致 StackOverflow，其实还是一个比较严重的问题。

提交到 github 上，很快获得了 core team 的确认，已接受作为一个Bug，并添加到了 0.14 的 milestone 中。