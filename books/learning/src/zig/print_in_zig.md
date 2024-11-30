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
3. Zig 中的 Tuple 可以理解为字段名是匿名的 Struct，.{ } 既可以定义 struct，也可以定义 Tuple。
   Tuple 可以使用 `[index]` 方式访问内部元素。

type 是什么？ @TypeOf 的值在运行期就是一个字符串，描述了类型名。