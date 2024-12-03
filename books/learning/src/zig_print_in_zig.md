# print in zig

示例来源于：[Case Study: print in zig](https://ziglang.org/documentation/0.13.0/#Case-Study-print-in-Zig)

```zig
const print = @import("std").debug.print;

const a_number: i32 = 1234;
const a_string = "foobar";

pub fn main() void {
    print("here is a string: '{s}' here is a number: {}\n", .{ a_string, a_number });
}

const Writer = struct {
    /// Calls print and then flushes the buffer.
    pub fn print(self: *Writer, comptime format: []const u8, args: anytype) anyerror!void {
        const State = enum {
            start,
            open_brace,
            close_brace,
        };

        comptime var start_index: usize = 0;
        comptime var state = State.start;
        comptime var next_arg: usize = 0;

        inline for (format, 0..) |c, i| {
            switch (state) {
                State.start => switch (c) {
                    '{' => {
                        if (start_index < i) try self.write(format[start_index..i]);
                        state = State.open_brace;
                    },
                    '}' => {
                        if (start_index < i) try self.write(format[start_index..i]);
                        state = State.close_brace;
                    },
                    else => {},
                },
                State.open_brace => switch (c) {
                    '{' => {
                        state = State.start;
                        start_index = i;
                    },
                    '}' => {
                        try self.printValue(args[next_arg]);
                        next_arg += 1;
                        state = State.start;
                        start_index = i + 1;
                    },
                    's' => {
                        continue;
                    },
                    else => @compileError("Unknown format character: " ++ [1]u8{c}),
                },
                State.close_brace => switch (c) {
                    '}' => {
                        state = State.start;
                        start_index = i;
                    },
                    else => @compileError("Single '}' encountered in format string"),
                },
            }
        }
        comptime {
            if (args.len != next_arg) {
                @compileError("Unused arguments");
            }
            if (state != State.start) {
                @compileError("Incomplete format string: " ++ format);
            }
        }
        if (start_index < format.len) {
            try self.write(format[start_index..format.len]);
        }
        try self.flush();
    }

    fn write(self: *Writer, value: []const u8) !void {
        _ = self;
        _ = value;
    }
    pub fn printValue(self: *Writer, value: anytype) !void {
        _ = self;
        _ = value;
    }
    fn flush(self: *Writer) !void {
        _ = self;
    }
};
```

理解上述的代码，有几个问题：

1. 如何理解函数调用 `print("here is a string: '{s}' here is a number: {}\n", .{ a_string, a_number });` 的执行过程？
   1. print 函数中包括了 comptime 的代码 和 inline 代码。
   2. comptime 代码会在编译期有确定的值，或者会在编译期执行。
   3. 其他的代码 会保留到运行期。
   4. inline 操作会结合了 comptime 与 运行期的代码，最终会输出一个替换后的 ast。（这个过程类似于 Scala3 的 inline ）
   
2. 包含 comptime 的方法，有些类似于 C++ template，会在 callsite 进行展开。在不被展开时，这个方法只要没有明显的语法错误，
   就可以编译通过，并不检查任何类型性的错误。（更类似于 C++ Template，不同于 Rust Generic ）
   和 comptime 最为相似的是 Scala3 的 Macro。

3. Zig 中的 Tuple 可以理解为字段名是匿名的 Struct，.{ } 既可以定义 struct，也可以定义 Tuple。
   Tuple 可以使用 `[index]` 方式访问内部元素。

4. zig 中的范型
   - anytype 范型
   - comptime 范型

type 是什么？ @TypeOf 的值在运行期就是一个字符串，描述了类型名。

# comptime

## comptime parameter

```zig
fn max(comptime T: type, a: T, b: T) T {
    return if (a > b) a else b;
}

// 这个方法是有错误的，但因为没有被调用，所以编译时并不报错
fn do_sth(comptime T: type): T {
   return T.MAX_VALUE; // 
}

// 这个方法是有错误的，但因为没有被调用，所以编译时并不报错
fn do_sth2() i16 {
   return i16.MAX_VALUE;
}

test "try to pass a runtime type" {
    foo(false);
}
fn foo(condition: bool) void {   
// fn foo(comptime condition: bool) void {   // change condition to comptime will fix the error
    const result = max(if (condition) f32 else u64, 1234, 5678);  // error: condition is not a comptime value
    _ = result;
}
```
1. `type` 是一个元类型，其值是一个类型，这个类型只能出现在编译期。zig 并没有提供 `type` 这个类型的内部结构（一般的，运行期所有的类型
都有自己的 layout 结构，但是 zig 语言中并没有定义 `type` 的 layout 结构），其内部结构是一个 opaque 的值，且仅能在 compile time
中存在。
2. comptime parameter 为 ziglang 提供了 generic 机制， 实际调用方法时，会为 comptime parameter 参数展开。
3. zig 的 generic 处理，更类似于 C++ 的 template，而非 Rust 的 generic。 参考上例，do_sth 中的 T.MAX_VALUE 是一个
   无效的访问，但是因为没有被调用，所以编译器并不会报错。
4. 不仅对 generic 的方法，对普通的方法，如果没有被调用，编译器也不会报错。

## comptime variable
1. 类似于 comptime parameter, comptime variable 也是一个编译期的值，不过，由于对 call site 透明，因此，并不会作为 generic 机制。
2. comptime parameter 也是一种类型的 comptime variable.
3. comptime variable 与 inline 结合时，可以实现混合：代码中一部分在编译期计算（展开、替换），一部分在运行期计算。（可以和 Scala3 inline 
   机制做一个对比，zig comptime + inline 比 scala3 inline 更简单，功能更强大，但功能完备性应该不如 Scala3 quotes API，后者可以在编译期
   直接操作 type 信息和 AST，理论上可以处理任何 blackbox 的功能，但 ziglang 具有 whitebox 的能力，又超出了 Scala3 Macro的边界）

> inline switch
> inline while
> inline for
> inline if
> inline fn

comptime variable

## comptime expression: 在编译期进行求值大的表达式



2. 

参考：https://zhuanlan.zhihu.com/p/622600857