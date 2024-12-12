# 在 Zig 中进行 Structure of Array 的一个小实验

# 1. What is AOS and SOA?

本案例参考： https://mitchellh.com/zig/parser 文中的 MultiArrayList 的介绍。

对如下的 struct 示例：
```zig
pub const Tree = struct {
    age: u32,       // trees can be very old, hence 32-bits
    alive: bool,    // is this tree still alive?
};
```

如果我们需要存储一个`[10]Tree`，采用 Array of Structure, 那么内存布局是这样的：
```
┌──────────────┬──────────────┬──────────────┬──────────────┐
│     Tree     │     Tree     │     Tree     │     ...      │
└──────────────┴──────────────┴──────────────┴──────────────┘
```
每个 Tree 占用8个字节，其内存布局是这样的：
```
age: u32, 4 bytes
alive: bool, 1 byte
padding: 3 bytes
```

那么，这个布局就会存在较为严重的内存浪费，如果 Struct 结构体没有重排（rust/zig 默认会对结构体进行重排），则可能会存在更多的 padding 内存占用，
而如果采用 Structure of Array, 那么内存布局是这样的：
```
          ┌──────┬──────┬──────┬──────┐
   age:   │ age  │ age  │ age  │ ...  │
          └──────┴──────┴──────┴──────┘
          ┌┬┬┬┬┐
 alive:   ││││││
          └┴┴┴┴┘
```
这样，我们可以看到，age 和 alive 分别存储在不同的数组中，这样，就可以减少 padding 的内存占用。

此外，如果结构题字段很多，例如 AST Node, 在编译期的给定迭代会遍历大量的 Node，但一般会处理有限的字段时，Structure of Array 也会带来 Cache 的
友好性，因为这个字段的内存是连续存放的，只有需要使用到的字段才会加载到 Cache 中,而如果是 Array of Structure, 则会加载整个结构体到缓存中，
缓存的有效利用率就会降低。或许，这方面对性能的提升会比 padding 节约的内存更有价值。

# 2. How to effective implement SOA in Zig?
本文并不实现一个完整的 SOA 数据结构，而是探索在 Zig 中如何简单、高效的实现一个 SOA 数据结构，以及是否能够到到足够的高性能。

参考如下的代码示例
```zig 
const Node = struct {
    a: u32,
    b: u8
};

// 下一步使用 comptime 生成一个 SOA 的数据结构
// fn SOA(structure: type, N: comptime_int) type {
// 
// }

// 这个示例使用手写版本的 SOA 
fn NodeSOA(N: comptime_int) type {
    const result = struct {
        a: [N]u32,
        b: [N]u8,

        fn init() NodeSOA(N) {
            return NodeSOA(N) {
                .a = undefined,
                .b = undefined,
            };
        }

        fn get(self: *NodeSOA(N), index: u32) Node {
            return Node{ .a = self.a[index], .b = self.b[index] };
        }

        fn set(self: *NodeSOA(N), index: u32, node: Node) void {
            self.a[index] = node.a;
            self.b[index] = node.b;
        }
    };

    return result;
}
```

## 2.1 using comptime to generate SOA
从目前对 Zig 的了解来看，应该是可以使用 comptime 来自动生成这个 SOA 结构的。这个可以留作下一步的学习 zig 的挑战。

## 2.2 是否高效
上述的实现中，我很好奇的是，如果我们需要访问 ` soa[index].x ` 这样的操作，是否是高效的。由于 zig 不支持运算符重载，因此，语法为：
`soa.get(index).x`,
1. soa.get(index) 会有一个构造 Node 的操作，如果字段较多，会涉及到很多的字段赋值，返回值作为值的传递也会有很大的开销。
2. 我们实际上仅用到 x 字段，其他的字段其实是没有被使用到了。

带着对这个问题的好奇，我做一个简单的测试：

```zig 
pub fn main() !void {

    var nodes = NodeSOA(10).init();

    // get argv[1] and convert it to u32
    // const arg = std.os.args.arg(1);
    var args = std.process.args();
    defer args.deinit();
    _ = args.skip();
    const arg1 = args.next();
    const index: u32 = if(arg1) |x| try std.fmt.parseInt(u32, x, 10)
        else 0;

    nodes.set(0, Node{ .a = 1, .b = 2 });
    nodes.set(1, Node{ .a = 3, .b = 4 });
    nodes.set(2, Node{ .a = 5, .b = 6 });
    nodes.set(3, Node{ .a = 7, .b = 8 });
    nodes.set(4, Node{ .a = 9, .b = 10 });
    nodes.set(5, Node{ .a = 11, .b = 12 });
    nodes.set(6, Node{ .a = 13, .b = 14 });
    nodes.set(7, Node{ .a = 15, .b = 16 });
    nodes.set(8, Node{ .a = 17, .b = 18 });
    nodes.set(9, Node{ .a = 19, .b = 20 });

    var x: u32 = 123;
    print("x = {}\n", .{x});

    // 重点关注这几段代码生成的asm代码：
    x += nodes.get(index).a;
    x += nodes.get(index).b;
    print("x = {}\n", .{x});

    x += nodes.get(index+1).a;
    x += nodes.get(index+1).b;
    print("x = {}\n", .{x});
}
```

在 ReleaseSmall/ReleaseFast 模式下：
```
	lea	rdi, [rsp + 48]
	mov	dword ptr [rdi], 123
	call	_debug.print__anon_1219

	mov	r14d, r14d
	mov	eax, dword ptr [rsp + 4*r14 + 56]   // nodes.get(index).a
	movzx	ecx, byte ptr [rsp + r14 + 96]  // nodes.get(index).b
	lea	ebp, [rax + rcx]
	add	ebp, 123
	lea	rdi, [rsp + 52]
	mov	dword ptr [rdi], ebp
	call	_debug.print__anon_1219

	add	ebp, dword ptr [rsp + 4*r14 + 60]  // nodes.get(index+1).a
	movzx	eax, byte ptr [rsp + r14 + 97] // nodes.get(index+1).b
	add	eax, ebp
	lea	rdi, [rsp + 24]
	mov	dword ptr [rdi], eax
	call	_debug.print__anon_1219
```

可以看到，nodes.get(index).a 这样的操作已经被优化成了与手写代码一样的效率，这些应该是 LLVM IR 优化带来的巨大价值。

当然，在 Debug 模式下，是不会进行这个优化的。

# 3. 总结
1. 利用 Zig 的 comptime 特性，可以生成一个 SOA 的数据结构（TODO）
2. zig 对 这种 SOA 的数据结构的访问，由于编译优化，实际上是高效的，完全无需担心额外的性能开销。（Zero Cost Abstraction）
3. Rust 采用 macro 应该也能实现类似的方式。相比之下，comptime 应该更简单一些。毕竟 rust macro 本质上又是另外一门语言了。
4. comptime 生成的类型，在调试器中是有很清晰的结构。不过，IDE 对这类的支持还不够完善。相比 rust, zig 的调试看起来要清爽很多。

由于优化器的能力提升，很多的 Zero Cost Abstraction 的实现，实际上已经从 compiler 的 front end 转移到一个 backend 的优化器上了。