# 栈上内存分配

本文通过一些代码示例，来了解 zig 中函数调用栈上内存分配的情况。并对比与其他语言的差异。

```zig
const std = @import("std");

pub fn main() !void {
    var x1: i32 = 10; // i32
    const str1 = "hello"; // str1 is a pointer to static memory(text section)
    const str2: [5:0]u8 = .{ 'h', 'e', 'l', 'l', 'o' }; // str2 is a pointer to static memory(text section)
    var str3: [8:0]u8 = .{ 'h', 'e', 'l', 'l', 'o', 'w', 'o', 'r' }; // alloc in stack

    var x2: i32 = 20; // i32

    std.debug.print("&x1 = {*}, &x2 = {*}\n", .{ &x1, &x2 });
    std.debug.print("str1 = {*}, &str1 = {*}\n", .{ str1, &str1 });

    std.debug.print("&str2 = {*}\n", .{&str2});
    std.debug.print("&str3 = {*}\n", .{&str3});
}

```

输出：
```
&x1 = i32@16d8d31dc
str1 = [5:0]u8@1025b19f0, &str1 = *const [5:0]u8@1025cc5c8
&str2 = [5:0]u8@1025b19f0
&str3 = [8:0]u8@16d8d31e0
&x2 = i32@16d8d31ec
```

结论：
1. str1 类型为 [5:0]u8 ，是一个数组， 但在堆栈中存储的是一个这个值的指针。数据在 static memory 中。
2. str2 类型为 [5:0]u8 ，是一个数组， 但在堆栈中存储的是一个这个值的指针。数据在 static memory 中。
3. str3 类型为 [8:0]u8 ，是一个数组， 这个值在 stack 中分配， str3 是这个数组的初始地址。
4. x1 的 下一个地址是 str3 ，然后是 x2，可以看到 str1, str2 这些 const 变量都存储在 static memory 中，未占用栈空间。