# dynamic construct a type in comptime

Zig 可以通过 comptime 来实现 generic，但官网给的例子还是比较简单的：

```zig
fn List(comptime T: type) type {
    return struct {
        items: []T,
        len: usize,
    };
}

// The generic List data structure can be instantiated by passing in a type:
var buffer: [10]i32 = undefined;
var list = List(i32){
    .items = &buffer,
    .len = 0,
};
```

这个例子中，构造的 `List(i32)` 还是感觉不够动态，譬如，是否可以：
- 结构体的成员数量、类型是动态的？
- 结构体内的 fn 是动态的？

这一切的奥秘，隐藏在 `@typeInfo`, `@Type` 这几个内置函数中。如下是一个简单的示例： User2 是一个 comptime 动态计算出来的类型，其一部份
字段是从 User 这个模版类型中复制来的，email 字段则是动态添加上去的。

```zig

const std = @import("std");

// used as a type Template
const User = struct {
    name: [:0]const u8,
    age: u32,
};

pub fn main() void {

    const print = std.debug.print;
    const t_info: std.builtin.Type = @typeInfo(User);

    // dynamic construct a Type
    const t_info2: std.builtin.Type = .{
       .Struct = .{
          .layout = t_info.Struct.layout,
           .backing_integer =  t_info.Struct.backing_integer,
           .fields = & .{
               .{
                   .name = "NAME",
                   .type = t_info.Struct.fields[0].type,
                   .default_value = t_info.Struct.fields[0].default_value,
                   .is_comptime = t_info.Struct.fields[0].is_comptime,
                   .alignment = t_info.Struct.fields[0].alignment
               },
               .{
                   .name = "AGE",
                   .type = t_info.Struct.fields[1].type,
                   .default_value = t_info.Struct.fields[1].default_value,
                   .is_comptime = t_info.Struct.fields[1].is_comptime,
                   .alignment = t_info.Struct.fields[1].alignment
               },
               .{
                   .name = "email",
                   .type = [:0]const u8,
                   .default_value = null,
                   .is_comptime = false,
                   .alignment = 1
               }
           },
           .decls = t_info.Struct.decls,
           .is_tuple = false
       }
    };

    // build type User2
    const User2 = @Type(t_info2);

    // now, User2 can be used in source code.
    const u: User2 = .{
        .NAME = "WANGZX",
        .AGE = 20,
        .email = "wangzx@qq.com",
    };

    print("Users = {any}\n", .{User2});
    print("u.NAME = {s}, u.AGE = {d} u.email = {s}\n",
        .{ u.NAME, u.AGE, u.email });
}

```

## 结论
1. std.builtin.Type 类似于 scala.quotes.TypeRepr，是 comptime 时用于描述 Type 的元数据结构。
2. 目前来看，并没有提供动态构建一个 Fn ，即操作 AST 的 API。因此，动态构建的类型，还是有一些局限的。