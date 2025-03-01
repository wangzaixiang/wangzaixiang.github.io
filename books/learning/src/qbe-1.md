# 初识 QBE

[QBE](https://c9x.me/compile/docs.html) 是一个教学性质的编译器后端实现，可以理解为一个简单版的 LLVM。与LLVM相似，他也是基于 SSA IR 的，
但仅有大约 12000 行 C 代码，就实现了 LLVM 70% 的功能。如果从学习编译器后端的角度来看，QBE 是一个非常好的选择。

- 核心代码：在 src 目录下，6.7k 行（如果剔除 IR parser，则 ~5.3K 行代码）
- 3个架构：amd86(~2.3k), ARM64(~1.9k), RISC-V64(~1.5k) 共 5.7K 行
- 1 个 mini-c 的前端编译器，大约 2.7k loc。 这个工具可以将 mini-c 代码编译成 QBE IR，在学习的过程中，可以尝试使用这种方式来编写 IR。

所以，如果真正阅读源代码的话，核心代码大约 6-8k, 而且是 pure c 代码，非常适合学习。

## 理解 SSA IR
QBE 文档页面有参考资料，包括：
1. [QBE SSA IR](https://c9x.me/compile/doc/il.html) 要学习 QBE，现需要熟悉它的 SSA IR 定义。

   相比于 LLVM IR, QBE 的 IR 要简化很多，不过麻雀虽小，概念俱全，对于理解 SSA 尤其是 phi 函数来说，其实是足够了的。

2. [The Static Single Assignment Book](https://pfalcon.github.io/ssabook/latest/book-full.pdf) SSA 教程。

我的理解：
- SSA 更适合于数据流的静态分析，因此，为优化提供了更多的可能性。
- 大部份的 backend 都是对 SSA 的优化，SSA IR 为 backend 提供了一个统一的IR。虽然众多的 phase 构成了复杂而庞大的 backend，但其核心依然相对简洁。
- 一个快速的学习方式是，现抛开各种优化，一个最简单的 backend 会是什么样子？这里最简单的 backend 甚至比 QBE 还要简单，只生成可以正确运行的机器代码
  即可，甚至连寄存器分配都可以忽略，这样的 backend 会包括什么？
- 然后在这个最简化的模型上，逐步叠加各种优化，理解每一种优化的原理和实现。

按照这个逻辑来学习 backend，可能会是一个不错的思路。不必在一开始就陷入到复杂的概念、数据结构、算法细节中（compiler backend 无疑是目前最为复杂的
软件体系，要比操作系统、数据库、浏览器等要复杂、精密得多）。

## 从 mini-c 开始
在还不是足够熟悉 QBE IR 的情况下，如果要手写 IR 的话，还是会遇到一定的困难，这个时候，mini-c 就可以发挥用场了。

```c
# include <stdio.h>

fib(int n) {
  if(n==0) return 1;
  if(n==1) return 1;
  return fib(n-1) + fib(n-2);
}

main(){
    int result;
    result = fib(10);
    printf("result = %d\n", result);
}
```

1. 使用 mini-c 编译器，将 mini-c 代码编译成 QBE IR。 `cat demo.c | minic > demo.ir`
2. 使用 QBE 编译器，将 IR 编译成汇编代码。 `qbe -o demo.s demo.ir`
3. 使用 clang 编译汇编代码。 `clang -o demo demo.s`
4. 运行 demo。 `./demo`

阅读一下 demo.ir 的代码，如下：

```text
export function w $fib(w %t0) {
@l0
	%n =l alloc4 4
	storew %t0, %n
	%t2 =w loadw %n
	%t1 =w ceqw %t2, 0
	jnz %t1, @l1, @l2
@l1
	ret 1
@l2
	%t6 =w loadw %n
	%t5 =w ceqw %t6, 1
	jnz %t5, @l4, @l5
@l4
	ret 1
@l5
	%t12 =w loadw %n
	%t11 =w sub %t12, 1
	%t10 =w call $fib(w %t11, ...)
	%t16 =w loadw %n
	%t15 =w sub %t16, 2
	%t14 =w call $fib(w %t15, ...)
	%t9 =w add %t10, %t14
	ret %t9
}

export function w $main() {
@l7
	%result =l alloc4 4
	%t1 =w call $fib(w 10, ...)
	storew %t1, %result
	%t5 =w loadw %result
	%t3 =w call $printf(l $glo1, w %t5, ...)    # minic 生成的 IR 有些问题，应该是 call $printf(l $fmt, ..., w %t5)
	ret 0
}

data $glo1 = { b "result = %d\n", b 0 }
```

可以看到，这段 IR 代码并不是很高效，有很大的优化空间，比如，我们可以手动优化为：

```text
export function w $fib(w %t0) {
@l0
	%t1 =w ceqw %t0, 0
	jnz %t1, @l1, @l2
@l1
	ret 1
@l2
	%t5 =w ceqw %t0, 1
	jnz %t5, @l4, @l5
@l4
	ret 1
@l5
	%t11 =w sub %t0, 1
	%t10 =w call $fib(w %t11, ...)
	%t15 =w sub %t0, 2
	%t14 =w call $fib(w %t15, ...)
	%t9 =w add %t10, %t14
	ret %t9
}

export function w $main() {                # Main function
@start
        %r =w call $fib(w 10)
        call $printf(l $fmt, ..., w %r)    # Show the result
        ret 0
}
data $fmt = { b "result = %d!\n", b 0 }
```

这个 IR 显然更为简洁，更为高效。

不过，对两个版本的 IR 采用 qbe 编译后，却可以发现，两个版本的汇编代码是一样的，这是因为 QBE 会自动进行优化，将 IR 优化为更高效的代码。